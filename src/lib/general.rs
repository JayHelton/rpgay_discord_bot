use crate::structs::{CommandCounter, ShardManagerContainer};

use serenity::prelude::*;
use serenity::{
    client::bridge::gateway::ShardId,
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::channel::Message,
};
use std::fmt::Write;

#[group]
#[commands(about, commands, latency)]
pub struct General;

#[command]
#[bucket = "complicated"]
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
#[allowed_roles("mods")]
async fn test_role(_ctx: &Context, _msg: &Message, _args: Args) -> CommandResult {
    println!("we made it into this command");
    Ok(())
}

#[command]
async fn about(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(why) = msg
        .channel_id
        .say(&ctx.http, "This is a small test-bot! : )")
        .await
    {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}

#[command]
async fn latency(ctx: &Context, msg: &Message) -> CommandResult {
    // The shard manager is an interface for mutating, stopping, restarting, and
    // retrieving information about shards.
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

    // Shards are backed by a "shard runner" responsible for processing events
    // over the shard, so we'll get the information about the shard runner for
    // the shard this command was sent over.
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
