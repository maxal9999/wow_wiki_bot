use api::Api;
use api::types::{User, UrlParams, Message};
use api::listener::Listener;
use config::Config;
use error::Result;

/// Struct for work with telegram bot 
pub struct TelegramBotApi {
    pub api: Api
}

impl Clone for TelegramBotApi {
    fn clone(&self) -> TelegramBotApi {
        TelegramBotApi {
            api: self.api.clone()
        }
    }
}

impl TelegramBotApi {
    /// Create a new bot by the config.toml.
    pub fn new(conf: &Config) -> Result<TelegramBotApi> {
        Ok(TelegramBotApi {
            api: Api::new(conf.get_telegram_bot_endpoint())
        })
    }

    /// Corresponds to the "getMe" method of the API.
    pub fn get_me(&self) -> Result<User> {
        // Execute request with empty parameter list
        self.api.send_request("getMe", UrlParams::new())
    }

    /// Corresponds to the "sendMessage" method of the API.
    pub fn send_message(&self, chat_id: i64, text: String,
                        disable_web_page_preview: Option<bool>,
                        reply_to_message_id: Option<i64>)
                        -> Result<Message> {
        // Prepare parameters
        let mut params = UrlParams::new();
        params.add_value("chat_id", chat_id);
        params.add_value("text", text);
        params.add_opt_value("disable_web_page_preview", disable_web_page_preview);
        params.add_opt_value("reply_to_message_id", reply_to_message_id);

        // Execute request
        self.api.send_request("sendMessage", params)
    }

    /// Get Listener object
    pub fn listener(&self, timeout_value: Option<i64>) -> Listener {
        Listener {
            confirmed: 0,
            url: self.api.url.clone(),
            client: Api::create_default_client(),
            timeout: timeout_value
        }
    }
}