pub mod error_embed;
pub mod config;
pub mod gpt;
pub mod serenity_new;

use serenity::async_trait;
use serenity::all::*;
use error_embed::ErrorLog;

#[derive(Debug)]
pub enum MyErr{
    Tokio(tokio::io::Error),
    Serde(serde_json::Error),
    Serenity(serenity::Error),
    Reqwest(reqwest::Error),
    Custom(String)
}
impl std::error::Error for MyErr{}
impl std::fmt::Display for MyErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            MyErr::Tokio(x)=>x.fmt(f),
            MyErr::Serenity(x)=>x.fmt(f),
            MyErr::Serde(x)=>x.fmt(f),
            MyErr::Reqwest(x)=>x.fmt(f),
            MyErr::Custom(x)=>x.fmt(f),
        }
    }
}
impl From<reqwest::Error> for MyErr {
    fn from(value: reqwest::Error) -> Self {
        MyErr::Reqwest(value)
    }
}
impl From<tokio::io::Error> for MyErr{
    fn from(value: tokio::io::Error) -> Self {
        MyErr::Tokio(value)
    }
}
impl From<serenity::Error> for MyErr {
    fn from(value: serenity::Error) -> Self {
        MyErr::Serenity(value)
    }
}
impl From<serde_json::Error> for MyErr {
    fn from(value: serde_json::Error) -> Self {
        MyErr::Serde(value)
    }
}

impl MyErr {
    pub(super) fn get(&self)->String{
        match self {
            MyErr::Custom(x)=>x.to_string(),
            MyErr::Reqwest(x)=>x.to_string(),
            MyErr::Tokio(x)=>x.to_string(),
            MyErr::Serde(x)=>x.to_string(),
            MyErr::Serenity(x)=>x.to_string(),
        }
    }
    pub(super) fn severity(&self)->bool{
        match self {
            MyErr::Custom(_)=>false,
            MyErr::Reqwest(_)=>true,
            MyErr::Tokio(_)=>true,
            MyErr::Serde(_)=>false,
            MyErr::Serenity(_)=>false,
        }
    }
    pub(super) fn advice(&self)->String{
        match self {
            MyErr::Custom(_)=>"Error message is writen by author themself, please read the message carefully or consult".to_string(),
            MyErr::Tokio(_)=>"file system error or paralel thread broken, report this!".to_string(),
            MyErr::Serde(_)=>"failed to operate with json data".to_string(),
            MyErr::Serenity(_)=>"discord error, well discord unreasonably do this sometime, but rest assured, whatever you do, its already finished successfully, but if you find you missing something, you could report this".to_string(),
            MyErr::Reqwest(_)=>"error on api client connection, you can report this to be investigated".to_owned(),
        }
    }
}

