use mongodb::bson::doc;
use mongodb::{error::Error, Database};

pub async fn start_window(db: &Database) -> Result<bool, Error> {
    let collection = db.collection("nominations");
    let filter = doc! { "active": true };
    let cursor = collection.find_one(filter, None).await?;
    match cursor {
        Some(_document) => Ok(false),
        None => {
            let new_nomination = doc! {
               "active": true,
            };
            let _insert_result = collection.insert_one(new_nomination.clone(), None).await?;
            Ok(true)
        }
    }
}

pub async fn end_window(db: &Database) -> Result<bool, Error> {
    let collection = db.collection("nominations");
    let filter = doc! { "active": true };
    let cursor = collection.find_one(filter, None).await?;
    match cursor {
        Some(document) => {
            let _updated = collection
                .update_one(
                    doc! {
                        "_id": document.get("_id").expect("No Id Found")
                    },
                    doc! {
                     "$set":  {"active": false }
                    },
                    None,
                )
                .await?;
            Ok(true)
        }
        None => Ok(false),
    }
}
