use crate::structs::{CommandCounter, ShardManagerContainer};

use serenity::prelude::*;
use serenity::{
    client::bridge::gateway::ShardId,
    framework::standard::{
        macros::{command, group},
        CommandResult,
    },
    model::channel::Message,
};
use std::fmt::Write;

#[group]
#[commands(about, commands, latency)]
pub struct General;

#[command]
async fn commands(ctx: &Context, msg: &Message) -> CommandResult {
    let mut contents = "Commands used:\n".to_string();

    let data = ctx.data.read().await;
    let counter = data
        .get::<CommandCounter>()
        .expect("Expected CommandCounter in TypeMap.");

    for (k, v) in counter {
        let _ = write!(contents, "- {name}: {amount}\n", name = k, amount = v);
    }

    if let Err(why) = msg.channel_id.say(&ctx.http, &contents).await {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}

#[command]
async fn about(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(why) = msg
        .channel_id
        .say(&ctx.http, "I'm Rudy, the Nomination Bot : )")
        .await
    {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}

#[command]
async fn latency(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;

    let shard_manager = match data.get::<ShardManagerContainer>() {
        Some(v) => v,
        None => {
            let _ = msg
                .reply(ctx, "There was a problem getting the shard manager")
                .await;

            return Ok(());
        }
    };

    let manager = shard_manager.lock().await;
    let runners = manager.runners.lock().await;

    let runner = match runners.get(&ShardId(ctx.shard_id)) {
        Some(runner) => runner,
        None => {
            let _ = msg.reply(ctx, "No shard found");

            return Ok(());
        }
    };

    let _ = msg
        .reply(ctx, &format!("The shard latency is {:?}", runner.latency))
        .await;

    Ok(())
}
