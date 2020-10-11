use serenity::framework::standard::CommandGroup;
use serenity::{
    framework::standard::{help_commands, macros::help, Args, CommandResult, HelpOptions},
    model::{channel::Message, id::UserId},
    prelude::*,
};
use std::collections::HashSet;

#[help]
#[individual_command_tip = "Hello!\n\
If you want more information about a specific command, just pass the command as argument."]
#[command_not_found_text = "Could not find: `{}`."]
#[max_levenshtein_distance(3)]
#[indention_prefix = "+"]
#[lacking_permissions = "Hide"]
#[lacking_role = "Nothing"]
#[wrong_channel = "Strike"]
pub async fn my_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}
