use crate::utilities::git::{get_current_branch, get_head_revision};

use itertools::Itertools;
use git2::Repository;
use serenity::{
    builder::{CreateEmbed, CreateEmbedFooter, CreateMessage, CreateEmbedAuthor},
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::prelude::Message, all::{ChannelType, VerificationLevel, MfaLevel, ExplicitContentFilter, PremiumTier, PremiumType}
};

use std::fmt::Write;

#[command]
#[aliases("info", "botinfo")]
#[description = "Shows information about the bot."]
#[max_args(0)]
async fn about(context: &Context, message: &Message) -> CommandResult {
    let repo = Repository::open(env!("CARGO_MANIFEST_DIR"))?;

    let version = env!("CARGO_PKG_VERSION").to_string();
    let codename = "Graf Zeppelin".to_string();
    let branch = get_current_branch(&repo);
    let revision = get_head_revision(&repo);

    let current_user = context.cache.current_user().clone();

    let bot_name = &current_user.name;
    let bot_avatar = &current_user.avatar_url().unwrap();
    let bot_owner = context.http.get_current_application_info().await?.owner.unwrap().tag();

    let num_shards = context.cache.shard_count();
    let num_guilds = context.cache.guilds().len();
    let num_channels = context.cache.guild_channel_count();
    let num_users = context.cache.user_count();

    let about_fields = vec![
        ("Version", version, true),
        ("Codename", codename.to_string(), true),
        ("Branch", branch, true),
        ("Revision", format!("`{revision}`"), true),
        ("Owner", bot_owner, true),
        ("Shards", num_shards.to_string(), true),
        ("Guilds", num_guilds.to_string(), true),
        ("Channels", num_channels.to_string(), true),
        ("Users", num_users.to_string(), true),
    ];

    let embed = CreateEmbed::new()
        .title(format!("**{bot_name}**"))
        .url("https://github.com/panzer-chan/graf_zeppelin")
        .thumbnail(bot_avatar)
        .color(0x00BFFF)
        .fields(about_fields)
        .footer(CreateEmbedFooter::new("Written with Rust & Serenity."));

    message.channel_id.send_message(&context, CreateMessage::new().embed(embed)).await?;

    Ok(())
}

