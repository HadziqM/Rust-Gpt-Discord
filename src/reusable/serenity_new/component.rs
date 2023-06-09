use serenity::all::*;
use crate::{Mytrait,MyErr};
use super::{Mybundle,SlashBundle};

pub struct Components;


impl Components{
    pub fn normal_button(name:&str,custom_id:&str,style:ButtonStyle,emoji:&str)->CreateButton{
        let mut b = CreateButton::new(custom_id).label(name).style(style);
        if let Ok(emj)=emoji.parse::<ReactionType>(){
            b = b.emoji(emj);
        }
        b
    }
    pub fn interaction_response(content:&str,ephemeral:bool)->CreateInteractionResponse{
        CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new().content(content).ephemeral(ephemeral)
            )
    }
    pub async fn response<T:Mybundle>(bnd:&T,content:&str,ephemeral:bool)->Result<(),MyErr>{
        let cmd = bnd.cmd();
        Ok(cmd.response(bnd.ctx(), Components::interaction_response(content, ephemeral)).await?)
    }
    pub async fn response_adv<T:Mybundle>(bnd:&T,content:CreateInteractionResponse)->Result<(),MyErr>{
        let cmd = bnd.cmd();
        Ok(cmd.response(bnd.ctx(), content).await?)
    }
    pub async fn edit<T:Mybundle>(bnd:&T,content:&str)->Result<(),MyErr>{
        let cmd = bnd.cmd();
        let rply = EditInteractionResponse::new().content(content);
        Ok(cmd.edit(bnd.ctx(), rply).await?)
    }
    pub async fn edit_adv<T:Mybundle>(bnd:&T,content:EditInteractionResponse)->Result<(),MyErr>{
        let cmd = bnd.cmd();
        Ok(cmd.edit(bnd.ctx(), content).await?)
    }
    pub fn sub_options<'a>(bnd:&'a SlashBundle<'_>)->Result<&'a Vec<CommandDataOption>,MyErr>{
        for data in &bnd.cmd.data.options{
            if let CommandDataOptionValue::SubCommand(x) = &data.value{
                return Ok(x);
            }
        }
        Err(MyErr::Custom("cant find subcommand".to_string()))
    }
}
