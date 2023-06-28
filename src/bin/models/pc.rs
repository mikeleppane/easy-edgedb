use crate::models::City;
use edgedb_derive::Queryable;

#[derive(Debug, Queryable)]
pub struct Person {
    pub name: String,
    pub places_visited: Vec<City>,
}
