use crate::commands::command_handler::CommandHandler;

use std::sync::Arc;

use serenity::async_trait;
use serenity::client::Context;
use serenity::prelude::{TypeMap, RwLock};
use serenity::model::channel::Message;

pub struct PingHandler;

#[async_trait]
impl CommandHandler for PingHandler {
    async fn init(_ctx: Arc<RwLock<TypeMap>>) {}

    async fn process(ctx: &Context, msg: &Message) {
        msg.reply(ctx, format!("pong!")).await.unwrap();
    }
}