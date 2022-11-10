use mongodb::Client;


pub struct Diff {
    tgt_client : Client,
    src_client : Client,
    meta_client : Client,
    block_size : int16,
    filters: HashMap<String, Filter>,
}

pub struct Filter {
    filter : mongodb::bson::D,
    namespace : String,
    to : String
}


async fn new_diff(config : Config) -> Diff{
   let tgtClient  = diffly::client("").await?;
   let srcClient = diffly::client("").await?;
    
    Diff{
        tgtClient : tgtClient,
        srcClient: srcClient,
        metaClient: tgtClient,
    }
}




