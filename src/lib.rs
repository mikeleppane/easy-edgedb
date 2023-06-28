use color_eyre::Result;
use edgedb_tokio::{Client, Error};

pub async fn create_client() -> Result<Client, Error> {
    edgedb_tokio::create_client().await
}
