use serenity::prelude::*;
use serenity::{
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::channel::Message,
};

#[group]
#[commands(nomination, nominate)]
pub struct Nomination;

#[command]
#[allowed_roles("Lord Ruler")]
async fn nomination(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let first_arg: String = args.parse().unwrap();
    match first_arg.as_str() {
        "start" => {
            if let Err(why) = msg
                .channel_id
                .say(&ctx.http, "You've Started the Nomination Period!")
                .await
            {
                println!("Error sending message: {:?}", why);
            }
        }
        "end" => {
            if let Err(why) = msg
                .channel_id
                .say(&ctx.http, "You've Ended the Nomination Period!")
                .await
            {
                println!("Error sending message: {:?}", why);
            }
        }
        _ => {}
    }
    Ok(())
}

#[command]
#[allowed_roles("patron")]
async fn nominate(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let first_arg: String = args.parse().unwrap();
    if let Err(why) = msg
        .channel_id
        .say(&ctx.http, format!("You've nominated {}!", first_arg))
        .await
    {
        println!("Error sending message: {:?}", why);
    }
    Ok(())
}

#[command]
async fn list(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if let Err(why) = msg
        .channel_id
        .say(
            &ctx.http,
            "Ill list all the games nominated for this round.",
        )
        .await
    {
        println!("Error sending message: {:?}", why);
    }
    Ok(())
}
