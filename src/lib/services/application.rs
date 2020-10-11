use serenity::http::Http;
use std::collections::HashSet;

pub async fn get_application_data(
    token: &str,
) -> (
    HashSet<serenity::model::id::UserId>,
    serenity::model::id::UserId,
) {
    let http = Http::new_with_token(token);

    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            if let Some(team) = info.team {
                owners.insert(team.owner_user_id);
            } else {
                owners.insert(info.owner.id);
            }
            match http.get_current_user().await {
                Ok(bot_id) => (owners, bot_id.id),
                Err(why) => panic!("Could not access the bot id: {:?}", why),
            }
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };
    return (owners, bot_id);
}
