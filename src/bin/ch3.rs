use color_eyre::Result;
use edgedb_tokio::create_client;

use crate::models::{Country, Vampire};

mod models;

#[tokio::main]
async fn main() -> Result<()> {
    let client = create_client().await?;

    println!("Inserting countries...");
    let query = r#"
        insert Country {
          name := 'Hungary',
          modern_name := ''
        };
        
        insert Country {
          name := 'Romania',
          modern_name := ''
        };
    "#;
    client.execute(query, &()).await?;

    println!("Checking countries...");
    let query = r#"
        select Country {
            name,
            modern_name,
            important_places
        };
    "#;

    let countries: Vec<Country> = client.query(query, &()).await?;
    println!("{countries:#?}");

    println!("Inserting Dracula...");
    let query = r#"
        insert Vampire {
          name := 'Count Dracula',
          age := 800,
          places_visited := (select Place filter .name = 'Romania'),
        };
    "#;
    client.execute(query, &()).await?;

    let query = r#"
        select Vampire {
          name,
          age,
          places_visited: {
            id,
            modern_name,
            name,
            important_places
          }
        };
    "#;
    let vampire: Vec<Vampire> = client.query(query, &()).await?;
    println!("{vampire:#?}");

    Ok(())
}
