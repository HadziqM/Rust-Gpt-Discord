use serenity::{all::Interaction, prelude::Context};
use crate::SlashBundle;
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
        _=>{return ;}
    }
}
