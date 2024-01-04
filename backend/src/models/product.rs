use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct Product {
    id: Option<Uuid>,
    name: String,
    color: String,
    category: String,
    price: i32,
}
