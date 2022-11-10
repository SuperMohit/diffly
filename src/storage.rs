

use mongodb::{Client, options::ClientOptions};

pub(crate) async fn Create_Client(uri :String) -> Result<Client, mongodb::error::Error>{

    // Parse a connection string into an options struct.
    let mut client_options = ClientOptions::parse(uri).await?;

    // Manually set an option.
    client_options.app_name = Some("diffly".to_string());

    // Get a handle to the deployment.
    let client = Client::with_options(client_options)?;
    
    Ok(client)
    
} 



