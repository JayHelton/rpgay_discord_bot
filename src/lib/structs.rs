use serenity::{
    async_trait, client::bridge::gateway::ShardManager, model::gateway::Ready, prelude::*,
};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

pub struct CommandCounter;

impl TypeMapKey for CommandCounter {
    type Value = HashMap<String, u64>;
}

pub struct DbContext;

impl TypeMapKey for DbContext {
    type Value = HashMap<String, mongodb::Database>;
}

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
