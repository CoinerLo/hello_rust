use crate::Serialize;

#[derive(Debug, Serialize)]
pub struct Chat {
    pub id: i32,
    pub name: String,
    pub creator: String,
}
