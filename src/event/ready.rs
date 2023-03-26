use serenity::gateway::ActivityData;
use serenity::model::prelude::{GuildId, UserId};
use serenity::model::prelude::Ready;
use serenity::prelude::*;


pub async fn ready(ctx:&Context, ready:Ready){
    let user = UserId::new(crate::INIT.discord.author).to_user(&ctx.http).await.unwrap();
    let mut c_user = crate::AUTHOR.lock().await;
    *c_user = user.clone();
    println!("----------------------------------------------------------------");
    println!("-------------------------- START -------------------------------");
    println!("----------------------------------------------------------------");
    println!("🤖 Bot is running as {}",ready.user.tag());
    println!("🛠 {} is acknowledged as author",user.tag());
    let command = crate::command::reg();
    for guild in &ready.guilds{
        let x = guild.id.to_partial_guild(&ctx.http).await.unwrap();
        println!("🏛 {} is on guild **{}**",&ready.user.tag(),&x.name);
        GuildId::set_application_commands(guild.id, &ctx.http,command.clone()).await.unwrap();
    }
    ctx.set_activity(Some(ActivityData::competing("I want to die")));
}
