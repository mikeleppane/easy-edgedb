mod models;

use crate::models::{City, Npc};
use color_eyre::Result;
use edgedb_protocol::value::Value;
use edgedb_tokio::create_client;

#[tokio::main]
async fn main() -> Result<()> {
    let client = create_client().await?;
    let query = r#"
        select City {
          id,
          modern_name,
          name
        }
    "#;
    let cities: Vec<City> = client.query(query, &()).await?;
    if cities.is_empty() {
        let query: Value = client
            .query_required_single(
                "\
            insert City {
                name := 'Munich',
                modern_name := ''
            };
            insert City {
              name := 'Buda-Pesth',
              modern_name := 'Budapest'
            };
        
            insert City {
              name := 'Bistritz',
              modern_name := 'Bistrița'
            };",
                &(),
            )
            .await?;
        print!("New cities inserted:\n{query:#?}");
    }

    let query = r#"
        select City {
          id,
          modern_name,
          name
        }
    "#;
    let cities: Vec<City> = client.query(query, &()).await?;
    println!("{cities:#?}");

    let query = r#"
        insert NPC {
          name := "Jonathan Harker",
          places_visited := City
        }
    "#;
    let insert_npc: Value = client.query_required_single(query, &()).await?;
    println!("{insert_npc:#?}");

    let query = r#"
        select NPC {
          id,
          name,
          places_visited: {
            id,
            modern_name,
            name,
          }
        };
    "#;
    let npc: Vec<Npc> = client.query(query, &()).await?;
    println!("{npc:#?}");

    let res: bool = client
        .query_required_single("select 8 is int64;", &())
        .await?;
    assert!(res);

    Ok(())
}
