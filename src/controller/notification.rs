use rocket::response::status::Created;
use rocket::serde::json::json;

use bambangshop::Result;
use crate::model::subscriber::Subscriber;
use crate::service::notification::NotifcationService;