use crate::commands::command_handler::CommandHandler;

use serenity::async_trait;
use serenity::client::Context;
use serenity::model::channel::Message;

pub struct PingHandler {}

#[async_trait]
impl CommandHandler for PingHandler {
    fn init() {}

    async fn process(ctx: &Context, msg: &Message) {
        msg.reply(ctx, format!("pong!")).await.unwrap();
    }
}