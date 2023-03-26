pub mod reusable;
pub mod event;
pub mod command;


use lazy_static::lazy_static;
use std::collections::HashMap;
use reusable::{config::Init,MyErr,Mytrait,error_embed::ErrorLog,serenity_new::{reg::AppReg,SlashBundle,ComponentBundle,ModalBundle,Mybundle,component::Components}};
use serenity::{prelude::{Mutex, GatewayIntents}, Client, all::User};
use event::Handler;

lazy_static!{
    static ref COOLDOWN:Mutex<HashMap<String,i64>> = Mutex::new(HashMap::new());
    static ref CHAT:Mutex<HashMap<String,reusable::gpt::chat::CompModel>> = Mutex::new(HashMap::new());
    static ref INIT:Box<Init> = Box::new(Init::block_new().unwrap());
    static ref AUTHOR:Mutex<User> = Mutex::new(User::default());
}

#[tokio::main]
async fn main() {
    let intents = GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILD_MEMBERS;
    let mut client = Client::builder(INIT.discord.token.to_owned(), intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
