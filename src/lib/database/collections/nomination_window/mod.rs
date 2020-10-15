use crate::utils::insert_operation;
use crate::utils::update_operation;
use mongodb::bson::doc;
use mongodb::{error::Error, Database};

pub async fn start_window(db: &Database) -> Result<bool, Error> {
    let collection = db.collection("nominations");
    insert_operation(&collection, || doc! {"active": true}, doc! {"active": true}).await
}

pub async fn end_window(db: &Database) -> Result<bool, Error> {
    let collection = db.collection("nominations");
    let update_value = doc! {
       "active": false,
    };
    let get_filter = || doc! {"active": true};
    update_operation(&collection, get_filter, update_value).await
}
