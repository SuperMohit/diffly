use mongodb::{Client, bson::{Document, Bson}};
use futures::{stream, StreamExt};
use serde::{Serialize, Deserialize};



// creats task for verification
pub struct JobCreator {
   pub client : Client
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Status {
    New,
    Processing,
    Verified,
    Retry
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Job {
    id :  Option<mongodb::bson::oid::ObjectId>,
    pub filter : QFilter,
    pub doc_ids : Vec<Bson>,
    pub status : Status
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QFilter {
    pub db : String,
    pub coll: String,
    pub query: Option<mongodb::bson::Document>,
    pub to_db: Option<String>,
    pub to_coll: Option<String>,
}


impl JobCreator {
    // breaks each collection into smaller tasks 
    // each task is responsible for verifying range of documents
    pub async fn create_jobs(&self) -> Result<String, mongodb::error::Error> {
        // create tasks based on each namespace
        let mut namespaces: Vec<String> = Vec::new();
        // collects all the namespaces form source database
        for db in self.client.list_databases(None, None).await? {
          for coll  in  self.client.database(&db.name).list_collection_names(None).await?{
            // run splitter with following parallelism
            if namespaces.len() >= 8 {
                self.create_sub_jobs(&namespaces).await; 
                namespaces.clear();
            }
            let db_name = &db.name ;
            let ns = format!("{}.{}", db_name.to_string(),coll);    
            namespaces.push(ns.to_string());
          }
        }
       Ok("yes".to_string())
    }

    // processes the namespace in parallel depending on concurrency set
    // todo: read concurrency from config  
    async fn create_sub_jobs(&self, namespaces : &Vec<String>) {
        let concurrency = 8;
        stream::iter(namespaces)
            .for_each_concurrent(concurrency, |job| async move {
                let v : Vec<&str> = job.split(".").collect();
                self.split_collection(v[0].to_string(), v[1].to_string()).await;
            })
            .await; 
    
    }

    // opens a cursor to individual collections
    // creates a task for workers so they can verify the document
    // batch size is default to 1000 but should be configured based on
    // the docs size and VMs resources 
    async fn split_collection(&self, db : String, coll : String) {
        println!("processing the namespace {} . {}", db, coll);
        
        let collection = self.client.database(db.as_str()).collection(coll.as_str());
        let task_coll  = self.client.database("meta").collection("task");
        
        let data : Result<mongodb::Cursor<Document>, ()>= collection.find(None, None)
        .await
        .map_err(|e| println!("{}", e));
       // data is a Result<mongodb::Cursor> type
        match data {
            Ok(mut cursor) => {                
                
                let mut docs : Vec<Bson> = Vec::new();
                let mut count : i64 = 0;
                let f = QFilter{ db : db.to_string(), coll: coll.to_string(), 
                    query: None, to_coll: Some(coll.to_string()), to_db: Some(db.to_string())}; 
                
                while let Some(doc) = cursor.next().await {
                    if count % 1000 == 0 && !docs.is_empty(){
                        
                        let j = Job{
                            filter: f.clone(),
                            doc_ids: docs.clone(),
                            status: Status::New,
                            id: None,
                        };
                        // let it panic so that no doc range is missed.
                        // TODO: retry
                        let _r  = task_coll.insert_one(j, None).await.unwrap();
                        docs.clear();
                    }                    
                    let raw_doc = match doc {
                        Ok(rd) => rd,
                        Err(err) => panic!("error creating mongo client: {:?}", err),
                    };
                    count+=1;                    
                    let id = raw_doc.get("_id").unwrap();
                    docs.push(id.to_owned());                    
                }
                
                // check for vector incase docs length is > 0
                if !docs.is_empty() {
                    let j = Job{
                        filter: f,
                        doc_ids: docs.clone(),
                        status: Status::New,
                        id: None,
                    };
                    let r = task_coll.insert_one(j, None).await;
                    match r {
                        Ok(res) => print!("added to task {}", res.inserted_id),
                        Err(err) => print!("error occurred {}", err),
                    }
                }
            },
            Err(e) => println!("{:?}", e),
        }
    }
}





