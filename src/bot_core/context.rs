use std::{sync::Mutex, collections::HashMap};

use once_cell::sync::Lazy;

pub static GLOBAL_CONTEXT: Lazy<Mutex<BotContext>> = Lazy::new(|| {
    Mutex::new(BotContext::init())
});

pub struct BotContext {
    bot_meta: HashMap<String, String>,
}

impl BotContext {
    pub fn init() -> BotContext {
        BotContext {
            bot_meta: HashMap::new()
        }
    }

    pub fn get_meta_value(&mut self, key: String) -> String {
        match self.bot_meta.get(key.as_str()) {
            Some(x) => x.to_string(),
            None => "".to_string()
        }
    }

    pub fn set_meta_value(&mut self, key: String, value: String) {
        self.bot_meta.insert(key, value);
    }
}