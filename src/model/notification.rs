use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize,Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Notfication {
    pub product_title: String,
    pub product_type: String,
    pub product_url: String,
    pub subcriber_name: String,
    pub status: String,
}