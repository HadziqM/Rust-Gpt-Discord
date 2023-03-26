use std::path::{PathBuf, Path};

use serde::Deserialize;

use super::MyErr;

#[derive(Deserialize)]
pub struct Init {
    pub discord: Discord,
    pub gpt:GptConf
}

#[derive(Deserialize)]
pub struct Discord{
    pub token:String,
    pub err_channel:u64,
    pub err_image:String,
    pub author:u64
}
#[derive(Deserialize)]
pub struct GptConf{
    pub token:String
}

impl Init {
    fn path()->PathBuf{
        Path::new(".").join("config.json")
    }
    pub async fn new()->Result<Init,MyErr>{
        let input = tokio::fs::read_to_string(&Init::path()).await?;
        Ok(serde_json::from_str(&input)?)
    }
    pub fn block_new()->Result<Init,MyErr>{
        let input = std::fs::read_to_string(&Init::path())?;
        Ok(serde_json::from_str(&input)?)
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn config() {
        Init::block_new().unwrap();
    }
}
