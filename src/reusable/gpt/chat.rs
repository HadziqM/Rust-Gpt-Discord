use super::{MyErr,Gpt};
use serde::{Serialize,Deserialize};
use crate::{Mybundle,Components};
use serenity::all::*;

#[derive(Serialize,Deserialize,Clone)]
pub struct Choiches{
    pub message:Message
}
#[derive(Serialize,Deserialize,Clone)]
pub struct Message{
    pub content:String,
    role:String
}
#[derive(Serialize,Deserialize,Clone)]
pub struct Completition{
    pub choices:Vec<Choiches>
}
#[derive(Serialize,Clone)]
pub struct CompletitionData{
    model:String,
    pub messages:Vec<Message>
}
#[derive(Clone)]
pub struct CompModel{
    pub data:CompletitionData,
    pub comp:Completition
}
impl CompletitionData{
    fn new(messages:Vec<Message>)->Self{
        CompletitionData { model: "gpt-3.5-turbo".to_owned(), messages }
    }
}
impl Message{
    fn user(content:&str)->Self{
        Message { content:content.to_owned(), role: "user".to_owned() }
    }
    fn assistant(content:&str)->Self{
        Message { content:content.to_owned(), role: "assistant".to_owned() }
    }
}
impl CompModel{
    async fn cached(&self,id:&str){
        let mut chat = crate::CHAT.lock().await;
        match chat.get_mut(id){
            Some(x)=>{
                *x = self.clone();
            }
            None => {
                chat.insert(id.to_owned(), self.clone());
            }
        }
    }
    pub async fn retieve(id:&str)->Result<Self,MyErr>{
        let chat = crate::CHAT.lock().await;
        match chat.get(id){
            Some(x)=>{
                return Ok(x.to_owned());
            }
            None => {
                return Err(MyErr::Custom("the chat hystory is erased".to_owned()));
            }
        }
    }
    pub async fn send<T:Mybundle>(&self,bnd:&T,id:&str)->Result<(),MyErr>{
        let button = Components::normal_button("Reply", &format!("chat-{}",id), ButtonStyle::Primary, "🤨");
        Components::edit_adv(bnd, EditInteractionResponse::new()
            .components(vec![CreateActionRow::Buttons(vec![button])]).content(&self.comp.choices[0].message.content)).await?;
        self.cached(id).await;
        Ok(())
    }
}

impl Gpt{
    async fn comp(&self,data:&CompletitionData)->Result<Completition,MyErr>{
        let url = "https://api.openai.com/v1/chat/completions";
        let client =  reqwest::Client::new();
        let resp = client.post(url).headers(self.head.to_owned()).json(&data).send().await?.text().await?;
        let comp =match serde_json::from_str::<Completition>(&resp){
            Ok(x)=>x,
            Err(_)=>{
                return Err(MyErr::Custom(format!("getting invalid response with the parsed data:\n\n{}",&resp)));
            }
        };
        Ok(comp)
    }
    pub async fn completition(&self,ask:&str)->Result<CompModel,MyErr>{
        let data = CompletitionData::new(vec![Message::user(ask)]);
        let comp = self.comp(&data).await?;
        Ok(CompModel { data, comp })
    }
    pub async fn reply_comp(&self,data:&CompModel,ask:&str)->Result<CompModel,MyErr>{
        let mut n = data.clone();
        n.data.messages.push(Message::assistant(&data.comp.choices[0].message.content));
        n.data.messages.push(Message::user(ask));
        let comp = self.comp(&n.data).await?;
        Ok(CompModel { data:n.data, comp })
    }
}
#[cfg(test)]
mod testing{
    use super::*;

    #[tokio::test]
    async fn chat_gpt() {
        let gpt = Gpt::new().unwrap();
        let ask = gpt.completition("what is rust programing language?").await.unwrap();
        println!("rust is : {}\n",ask.comp.choices[0].message.content);
        let reply = gpt.reply_comp(&ask, "hello world example?").await.unwrap();
        println!("example : {}",reply.comp.choices[0].message.content)
    }
}
