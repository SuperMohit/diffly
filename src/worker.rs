use futures::{stream, StreamExt};




// pub struct Worker {
//     d :Diff
// }

// impl Worker {
//     async fn compute_job(&self, job: Job) -> i64 {
//         let src_db : string = "";
//         let tgt_db : string = "";    

//         let src_coll = self.d.src_client.database("sample_mflix").collection("movies");
//         let tgt_coll = self.d.tgt_client.database("sample_mflix").collection("movies");
        
//         let src_docs : mongodb::bson::Raw = src_coll.find(
//             doc! {
//                 "title": "Parasite"
//             },
//             None,
//             ).await?.expect("Missing 'Parasite' document.");

//         let tgt_docs : mongodb::bson::Raw = src_coll.find(
//             doc! {
//                 "title": "Parasite"
//             },
//             None,
//             ).await?.expect("Missing 'Parasite' document.");
//         // compare the doc    

//       }
// }








// async fn process_result(result: i64) {
//     println!("{}", result);
//     // result is failed // update the status and retry
//     // if it is passed // do nothing
// }

// #[tokio::main]
// async fn task() {
    
//     loop {
//         // fetch 8 tasks at a time max and process
//         // break when main tasks is finished
//         // if no tasks are available, try few more times, and then exit  
//         let jobs = 0..100;
//         let concurrency = 42;

//         stream::iter(jobs)
//             .for_each_concurrent(concurrency, |job| async move {
//                 let result = compute_job(job).await;
//                 process_result(result).await;
//             })
//             .await; 
        
            
//     }
    
    
// }

// async fn compareDocs(srcDocs : HashMap<>, tgtDocs : HashMap<>) {
//         // compare the length of the maps
//         // iterate over source and check if it is available in tgt
          
// }