use crate::database::collections::nominations::add_nomination;
use crate::utils::{get_database_from_ctx, send_discord_message};
use serenity::prelude::*;
use serenity::{
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::channel::Message,
};

use crate::database::collections::nomination_window::{end_window, start_window};

#[group]
#[commands(nomination, nominate)]
pub struct Nomination;

#[command]
#[allowed_roles("Lord Ruler")]
async fn nomination(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let db = get_database_from_ctx(ctx).await;
    let first_arg: String = args.parse().unwrap();
    let content: &str;

    match first_arg.as_str() {
        "start" => {
            let started = start_window(&db).await?;

            content = if started {
                "Nomination Started!"
            } else {
                "Nomination failed to start! Has one already been started?"
            };

            send_discord_message(msg, ctx, content).await;
        }
        "end" => {
            let ended = end_window(&db).await?;

            content = if ended {
                "Nomination ended!"
            } else {
                "Nomination not found! Has one been started yet?"
            };

            send_discord_message(msg, ctx, content).await;
        }
        _ => {}
    }
    Ok(())
}

#[command]
#[allowed_roles("patron")]
async fn nominate(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let db = get_database_from_ctx(ctx).await;
    let first_arg: String = args.parse().unwrap();

    let _nominated = add_nomination(&db, first_arg.clone()).await?;
    send_discord_message(
        msg,
        ctx,
        format!("You've nominated {}!", first_arg.clone()).as_str(),
    )
    .await;
    Ok(())
}
