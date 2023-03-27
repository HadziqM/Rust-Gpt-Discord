use serenity::{all::Interaction, prelude::Context};
use crate::{SlashBundle,ComponentBundle};
use crate::command;

pub async fn handled(ctx:&Context,int:&Interaction){
    match int{
        Interaction::Command(cmd)=>{
            let bnd = SlashBundle{ctx,cmd};
            let wth = cmd.data.name.as_str();
            match wth{
                "gpt" => command::gpt::discord_slash(&bnd).await,
                _=> {return;}
            }
        }
        Interaction::Component(cmd)=>{
            let bnd = ComponentBundle{ctx,cmd};
            if cmd.data.custom_id.contains("chat"){
                command::gpt::discord_button(&bnd).await
            }
        }
        _=>{return ;}
    }
}
