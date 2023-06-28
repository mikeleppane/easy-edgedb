use crate::models::City;
use edgedb_derive::Queryable;
use edgedb_protocol::model::Uuid;

#[derive(Debug, Queryable)]
pub struct Npc {
    pub id: Uuid,
    pub name: String,
    pub places_visited: Vec<City>,
}
