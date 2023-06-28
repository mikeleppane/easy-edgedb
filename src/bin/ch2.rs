use crate::models::Person;
use color_eyre::Result;
use edgedb_tokio::create_client;

mod models;

#[tokio::main]
async fn main() -> Result<()> {
    let client = create_client().await?;

    println!("Inserting Bistritz...");
    let query = r#"
        insert City {
          name := 'Bistritz',
          modern_name := 'Bistrița',
          important_places := ['Golden Krone Hotel'],
        };
    "#;
    client.execute(query, &()).await?;

    println!("Inserting Person...");
    let query = r#"
        insert PC {
          name := 'Emil Sinclair',
          places_visited := City,
          class := 'Mystic',
        };
    "#;
    client.execute(query, &()).await?;

    println!("Selecting persons...");
    let query = r#"
        select Person {
          name,
          places_visited: {
            id,
            modern_name,
            name,
            important_places
          }
        };
    "#;

    let persons: Vec<Person> = client.query(query, &()).await?;
    println!("{persons:#?}");

    let res: i64 = client
        .query_required_single("select <int64>'9' + 9;", &())
        .await?;
    assert_eq!(res, 18);

    println!("Select Emil...");
    let query = r#"
        select Person {
          name,
          places_visited: {
            id,
            modern_name,
            name,
            important_places
          }
        } filter .name = 'Emil Sinclair';
    "#;

    let emil: Vec<Person> = client.query(query, &()).await?;
    println!("{emil:#?}");

    Ok(())
}
