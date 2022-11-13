use job::JobCreator;
use tokio;

pub mod storage;
mod job;
mod worker;

mod diffv;

use diffv::model::Diff;
use worker::workers::Worker;


#[tokio::main]
async fn main(){
    let src_client = storage::create_client("".to_string()).await;
    let tgt_client = storage::create_client("".to_string()).await;

    let s_client = match src_client {
        Ok(client) => client,
        Err(error) => panic!("error creating mongo client: {:?}", error),
    };

    let t_client = match tgt_client {
        Ok(client) => client,
        Err(error) => panic!("error creating mongo client: {:?}", error),
    };

    let jb = JobCreator{
        client: s_client.clone(),
    };

    let job_result = jb.create_jobs().await;

    match job_result {
        Ok(res) => println!("job done {}", res),
        Err(error) => panic!("error creating jobs: {:?}", error),
    }

    let d = Diff {
        tgt_client: t_client.clone(),
        src_client: s_client,
        meta_client: t_client,
        block_size: 250,
        filters: None,
    }; 
    
    let w = Worker{
        diff: d
    };

    w.task();

}



