use crate::database::collections::nominations::{add_nomination, get_nomination_list};
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
#[commands(nomination, nominate, list)]
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

    let nominated = add_nomination(&db, first_arg.clone(), msg.author.name.clone()).await?;
    let content: &str = if nominated {
        "Nomination Successful!"
    } else {
        "Nominationed failed. Please try again after complaining."
    };
    send_discord_message(msg, ctx, content).await;
    Ok(())
}

#[command]
#[allowed_roles("patron")]
async fn list(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let db = get_database_from_ctx(ctx).await;
    let nominations = get_nomination_list(&db).await?;
    let sent = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.content("Hello, World!");
            m.embed(|e| {
                e.title("Nomination List");
                // e.fields(nominations.iter().map(|x| {
                //     let doc = x.as_document();
                //     match doc {
                //         Some(document) => match document.get_str("name") {
                //             Ok(name) => (name, "", true),
                //             Err(_why) => ("", "", true),
                //         },
                //         None => ("", "", true),
                //     }
                // }));
                e.footer(|f| {
                    f.text("This list is immutable.");
                    f
                });

                e
            });
            m
        })
        .await;

    if let Err(why) = sent {
        println!("Error sending message: {:?}", why);
    }
    Ok(())
}
