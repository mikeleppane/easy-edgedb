use edgedb_derive::Queryable;

#[derive(Debug, Queryable)]
pub struct Country {
    pub name: String,
    pub modern_name: String,
    pub important_places: Vec<String>,
}
