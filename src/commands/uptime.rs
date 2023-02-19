use crate::commands::command_handler::CommandHandler;

use std::sync::Arc;
use std::time::SystemTime;

use serenity::async_trait;
use serenity::client::Context;
use serenity::prelude::{TypeMap, RwLock};
use serenity::model::channel::Message;

use chrono::DateTime;
use chrono::offset::Local;
use crate::bot_core::context::BotContext;

const UPTIME_KEY: &str = "uptime";

pub struct UptimeHandler;

#[async_trait]
impl CommandHandler for UptimeHandler {
    async fn init(ctx: Arc<RwLock<TypeMap>>) {
        let now = SystemTime::now();
        let datetime: DateTime<Local> = now.into();
        BotContext::set_meta_value(ctx, UPTIME_KEY.to_string(), datetime.format("%d/%m/%Y %T").to_string()).await;
    }

    async fn process(ctx: &Context, msg: &Message) {
        msg.reply(ctx, format!("```Bot start at {}```", BotContext::get_meta_value(ctx.data.clone(), UPTIME_KEY.to_string()).await)).await.unwrap();
    }
}