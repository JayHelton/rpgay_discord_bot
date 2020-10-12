use mongodb::bson::doc;
use mongodb::{error::Error, Database};

pub async fn add_nomination(db: &Database, nomination: String) -> Result<bool, Error> {
    let collection = db.collection("nominations");
    let filter = doc! { "active": true };
    let cursor = collection.find_one(filter, None).await?;
    match cursor {
        Some(document) => {
            let _updated = collection.update_one(
                doc! {
                    "_id": document.get("_id").expect("No ID")
                },
                doc! {
                    "$push": {
                        "nominations": doc! {
                            "name": nomination
                        }
                    }
                },
                None,
            );
            Ok(true)
        }
        None => Ok(false),
    }
}
