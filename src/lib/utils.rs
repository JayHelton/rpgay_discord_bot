use crate::DbContext;
use serenity::model::channel::Message;
use serenity::prelude::*;

pub async fn get_database_from_ctx(ctx: &Context) -> mongodb::Database {
    let data = ctx.data.read().await;
    match data.get::<DbContext>() {
        Some(db_context) => match db_context.get("db") {
            Some(db) => db.to_owned(),
            None => panic!("Proble getting db context"),
        },
        None => panic!("Problem getting db context"),
    }
}

pub async fn send_discord_message(msg: &Message, ctx: &Context, content: &str) {
    if let Err(why) = msg.channel_id.say(&ctx.http, content).await {
        println!("Error sending message: {:?}", why);
    }
}

pub async fn update_operation<F: Fn() -> mongodb::bson::Document>(
    collection: &mongodb::Collection,
    filter_fn: F,
    update_value: mongodb::bson::Document,
) -> Result<bool, mongodb::error::Error> {
    let cursor = collection.find_one(filter_fn(), None).await?;
    if let Some(_document) = cursor {
        let updated_fn = collection.update_one(filter_fn(), update_value, None);

        match updated_fn.await {
            Ok(_result) => Ok(true),
            Err(_why) => Ok(false),
        }
    } else {
        Ok(false)
    }
}

pub async fn insert_operation<F: Fn() -> mongodb::bson::Document>(
    collection: &mongodb::Collection,
    filter_fn: F,
    insert_value: mongodb::bson::Document,
) -> Result<bool, mongodb::error::Error> {
    let cursor = collection.find_one(filter_fn(), None).await?;
    match cursor {
        Some(_document) => Ok(false),
        None => {
            let insert_result = collection.insert_one(insert_value, None).await;
            match insert_result {
                Ok(_res) => Ok(true),
                Err(_why) => Ok(false),
            }
        }
    }
}