#[command]
#[description = "Shows various information about the current guild."]
#[aliases("guild", "guildinfo", "ginfo", "server", "serverinfo", "serverstats", "sinfo")]
#[only_in(guilds)]
#[max_args(0)]
async fn guild(context: &Context, message: &Message) -> CommandResult {
    let cache = &context.cache;
    let guild_id = message.guild_id.unwrap();
    let guild_id_u64 = guild_id.get();
    let cached_guild = cache.guild(guild_id).unwrap().clone();

    let guild_icon = cached_guild.icon_url().unwrap();
    let guild_name = &cached_guild.name;
    let guild_owner = cached_guild.member(&context, cached_guild.owner_id).await.unwrap().user.tag();
    let guild_system_channel = cached_guild.system_channel_id.unwrap();
    let guild_system_channel_id = guild_system_channel.get();
    let guild_creation_date = guild_id.created_at().format("%B %e, %Y @ %l:%M %P");
    let guild_members = cached_guild.members.len();
    let guild_presences = cached_guild.presences.len();
    let guild_channels: Vec<_> = cached_guild.channels.values().map(|c| Some(c).unwrap()).collect();
    let guild_channels_all = guild_channels.len();
    let guild_channels_text = guild_channels.iter().filter(|c| c.kind == ChannelType::Text).count();
    let guild_channels_voice = guild_channels.iter().filter(|c| c.kind == ChannelType::Voice).count();
    let guild_emojis = cached_guild.emojis.len();
    let guild_emojis_animated = cached_guild.emojis.iter().filter(|(_, e)| e.animated).count();
    let guild_emojis_normal = cached_guild.emojis.iter().filter(|(_, e)| !e.animated).count();
    let guild_features = cached_guild.features.iter().join(", ");

    let guild_verification_level = match cached_guild.verification_level {
        VerificationLevel::None => "None - Unrestricted.",
        VerificationLevel::Low => "Low - Must have a verified email.",
        VerificationLevel::Medium => "Medium - Registered on Discord for 5+ minutes.",
        VerificationLevel::High => "(╯°□°)╯︵ ┻━┻ - In the server for 10+ minutes.",
        VerificationLevel::Higher => "┻━┻ ﾐヽ(ಠ益ಠ)/彡┻━┻) - Must have a verified phone number.",
        _ => "Unrecognized verification level."
    };

    let guild_mfa_level = match cached_guild.mfa_level {
        MfaLevel::None => "Multi-factor authentication not required.",
        MfaLevel::Elevated => "Multi-factor authentication required.",
        _ => "Unrecognized multi-factor authentication level."
    };

    let guild_explicit_filter = match cached_guild.explicit_content_filter {
        ExplicitContentFilter::None => "Disabled".to_owned(),
        ExplicitContentFilter::WithoutRole => "Media scanned from members w/o a role.".to_owned(),
        ExplicitContentFilter::All => "Everyone".to_owned(),
        _ => "Unrecognized filter setting.".to_owned()
    };

    let guild_boosts = cached_guild.premium_subscription_count.unwrap_or_default();
    let guild_boost_tier = match cached_guild.premium_tier {
        PremiumTier::Tier0 => "No current tier (not boosted)",
        PremiumTier::Tier1 => "Level 1 (2+ boosts)",
        PremiumTier::Tier2 => "Level 2 (15+ boosts)",
        PremiumTier::Tier3 => "Level 3 (30+ boosts)",
        _ => "Unrecognized boost tier."
    };

    let guild_roles_sorted = cached_guild.roles.iter().sorted_by_key(|&(_, r)| r.position).rev();
    let guild_roles_map = guild_roles_sorted.filter(|&(_, r)| r.id.get() != guild_id_u64).map(|(_, r)| &r.name).join(" / ");
    let guild_role_count = cached_guild.roles.iter().filter(|&(_, r)| r.id.get() != guild_id_u64).count();

    let mut highest = None;
    for role_id in cached_guild.roles.keys() {
        if let Some(role) = cached_guild.roles.get(role_id) {
            if let Some((id, pos)) = highest {
                if role.position < pos || (role.position == pos && role.id > id) {
                    continue;
                }
            }
            highest = Some((role.id, role.position));
        }
    }

    let highest_role_id = highest.map(|(id, _)| id).unwrap();
    let highest_role = cached_guild.roles.get(&highest_role_id).unwrap();
    let highest_role_name = &highest_role.name;
    let highest_role_color = highest_role.colour;

    let mut summary = String::new();
    writeln!(summary, "**Owner**: {guild_owner}")?;
    writeln!(summary, "**System Channel**: <#{guild_system_channel_id}>")?;
    writeln!(summary, "**Creation Date**: {guild_creation_date}")?;
    writeln!(summary, "**Online Members**: {guild_presences}")?;
    writeln!(summary, "**Total Members**: {guild_members}")?;
    writeln!(summary, "**Channels**: {guild_channels_all} ({guild_channels_text} text, {guild_channels_voice} voice)")?;
    writeln!(summary, "**Emojis**: {guild_emojis} ({guild_emojis_normal} static, {guild_emojis_animated} animated)")?;
    writeln!(summary, "**Features**: {}", if !guild_features.is_empty() { &guild_features } else { "None" })?;
    writeln!(summary, "**MFA Level**: {guild_mfa_level}")?;
    writeln!(summary, "**Verification Level**: {guild_verification_level}")?;
    writeln!(summary, "**Explicit Content Filter**: {guild_explicit_filter}")?;
    writeln!(summary, "**Nitro Boosts**: {guild_boosts}")?;
    writeln!(summary, "**Nitro Boost Level**: {guild_boost_tier}")?;
    writeln!(summary, "**Highest Role**: {highest_role_name}")?;
    writeln!(summary, "**Roles ({guild_role_count})**: {guild_roles_map}")?;

    let embed = CreateEmbed::new()
        .author(CreateEmbedAuthor::new(guild_name).icon_url(guild_icon))
        .colour(highest_role_color)
        .description(&summary)
        .footer(CreateEmbedFooter::new(format!("{guild_name} server ID: {guild_id}")));

    message.channel_id.send_message(&context, CreateMessage::new().embed(embed)).await?;

    Ok(())
}

