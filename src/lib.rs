use discord_flows::{
    model::Message,
    ProvidedBot, Bot,
};
use flowsnet_platform_sdk::logger;

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

    let user_message = msg.content.trim().to_lowercase(); 

    let static_qa: Vec<(&str, &str)> = vec![
        ("what's your favorite song?", "My favorite song is 'Bohemian Rhapsody' by Queen."),
        ("who is your favorite artist?", "I love listening to music by Billie Eilish."),
        ("recommend me a song.", "I recommend 'Shape of You' by Ed Sheeran."),
        ("play some music.", "I'm just a text bot and can't play music, but I can recommend songs!"),
        ("What is your favourite language?", "I am a text bot, but my favourite language is Rust for sure!")
    ];
    
    if msg.author.bot {
        log::debug!("ignored bot message");
        return;
    }
    if msg.member.is_some() {
        log::debug!("ignored channel message");
        return;
    }

    let channel_id = msg.channel_id;

    if let Some(answer) = static_qa.iter().find(|(q, _)| user_message.contains(q)) {
        let resp = format!("{}", answer.1);

        _ = discord
            .send_message(
                channel_id.into(),
                &serde_json::json!({
                    "content": resp
                }),
            )
            .await;
    }
}