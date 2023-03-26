use serenity::all::*;
use super::{MyErr,Mytrait};
use super::serenity_new::Mybundle;

#[derive(Clone)]
pub struct ErrorLog<'a> {
    pub(crate) err: String,
    pub(crate) on:String,
    pub(crate) advice:String,
    pub(crate) ctx:&'a Context,
    pub(crate) usr:User,
    pub(crate) user:User,
}

impl<'a> ErrorLog<'a> {
    pub async fn from_bnd<T:Mybundle>(bnd:&'a T)->ErrorLog<'a>{
        let user = crate::AUTHOR.lock().await.to_owned();
        ErrorLog { 
            err: String::new(), 
            on: String::new(), advice:String::new(), 
            ctx:bnd.ctx(), 
            usr:bnd.user(),
            user,
        }
    }
    pub async fn new(ctx:&'a Context,usr:User)->ErrorLog<'a>{
        let user = crate::AUTHOR.lock().await.to_owned();
        ErrorLog { 
            err: String::new(), 
            on: String::new(), advice:String::new(), 
            ctx, 
            usr,
            user,
        }
    }
        pub fn change_error(&mut self,error:String,on:String,advice:String){
        self.err = error;
        self.on = on;
        self.advice = advice;
    }
    pub fn change_why(&mut self,error:String){
        self.err = error;
    }
    pub fn make_embed(&self,severity:bool)->CreateEmbed{
        let color = ||{if severity{return Color::RED;}Color::ORANGE};
        CreateEmbed::new()
        .title("🛑 Error Occured 🛑")
        .description("some cant be handled error occured")
        .fields(vec![
            ("🚧 occured on",format!("**{}**",self.on.to_uppercase()),false),
            ("📜 error message",format!("> {}",&self.err),false),
            ("⛑  author advice",format!("```\n{}\n```",&self.advice),false)
        ])
        .author(CreateEmbedAuthor::new(&self.usr.name).icon_url(self.usr.face()))
        .footer(CreateEmbedFooter::new(format!("you can consult this to {}",self.user.tag()))
            .icon_url(self.user.face()))
        .color(color())
        .image(&crate::INIT.discord.err_image)
    }
    pub async fn log_error_channel(&self,severity:bool){
        let ch_id = ChannelId::new(crate::INIT.discord.err_channel);
        if let Err(why) = ch_id.send_message(&self.ctx.http,CreateMessage::new()
            .embed(self.make_embed(severity)).content(format!("for {}",self.usr.to_string()))).await{
            println!("cant send error message to discord channel :{}",why)
        }
    }
    pub fn interaction_response(&self,ephemeral:bool,severity:bool)->CreateInteractionResponse{
        CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().embed(self.make_embed(severity)).ephemeral(ephemeral))
    }
    pub fn defer_response(&self,severity:bool)->EditInteractionResponse{
        EditInteractionResponse::new().embed(self.make_embed(severity))
    }
}

impl MyErr{
    pub async fn log_channel(&self,err:&ErrorLog<'_>){
        err.log_error_channel(self.severity()).await
    }
    pub async fn log_channel_ch(&self,err:&mut ErrorLog<'_>,on:&str){
        err.change_error(self.get(), on.to_owned(), self.advice());
        err.log_error_channel(self.severity()).await
    }
    pub async fn log_msg(&self,msg:&Message,on:&str,err:&mut ErrorLog<'_>){
        err.change_error(self.get(), on.to_owned(), self.advice());
        if let Err(why) = msg.channel_id.send_message(&err.ctx.http, CreateMessage::new().embed(err.make_embed(self.severity()))).await{
            MyErr::from(why).log_channel(err).await
        }
    }
    pub async fn log_defer<T:Mytrait>(&self,cmd:&T,on:&str,err:&mut ErrorLog<'_>){
        err.change_error(self.get(), on.to_owned(), self.advice());
        if let MyErr::Serenity(_) = self{
            err.log_error_channel(self.severity()).await
        }else{
            cmd.err_defer(err).await;
        }
    }
    pub async fn log<T:Mytrait>(&self,cmd:&T,on:&str,ephemeral:bool,err:&mut ErrorLog<'_>){
        err.change_error(self.get(), on.to_owned(), self.advice());
        if let MyErr::Serenity(_) = self{
            err.log_error_channel(self.severity()).await
        }else{
            cmd.err_response(err, ephemeral).await;
        }
    }
}
