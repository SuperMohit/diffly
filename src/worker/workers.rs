use std::collections::HashMap;

use futures::{stream, StreamExt};
use mongodb::{options::FindOptions, bson::{Document, Bson}, Collection};
use crate::{job::{Job, Status}, diffv::model::Diff};



pub struct Worker {
    pub diff : Diff
}


// pub struct Report<T> {
//     pub doc_id : T,
//     pub details : String,
//     pub cluster : String,
//     pub namespace: String,
// }

impl Worker {
    async fn compute_job<'a>(&'a self, job: &'a mut Job) -> &Job {
        let src_db_name = job.filter.db.clone();
        let tgt_db_name = match &job.filter.to_db {
            Some(db) => db,
            None => &job.filter.db,
        };

        let src_coll_name = job.filter.coll.clone();        
        let tgt_coll_name = match &job.filter.to_coll {
            Some(coll) => coll,
            None =>& job.filter.coll,
        };

        let src_collection = self.diff.src_client.database(&src_db_name).collection::<Document>(&src_coll_name);
        let tgt_collection = self.diff.src_client.database(&tgt_db_name).collection::<Document>(&tgt_coll_name);
        let src_docs = self.fetch_docs(src_collection).await;
        let tgt_docs = self.fetch_docs(tgt_collection).await;
        
        let report = self.compare_docs(&src_docs, &tgt_docs);
        
        if report.len() > 0 {
            job.doc_ids = report;
            job.status = Status::Retry;
        } 
        job

      }

    

    async fn fetch_docs(&self, coll : Collection<Document>) -> HashMap<String, Document> {
        let mut docs : HashMap<String, Document> = HashMap::new();
        let data   = coll.find(None, None).
        await
        .map_err(|e| println!("{}", e));
        match data {
            Ok(mut cursor) => { 
                while let Some(doc) = cursor.next().await {
                    let raw_doc = match doc {
                        Ok(rd) => rd,
                        Err(err) => panic!("error in cursor : {:?}", err),
                    };
                    let id = raw_doc.get("_id").unwrap();                             
                    docs.insert(id.to_string(), raw_doc);
                }       
            },
            Err(e) => println!("{:?}", e),
        }
        docs
    }   


    pub async fn process_result(&self, job: &Job) {
    let tgt_db_name = match &job.filter.to_db {
        Some(db) => db,
        None => &job.filter.db,
    };

    let task_coll = self.diff.src_client.database(&tgt_db_name).collection::<Job>("v_task");
    
    let r = task_coll.insert_one(job, None).await;
    match r {
        Ok(res) => print!("added to task {}", res.inserted_id),
        Err(err) => print!("error occurred {}", err),
    }
}


    pub async fn task(&self) {
        let task_coll  = self.diff.meta_client.database("meta").collection::<Job>("task");
        let options = FindOptions::builder()
                  .limit(8)
                  .build();

        let mut iter = 1;              
        loop {
            println!("{}", iter);
            let mut jobs: Vec<Job> = Vec::new();
            let data   = task_coll.find(None, options.clone()).
            await
            .map_err(|e| println!("{}", e));
            match data {
                Ok(mut cursor) => { 
                    while let Some(doc) = cursor.next().await {
                        let raw_doc = match doc {
                            Ok(rd) => rd,
                            Err(err) => panic!("error creating mongo client: {:?}", err),
                        };
                        jobs.push(raw_doc);
                    }
            },
            Err(_) => break,        
        }
        stream::iter(jobs)
            .for_each_concurrent(8, |mut job| async move {
                let result = self.compute_job(&mut job).await;                
                self.process_result(result).await;
            }).await;
            iter = iter + 1;
    }
}


 fn compare_docs<'a>(&'a self, src_docs : &'a HashMap<String, Document>, 
            tgt_docs : &'a HashMap<String, Document>) -> Vec<Bson> {
    let mut failed_docs : Vec<Bson>= Vec::new(); 
    for (id, src_doc) in src_docs {
        match tgt_docs.get(id) {
            Some(tgt_doc) => {
                let id = tgt_doc.get("_id").unwrap();
                if src_doc == tgt_doc {
                    // TODO compare each field
                    continue;
                } else {
                    failed_docs.push(id.to_owned());
                }
            },
            None => {
                let id = src_doc.get("_id").unwrap();
                failed_docs.push(id.to_owned());
            },
        }
      }
      failed_docs
}

}
