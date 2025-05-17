#[derive(Debug, serde::Deserialize)]
pub struct Chat {
    pub id: i32,
    pub name: String,
    pub creator: String,
}
