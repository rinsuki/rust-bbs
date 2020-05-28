use super::schema::boards;

#[derive(Queryable)]
pub struct Board {
    pub id: String,
    pub name: String,
}