#[command]
#[description = "Shows various information about a user."]
#[aliases("user", "userinfo")]
#[only_in("guilds")]
#[usage = "@user"]
async fn user_info(context: &Context, message: &Message) -> CommandResult {
    let user = match message.mentions.get(0) {
        Some(user) => user,
        None => &message.author
    };
    let user_id = user.id;
    let user_name = user.tag();
    let user_created = user.created_at();
    let user_avatar = user.face();
    let user_bot = user.bot;
    let user_nitro = match user.premium_type {
        PremiumType::None => "None",
        PremiumType::NitroClassic => "Nitro Classic",
        PremiumType::Nitro => "Nitro",
        PremiumType::NitroBasic => "Nitro Basic",
        _ => "Unrecognized Premium Type"
    };
    let guild = message.guild(&context.cache).unwrap().clone();
    let member = guild.member(&context.http, user_id).await.unwrap();

    let embed = CreateEmbed::new()
        .author(CreateEmbedAuthor::new(user_name.clone()).icon_url(user_avatar))
        .thumbnail(guild.icon_url().unwrap().to_string())
        .description(format!("Showing information about user {user_name}"))
        .field("ID", user_id.to_string(), true)
        .field("Created at", user_created.to_string(), true)
        .field("Is bot", user_bot.to_string(), true)
        .field("Joined server at", member.joined_at.unwrap().to_string(), true)
        .field("Nitro Subscription", user_nitro, true)
        .footer(CreateEmbedFooter::new(format!("User ID: {user_id}")));

    message.channel_id.send_message(&context, CreateMessage::new().embed(embed)).await?;

    Ok(())
}

#[command]
#[description = "Shows all of a user's profile pictures."]
#[aliases("profile", "avatar")]
#[only_in("guilds")]
#[usage = "@user"]
async fn user_avatars(context: &Context, message: &Message) -> CommandResult {
    let user = match message.mentions.get(0) {
        Some(user) => user,
        None => &message.author
    };
    let user_id = user.id;
    let user_name = user.tag();
    let guild = message.guild(&context.cache).unwrap().clone();
    let guild_avatar = guild.member(&context.http, user_id).await.unwrap().clone().avatar_url().unwrap_or("".to_string());

    if !guild_avatar.is_empty() { 
        let embed = CreateEmbed::new()
            .author(CreateEmbedAuthor::new(user_name.clone()).icon_url(user.face()))
            .description(format!("Showing profile pictures of {}", user_name.clone()))
            .image(user.face());

        let embed2 = CreateEmbed::new()
            .author(CreateEmbedAuthor::new(user_name.clone()).icon_url(guild_avatar.clone()))
            .description(format!("Showing profile pictures of {}", user_name.clone()))
            .image(guild_avatar.clone());

        message.channel_id.send_message(&context, CreateMessage::new().embeds(vec![embed, embed2])).await?;

    } else { 
        let embed = CreateEmbed::new()
            .author(CreateEmbedAuthor::new(user_name.clone()).icon_url(user.face()))
            .description(format!("Showing profile pictures of {user_name}"))
            .image(user.face());

        message.channel_id.send_message(&context, CreateMessage::new().embed(embed)).await?;
    }

    Ok(())
}