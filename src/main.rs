extern crate hyper;
extern crate hyper_rustls;
extern crate rustc_serialize;
extern crate serde_json;
extern crate serde;
extern crate url;
#[macro_use]
extern crate serde_derive;

mod api;
mod config;
pub mod error;
mod telegram;

use api::listener::ListeningAction;
use std::process;
use telegram::TelegramBotApi;


fn main() {
    // Get arguments list
    let cmd_args: Vec<_> = std::env::args().collect();

    if cmd_args.len() < 2 {
        println!("Illegal parameter use: specify path to config file");
        process::exit(1);
    }

    // Config object
    let cfg = config::Config::new(&cmd_args[1]);

    // Api object for interaction with telegram bot
    let api = TelegramBotApi::new(&cfg).unwrap();

    // Check activity of telegram bot
    match api.get_me() {
        Ok(user) => println!("getMe: {:?}", user.first_name),
        _ => println!("Not getMe")
    }
    
    // Listener object for get updates of telegram chat
    let mut listener = api.listener(None);

    let _res = listener.listen(|update| {
    	if let Some(user_msg) = update.message {
            let msg_text = match user_msg.text {
                Some(res_msg) => res_msg,
                None => "".to_string()
            };

            if msg_text.len() == 0 {
                return Ok(ListeningAction::Continue);
            }

            try!(api.send_message(user_msg.chat.id, msg_text, None, Some(user_msg.message_id)));
    	}
    	Ok(ListeningAction::Continue)
    });
}