use std::{sync::Arc, collections::HashMap};

use serenity::prelude::{TypeMap, TypeMapKey, RwLock};

pub struct BotMeta;

impl TypeMapKey for BotMeta {
    type Value = Arc<RwLock<HashMap<String, String>>>;
}

pub struct BotContext;

impl BotContext {
    pub async fn init_context(ctx: Arc<RwLock<TypeMap>>) {
        let mut data = ctx.write().await;
        data.insert::<BotMeta>(Arc::new(RwLock::new(HashMap::new())));
    }

    pub async fn get_meta_value(ctx: Arc<RwLock<TypeMap>>, key: String) -> String {
        let data_read = ctx.read().await;
        let bot_meta_lock =
            data_read.get::<BotMeta>().expect("Expected BotMeta in TypeMap.").clone();
        let bot_meta = bot_meta_lock.read().await;
        match bot_meta.get(key.as_str()) {
            Some(x) => x.to_string(),
            None => "".to_string()
        }
    }

    pub async fn set_meta_value(ctx: Arc<RwLock<TypeMap>>, key: String, value: String) {
        let bot_meta_lock = {
            let data_read = ctx.read().await;
            data_read.get::<BotMeta>().expect("Expected BotMeta in TypeMap.").clone()
        };
        let mut rz_meta = bot_meta_lock.write().await;
        rz_meta.insert(key, value);
    }
}