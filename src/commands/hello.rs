use crate::commands::command_handler::CommandHandler;

use serenity::async_trait;
use serenity::client::Context;
use serenity::model::channel::Message;

pub struct HelloHandler {}

#[async_trait]
impl CommandHandler for HelloHandler {
    fn init() {}

    async fn process(ctx: &Context, msg: &Message) {
        msg.reply(ctx, format!("hello {}!", msg.author.name)).await.unwrap();
    }
}