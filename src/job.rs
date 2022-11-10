use mongodb::{Client, bson::Document};
use futures::{stream, StreamExt};


// creats task for verification
pub struct JobCreator {
   pub client : Client
}

pub struct Job {
    id :  String,
    filter : String,
    doc_ids : String,
    status : String,
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

    async fn create_sub_jobs(&self, namespaces : &Vec<String>) {
        let concurrency = 8;
        stream::iter(namespaces)
            .for_each_concurrent(concurrency, |job| async move {
                let v : Vec<&str> = job.split(".").collect();
                self.split_collection(v[0].to_string(), v[1].to_string()).await;
            })
            .await; 
    
    }
    async fn split_collection(&self, db : String, coll : String) {
        
        println!("processing the namespace {} . {}", db, coll);
        
        let collection = self.client.database(db.as_str()).collection(coll.as_str());
        let data : Result<mongodb::Cursor<Document>, ()>= collection.find(None, None)
        .await
        .map_err(|e| println!("{}", e));
       // data is a Result<mongodb::Cursor> type
        match data {
            Ok(mut cursor) => {
                // I want to iterate the returned documents here
                // this doesn't compiles
                while let Some(doc) = cursor.next().await {
                    let raw_doc = match doc {
                        Ok(rd) => rd,
                        Err(err) => panic!("error creating mongo client: {:?}", err),
                    };

                    //println!("{}",raw_doc.get("_id").unwrap());
                    
                }
            },
            Err(e) => println!("{:?}", e),
        }
    }
}





