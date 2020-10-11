pub mod lib;
use crate::general::GENERAL_GROUP;
use crate::help::MY_HELP;
use crate::hooks::*;
use crate::nomination::NOMINATION_GROUP;
use crate::services::application::get_application_data;
use crate::structs::*;

use lib::*;

use serenity::{
    framework::standard::{DispatchError, StandardFramework},
    model::channel::Message,
};
use std::{collections::HashMap, env, sync::Arc};

use serenity::prelude::*;
use serenity::{futures::future::BoxFuture, FutureExt};

fn _dispatch_error_no_macro<'fut>(
    ctx: &'fut mut Context,
    msg: &'fut Message,
    error: DispatchError,
) -> BoxFuture<'fut, ()> {
    async move {
        if let DispatchError::Ratelimited(duration) = error {
            let _ = msg
                .channel_id
                .say(
                    &ctx.http,
                    &format!("Try this again in {} seconds.", duration.as_secs()),
                )
                .await;
        };
    }
    .boxed()
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let (owners, bot_id) = get_application_data(&token).await;

    let framework = StandardFramework::new()
        .configure(|c| {
            c.with_whitespace(true)
                .on_mention(Some(bot_id))
                .prefix("~")
                .delimiters(vec![", ", ","])
                .owners(owners)
        })
        .before(before)
        .after(after)
        .unrecognised_command(unknown_command)
        .normal_message(normal_message)
        .on_dispatch_error(dispatch_error)
        .help(&MY_HELP)
        .group(&GENERAL_GROUP)
        .group(&NOMINATION_GROUP);

    let mut client = Client::new(&token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<CommandCounter>(HashMap::default());
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
    }

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
