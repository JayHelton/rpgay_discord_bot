use crate::utils::update_operation;
use mongodb::bson::doc;
use mongodb::{error::Error, Database};

pub async fn add_nomination(
    db: &Database,
    nomination: String,
    username: String,
) -> Result<bool, Error> {
    let collection = db.collection("nominations");
    let update_value = doc! {
        "$push": {
            "nominations": doc! {
                "name": nomination,
                "nominated_by": username
            },
        }
    };
    let get_filter = || doc! {"active": true};
    update_operation(&collection, get_filter, update_value).await
}

pub async fn add_nomination_vote(
    db: &Database,
    nomination_vote: String,
    username: String,
) -> Result<bool, Error> {
    let collection = db.collection("nominations");
    let update_value = doc! {
        "$push": {
            "nominations.$.votes": username,
        }
    };
    let get_filter = || doc! {"active": true, "$elemMatch": doc! {"name": nomination_vote.clone()}};
    update_operation(&collection, get_filter, update_value).await
}

pub async fn get_nomination_list(db: &Database) -> Result<Vec<mongodb::bson::Bson>, Error> {
    let collection = db.collection("nominations");
    let cursor = collection.find_one(doc! {"active": true}, None).await?;
    if let Some(document) = cursor {
        let nominations = document.get("nominations");

        match nominations {
            Some(noms) => {
                match noms.as_array() {
                    Some(nom_doc) => Ok(nom_doc.clone()),
                    None => Ok(vec![]),
                }
            }
            None => Ok(vec![]),
        }
    } else {
        Ok(vec![])
    }
}
