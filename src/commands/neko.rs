use serenity::{framework::standard::{macros::command, CommandResult}, client::Context, all::Message, builder::{CreateMessage, CreateEmbed, CreateEmbedFooter}};
//use tracing::info;
//use tracing::info;
use crate::utilities::{global_data::ReqwestClientContainer, containers::Items};

#[command]
#[description = "Sends random neko images."]
#[aliases("random", "rand")]
#[bucket = "neko"]
async fn random(context: &Context, message: &Message) -> CommandResult {
    
    let request = {
        let data = context.data.read().await;

        let request = data.get::<ReqwestClientContainer>().unwrap();

        request.clone()  
    };

    let params = [
        ("rating", "safe"),
        ("limit", "1")
    ];

    let url = reqwest::Url::parse_with_params("https://api.nekosapi.com/v3/images/random", params).unwrap();

    let res = request.get(url).send().await;

    let res = match res {
        Ok(res) => res,
        Err(_) => {
            message.reply(&context.http, "Failed to get image.").await?;
            return Ok(());
        }
    };

    //info!("{}", res.text().await?);

    let data = res.json::<Items>().await;

    let data = match data {
        Ok(data) => data,
        Err(_) => {
            message.reply(&context.http, "Failed to get image.").await?;
            return Ok(());
        }
    };

    let image_url = data.items[0].image_url.clone();
    let id = data.items[0].id.clone().to_string();

    let embed = CreateMessage::new()
        .embeds(vec![CreateEmbed::new()
            .image(image_url)
            .title("Random Neko Image!")
            .description(id)
            .colour(0xff0055)
            .footer(CreateEmbedFooter::new("Powered by https://nekosapi.com"))]);

    message.channel_id.send_message(&context, embed).await?;

    Ok(())
}

#[command]
#[description = "Sends a catgirl image."]
#[aliases("catgirl", "cg")]
#[bucket = "neko"]
async fn catgirl(context: &Context, message: &Message) -> CommandResult {

    let request = {
        let data = context.data.read().await;

        let request = data.get::<ReqwestClientContainer>().unwrap();

        request.clone()  
    };

    let params = [
        ("limit", "1"),
        ("tag", "8"),
        ("rating", "safe")
    ];

    let url = reqwest::Url::parse_with_params("https://api.nekosapi.com/v3/images/random", params).unwrap();

    //info!("{}", url);

    let res = request.get(url).send().await;

    let res = match res {
        Ok(res) => res,
        Err(_) => {
            message.reply(&context.http, "Failed to get image.").await?;
            return Ok(());
        }
    };

    // info!("{:?}", res.text().await?);

    let data = res.json::<Items>().await;

    let data = match data {
        Ok(data) => data,
        Err(_) => {
            message.reply(&context.http, "Failed to get image.").await?;
            return Ok(());
        }
    };

    let image_url = data.items[0].image_url.clone();
    let id = data.items[0].id.clone().to_string();

    let embed = CreateMessage::new()
        .embeds(vec![CreateEmbed::new()
            .image(image_url)
            .title("Catgirl Image!")
            .description(id)
            .colour(0xff0055)
            .footer(CreateEmbedFooter::new("Powered by https://nekosapi.com"))]);

    message.channel_id.send_message(&context, embed).await?;

    Ok(())

}

#[command]
#[description = "Sends an image of an usagimimi."]
#[aliases("usagimimi", "um")]
#[bucket = "neko"]
async fn usagimimi(context: &Context, message: &Message) -> CommandResult {

    let request = {
        let data = context.data.read().await;

        let request = data.get::<ReqwestClientContainer>().unwrap();

        request.clone()  
    };

    let params = [
        ("limit", "1"),
        ("tag", "36"),
        ("rating", "safe")
    ];

    let url = reqwest::Url::parse_with_params("https://api.nekosapi.com/v3/images/random", params).unwrap();

    let res = request.get(url).send().await;

    let res = match res {
        Ok(res) => res,
        Err(_) => {
            message.reply(&context.http, "Failed to get image.").await?;
            return Ok(());
        }
    };

    let data = res.json::<Items>().await;

    let data = match data {
        Ok(data) => data,
        Err(_) => {
            message.reply(&context.http, "Failed to get image.").await?;
            return Ok(());
        }
    };

    let image_url = data.items[0].image_url.clone();
    let id = data.items[0].id.clone().to_string();
    
    let embed = CreateMessage::new()
        .embeds(vec![CreateEmbed::new()
            .image(image_url)
            .title("Usagimimi Image!")
            .description(id)
            .colour(0xff0055)
            .footer(CreateEmbedFooter::new("Powered by https://nekosapi.com"))]);

    message.channel_id.send_message(&context, embed).await?;

    Ok(())

}