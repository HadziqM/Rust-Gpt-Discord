pub mod ready;
pub mod interaction;


use serenity::{async_trait, prelude::EventHandler};


pub struct Handler;
use serenity::all::*;


#[async_trait]
impl EventHandler for Handler{
    async fn interaction_create(&self, ctx: Context, inter:Interaction) {
        interaction::handled(&ctx,&inter).await;
    }
    async fn ready(&self, ctx:Context, ready:Ready){
        ready::ready(&ctx, ready).await;
    }
}
