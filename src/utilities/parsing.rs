use serenity::{all::{Member, GuildId}, client::Context, Error};


pub async fn parse_user(text: &str, context: &Context, guild_id: GuildId) -> Result<Member, Error> {

    let to_trim: &[_] = &['<', '@', '!', '>'];
    
    let stripped = text.trim_matches(to_trim);

    let id = stripped.parse().unwrap();

    let member = context.http.get_member(guild_id, id).await?;

    Ok(member)
}