use rocket::response::status::Created;
use rocket::serde::json::json;

use bambangshop::Result;
use crate::model::subscriber::Subscriber;
use crate::service::notification::NotifcationService;

#[post("/subscribe/<product_type>", data = "<subscriber")]
pub fn subscribe(product_type: &str, susbcriber: Json<Subscriber>) -> Result<Created<Json<Subscriber>>> {
    return match NotifcationService::subscribe(product_type, subscriber.into_inner()) {
        Ok(f) => Ok(Created::new("/").body(Json::from(f))),
        Err(e) => Err(e)
    };
}