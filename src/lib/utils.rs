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
