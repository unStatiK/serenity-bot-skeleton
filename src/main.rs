extern crate libc_alloc;

mod commands;
mod bot_core;

use std::env;
use std::sync::Arc;

use commands::command_handler::CommandHandler;
use commands::uptime::UptimeHandler;
use commands::hello::HelloHandler;
use commands::ping::PingHandler;
use bot_core::context::BotContext;

use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{StandardFramework, CommandResult};

use libc_alloc::LibcAlloc;

#[global_allocator]
static ALLOCATOR: LibcAlloc = LibcAlloc;

#[group]
#[commands(ping, hello, uptime)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    start().await;
}

async fn init_context(ctx: Arc<RwLock<TypeMap>>) {
    BotContext::init_context(ctx).await;
}

async fn init_command_system(ctx: Arc<RwLock<TypeMap>>) {
    UptimeHandler::init(ctx.clone()).await;
    HelloHandler::init(ctx.clone()).await;
    PingHandler::init(ctx.clone()).await;
}

async fn start() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&GENERAL_GROUP);
    let token = env::var("DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    init_context(client.data.clone()).await;
    init_command_system(client.data.clone()).await;

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    PingHandler::process(ctx, msg).await;
    Ok(())
}

#[command]
async fn hello(ctx: &Context, msg: &Message) -> CommandResult {
    HelloHandler::process(ctx, msg).await;
    Ok(())
}

#[command]
async fn uptime(ctx: &Context, msg: &Message) -> CommandResult {
    UptimeHandler::process(ctx, msg).await;
    Ok(())
}