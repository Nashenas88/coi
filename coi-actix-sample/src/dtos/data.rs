use crate::models::data::Data;
use serde::Serialize;

#[derive(Serialize)]
pub struct DataDto {
    id: i64,
    name: String,
}

impl From<Data> for DataDto {
    fn from(data: Data) -> Self {
        Self {
            id: data.id,
            name: data.name,
        }
    }
}