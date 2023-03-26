pub mod gpt;

use crate::AppReg;
use serenity::builder::CreateCommand;

pub fn reg() -> Vec<CreateCommand> {
    let gpt = AppReg::normal_slash("gpt", "chat-gpt command").add_option(
        AppReg::subcommand(
            "chat",
            "chat with chat-gpt (using free token so expected rate limit)",
        )
        .add_sub_option(AppReg::str_option("ask", "what you ask to chat-gpt").required(true)),
    );
    vec![gpt]
}
