pub mod reg;
pub mod component;

use std::time::SystemTime;
use serenity::all::*;
use crate::{MyErr,Mytrait,COOLDOWN};

pub struct MyTime;

impl MyTime {
    pub fn now()-> i64{
        i64::try_from(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()).unwrap()
    }
    pub fn elapsed(el:i64)-> i64{
        MyTime::now() + el
    }
}

pub struct SlashBundle<'a>{
    pub cmd:&'a CommandInteraction,
    pub ctx:&'a Context
}
pub struct ComponentBundle<'a>{
    pub cmd:&'a ComponentInteraction,
    pub ctx:&'a Context,
}
pub struct ModalBundle<'a>{
    pub cmd:&'a ModalInteraction,
    pub ctx:&'a Context,
}

#[async_trait]
pub trait Mybundle {
    type Cmd:Mytrait;
    fn ctx<'a>(&'a self)->&'a Context;
    fn user(&self)->User;
    fn cmd<'a>(&'a self)->&'a Self::Cmd;
    fn name(&self)->String;
    async fn cd_check(&self,cd:i64)->Result<(),MyErr>;
    async fn cooldown(&self,cd:i64);
}


#[async_trait]
impl Mybundle for SlashBundle<'_>{
    type Cmd = CommandInteraction;
    fn ctx<'a>(&'a self)->&'a Context {
        self.ctx
    }
    fn user(&self)->User {
        self.cmd.user.clone()
    }
    fn cmd<'a>(&'a self)->&'a Self::Cmd {
        self.cmd
    }
    fn name(&self)->String {
        self.cmd.data.name.clone()
    }
    async fn cd_check(&self,time:i64)->Result<(),MyErr> {
        if time==0{
            return Ok(());
        }
        let pat = format!("{}-{}",self.name(),self.user().id.to_string());
        let mut cd = COOLDOWN.lock().await;
        let now = MyTime::now();
        match cd.get_mut(&pat){
            Some(x)=>{
                if *x as i64 > now{
                    return Err(MyErr::Custom(format!("youare still on cooldown to use this command wait till <t:{}:R>",x)));
                }else{
                    return Ok(());
                }
            }
            None=>{
                return Ok(());
            }
        }
    }
    async fn cooldown(&self,time:i64){
        let pat = format!("{}-{}",self.name(),self.user().id.to_string());
        let mut cd = COOLDOWN.lock().await;
        let now = MyTime::now();
        match cd.get_mut(&pat){
            Some(x)=>{
                *x = now + time;
            }
            None=>{
                cd.insert(pat.to_owned(), now+time);
            }
        }
    }
}
#[async_trait]
impl Mybundle for ComponentBundle<'_>{
    type Cmd = ComponentInteraction;
        fn ctx<'a>(&'a self)->&'a Context {
        self.ctx
    }
    fn user(&self)->User {
        self.cmd.user.clone()
    }
    fn cmd<'a>(&'a self)->&'a Self::Cmd {
        self.cmd
    }
    fn name(&self)->String {
        self.cmd.data.custom_id.clone()
    }
    async fn cd_check(&self,time:i64)->Result<(),MyErr> {
        if time==0{
            return Ok(());
        }
        let pat = format!("{}-{}",self.name(),self.user().id.to_string());
        let mut cd = COOLDOWN.lock().await;
        let now = MyTime::now();
        match cd.get_mut(&pat){
            Some(x)=>{
                if *x as i64 > now{
                    return Err(MyErr::Custom(format!("youare still on cooldown to use this command wait till <t:{}:R>",x)));
                }else{
                    return Ok(());
                }
            }
            None=>{
                return Ok(());
            }
        }
    }
    async fn cooldown(&self,time:i64){
        let pat = format!("{}-{}",self.name(),self.user().id.to_string());
        let mut cd = COOLDOWN.lock().await;
        let now = MyTime::now();
        match cd.get_mut(&pat){
            Some(x)=>{
                *x = now + time;
            }
            None=>{
                cd.insert(pat.to_owned(), now+time);
            }
        }
    }
}
#[async_trait]
impl Mybundle for ModalBundle<'_>{
    type Cmd = ModalInteraction;
        fn ctx<'a>(&'a self)->&'a Context {
        self.ctx
    }
    fn user(&self)->User {
        self.cmd.user.clone()
    }
    fn cmd<'a>(&'a self)->&'a Self::Cmd {
        self.cmd
    }
    fn name(&self)->String {
        self.cmd.data.custom_id.clone()
    }
    async fn cd_check(&self,time:i64)->Result<(),MyErr> {
        if time==0{
            return Ok(());
        }
        let pat = format!("{}-{}",self.name(),self.user().id.to_string());
        let mut cd = COOLDOWN.lock().await;
        let now = MyTime::now();
        match cd.get_mut(&pat){
            Some(x)=>{
                if *x as i64 > now{
                    return Err(MyErr::Custom(format!("youare still on cooldown to use this command wait till <t:{}:R>",x)));
                }else{
                    return Ok(());
                }
            }
            None=>{
                return Ok(());
            }
        }
    }
    async fn cooldown(&self,time:i64){
        let pat = format!("{}-{}",self.name(),self.user().id.to_string());
        let mut cd = COOLDOWN.lock().await;
        let now = MyTime::now();
        match cd.get_mut(&pat){
            Some(x)=>{
                *x = now + time;
            }
            None=>{
                cd.insert(pat.to_owned(), now+time);
            }
        }
    }
}