#[async_trait]
pub trait Mytrait{
    async fn err_response(&self,err:&ErrorLog<'_>,ephemeral:bool);
    async fn err_defer(&self,err:&ErrorLog<'_>);
    async fn defer_res(&self,err:&mut ErrorLog<'_>,on:&str,ephemeral:bool);
    async fn response(&self,ctx:&Context,rply:CreateInteractionResponse)->Result<(),MyErr>;
    async fn get_msg(&self,ctx:&Context)->Result<Message,MyErr>;
    async fn edit(&self,ctx:&Context,rlpy:EditInteractionResponse)->Result<(),MyErr>;
    fn user(&self)->User;
}

#[async_trait]
impl Mytrait for CommandInteraction{
    async fn err_response(&self,err:&ErrorLog<'_>,ephemeral:bool){
        if let Err(why) = self.create_response(&err.ctx.http, err.interaction_response(ephemeral,false)).await{
            MyErr::from(why).log_channel(err).await
        }
    }
    async fn err_defer(&self,err:&ErrorLog<'_>){
        if let Err(why) = self.edit_response(&err.ctx.http, err.defer_response(false)).await{
            MyErr::from(why).log_channel(err).await
        }
    }
    async fn defer_res(&self,err:&mut ErrorLog<'_>,on:&str,ephemeral:bool){
        let res;
        if ephemeral{
            res = self.defer_ephemeral(&err.ctx.http).await;
        }else{
            res = self.defer(&err.ctx.http).await;
        }
        if let Err(why) = res{
            let er = MyErr::from(why);
            err.change_error(er.get(),on.to_owned(), er.advice());
            err.log_error_channel(er.severity()).await;
        }
    }
    async fn response(&self,ctx:&Context,rply:CreateInteractionResponse)->Result<(),MyErr>{
        self.create_response(&ctx.http, rply).await?;
        Ok(())
    }
    async fn get_msg(&self,ctx:&Context)->Result<Message,MyErr>{
        Ok(self.get_response(&ctx.http).await?)
    }
    async fn edit(&self,ctx:&Context,rlpy:EditInteractionResponse)->Result<(),MyErr>{
        self.edit_response(&ctx.http, rlpy).await?;
        Ok(())
    }
    fn user(&self)->User{
        self.user.clone()
    }
}
#[async_trait]
impl Mytrait for ModalInteraction{
    async fn err_response(&self,err:&ErrorLog<'_>,ephemeral:bool){
        if let Err(why) = self.create_response(&err.ctx.http, err.interaction_response(ephemeral,false)).await{
            MyErr::from(why).log_channel(err).await
        }
    }
    async fn err_defer(&self,err:&ErrorLog<'_>){
        if let Err(why) = self.edit_response(&err.ctx.http, err.defer_response(false)).await{
            MyErr::from(why).log_channel(err).await
        }
    }
    async fn defer_res(&self,err:&mut ErrorLog<'_>,on:&str,_ephemeral:bool){
        let res = self.defer(&err.ctx.http).await;
        if let Err(why) = res{
            let er = MyErr::from(why);
            err.change_error(er.get(),on.to_owned(), er.advice());
            err.log_error_channel(er.severity()).await;
        }
    }
    async fn response(&self,ctx:&Context,rply:CreateInteractionResponse)->Result<(),MyErr>{
        self.create_response(&ctx.http, rply).await?;
        Ok(())
    }
    async fn get_msg(&self,ctx:&Context)->Result<Message,MyErr>{
        Ok(self.get_response(&ctx.http).await?)
    }
    async fn edit(&self,ctx:&Context,rlpy:EditInteractionResponse)->Result<(),MyErr>{
        self.edit_response(&ctx.http, rlpy).await?;
        Ok(())
    }
    fn user(&self)->User{
        self.user.clone()
    }
}

#[async_trait]
impl Mytrait for ComponentInteraction{
    async fn err_response(&self,err:&ErrorLog<'_>,ephemeral:bool){
        if let Err(why) = self.create_response(&err.ctx.http, err.interaction_response(ephemeral,false)).await{
            MyErr::from(why).log_channel(err).await
        }
    }
    async fn err_defer(&self,err:&ErrorLog<'_>){
        if let Err(why) = self.edit_response(&err.ctx.http, err.defer_response(false)).await{
            MyErr::from(why).log_channel(err).await
        }
    }
    async fn defer_res(&self,err:&mut ErrorLog<'_>,on:&str,_ephemeral:bool){
        let res = self.defer(&err.ctx.http).await;
        if let Err(why) = res{
            let er = MyErr::from(why);
            err.change_error(er.get(),on.to_owned(), er.advice());
            err.log_error_channel(er.severity()).await;
        }
    }
    async fn response(&self,ctx:&Context,rply:CreateInteractionResponse)->Result<(),MyErr>{
        self.create_response(&ctx.http, rply).await?;
        Ok(())
    }
    async fn get_msg(&self,ctx:&Context)->Result<Message,MyErr>{
        Ok(self.get_response(&ctx.http).await?)
    }
    async fn edit(&self,ctx:&Context,rlpy:EditInteractionResponse)->Result<(),MyErr>{
        self.edit_response(&ctx.http, rlpy).await?;
        Ok(())
    }
    fn user(&self)->User{
        self.user.clone()
    }
}
