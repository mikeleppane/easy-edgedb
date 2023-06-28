use edgedb_derive::Queryable;
use edgedb_protocol::model::Uuid;

#[derive(Debug, Queryable)]
pub struct City {
    pub id: Uuid,
    pub modern_name: String,
    pub name: String,
    pub important_places: Vec<String>,
}
