use discord_flows::{
    model::Message,
    ProvidedBot, Bot,
};
use flowsnet_platform_sdk::logger;
use chatgpt::prelude::*;
use hyper::body::Buf;
use hyper::{header, Body, Client, Request};
use hyper_tls::HttpsConnector;

#[derive(Deserialize, Debug)]
struct OAIChoices {
    text: String,
    index: u8,
    logprobs: Option<u8>,
    finish_reason: String,
}

#[derive(serde::Deserialize, Debug)]
struct Response{
    id: Option<String>,
    object: Option<String>,
    created: Option<u64>,
    model: Option<String>,
    choices: Vec<OAIChoices>
}

#[derive(serde::Deserialize, Debug)]
struct Request {
    prompt: String,
    max_tokens: u16,
}

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() -> anyhow::Result<()> {
    let discord_token = std::env::var("discord_token").unwrap();
    let bot = ProvidedBot::new(discord_token);
    bot.listen(|msg| handler(&bot, msg)).await;
    Ok(())
}


#[tokio::main]
async fn handler(bot: &ProvidedBot, msg: Message) {
    logger::init();
    let discord = bot.get_client();
    
    let gpt_key = std::env::var("gptKey").unwrap();

    let https = HttpsConnector::new();
    let client = Client::builder().build(https);
    let uri = "https://api.openai.com/v1/chat/completions";
    let author_header_val = format!("Bearer {}", gpt_key);


    let oai_request = Request {
        prompt: format!("{}", msg.content),
        max_tokens: 1000, 
    };

    let body = Body::from(serde_json::to_vec(&oai_request)?);

    let req = Request::post(uri)
        .header(header::CONTENT_TYPE, "application/json") 
        .header("Authorization", &auth_header_val) 
        .body(body) 
        .unwrap(); 

    
    let res = client.request(req).await?;

    let body = hyper::body::aggregate(res).await?; 
    
    let json: Response = serde_json::from_reader(body.reader())?;

    // let client = ChatGPT::new(gpt_key)?;

    // let response: CompletionResponse = client
    //     .send_message(msg.content)
    //     .await?;

    if msg.author.bot {
        log::debug!("ignored bot message");
        return;
    }
    if msg.member.is_some() {
        log::debug!("ignored channel message");
        return;
    }

    let channel_id = msg.channel_id;
    let resp = format!("{}",json.choices[0].text);

    _ = discord.send_message(
        channel_id.into(),
        &serde_json::json!({
            "content": resp
        }),
    ).await;
}
