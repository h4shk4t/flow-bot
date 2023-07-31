use discord_flows::{
    model::Message,
    ProvidedBot, Bot,
};
use flowsnet_platform_sdk::logger;
use chatgpt::prelude::*;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() -> anyhow::Result<()> {
    let discord_token = std::env::var("discord_token").unwrap();
    let bot = ProvidedBot::new(discord_token);
    bot.listen(|msg| handler(&bot, msg)).await;
    Ok(())
}

async fn handler(bot: &ProvidedBot, msg: Message) {
    logger::init();
    let discord = bot.get_client();
    
    let gpt_key = std::env::var("gptKey").unwrap();

    let client = ChatGPT::new(gpt_key)?;

    let response: CompletionResponse = client
        .send_message(msg.content)
        .await?;

    if msg.author.bot {
        log::debug!("ignored bot message");
        return;
    }
    if msg.member.is_some() {
        log::debug!("ignored channel message");
        return;
    }

    let channel_id = msg.channel_id;
    let resp = format!(response.message().content);

    _ = discord.send_message(
        channel_id.into(),
        &serde_json::json!({
            "content": resp
        }),
    ).await;
}
