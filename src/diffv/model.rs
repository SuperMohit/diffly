use std::collections::HashMap;

use mongodb::Client;


pub struct Diff {
    pub tgt_client : Client,
    pub src_client : Client,
    pub meta_client : Client,
    pub block_size : i16,
    pub filters: Option<HashMap<String, Filter>>,
}

pub struct Filter {
    filter : mongodb::bson::Document,
    namespace : String,
    to : String
}


// pub async fn new_diff(config : Config) -> Diff{
//    let tgt_client  = diffly::client("").await?;
//    let src_client = diffly::client("").await?;
    
//     Diff{
//         tgt_client : tgt_client,
//         src_client: src_client,
//         meta_client: tgt_client,
//     }
// }




