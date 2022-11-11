use job::JobCreator;
use tokio;

pub mod storage;
mod job;
mod worker;


#[tokio::main]
async fn main(){
    let result = storage::create_client("".to_string()).await;

    let m_client = match result {
        Ok(client) => client,
        Err(error) => panic!("error creating mongo client: {:?}", error),
    };

    let jb = JobCreator{
        client: m_client,
    };

    let job_result = jb.create_jobs().await;

    match job_result {
        Ok(res) => println!("job done {}", res),
        Err(error) => panic!("error creating jobs: {:?}", error),
    }
}



