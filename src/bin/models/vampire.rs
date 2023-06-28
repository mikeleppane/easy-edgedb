use crate::models::City;
use edgedb_derive::Queryable;

#[derive(Debug, Queryable)]
pub struct Vampire {
    pub name: String,
    pub age: i16,
    pub places_visited: Vec<City>,
}
