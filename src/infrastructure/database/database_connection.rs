use std::env;
use dotenv::dotenv;
use tokio_postgres::{Client, NoTls};

pub struct DatabaseConnection {
    pub client: Client
}

impl DatabaseConnection {

    pub async fn new() -> Result<Self, tokio_postgres::Error> {
        dotenv().ok();
        
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
        let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprint!("Connection error: {}", e);
            }
        });

        Ok(Self { client })
    }
}