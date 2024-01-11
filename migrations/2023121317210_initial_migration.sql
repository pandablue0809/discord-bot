-- guild settings schema
CREATE TABLE IF NOT EXISTS guild_settings (
    guild_id BIGINT NOT NULL UNIQUE,
    owner_id BIGINT NOT NULL,
    automod_enabled INTEGER NOT NULL DEFAULT 0,
    message_log_channel_id BIGINT,
    message_log_enabled INTEGER NOT NULL DEFAULT 0,
    mod_log_channel_id BIGINT,
    mod_log_enabled INTEGER NOT NULL DEFAULT 0,
    welcome_channel_id BIGINT,
    welcome_enabled INTEGER NOT NULL DEFAULT 0,
    welcome_message TEXT,
    prefix TEXT NOT NULL DEFAULT "-",
    mute_style TEXT NOT NULL DEFAULT "timeout",
    mute_duration INTEGER NOT NULL DEFAULT 3600,
    mute_role_id BIGINT,
    boosts INTEGER NOT NULL DEFAULT 0,
    boost_rewards_enabled INTEGER NOT NULL DEFAULT 0, -- role ids (to account for lists of roles) will be stored in another table
	PRIMARY KEY(guild_id)
);


-- user profile schema
CREATE TABLE IF NOT EXISTS user_profile (
    user_id BIGINT NOT NULL,
    guild_id BIGINT NOT NULL DEFAULT 0,
    first_joined_at TEXT NOT NULL,
    latest_joined_at TEXT NOT NULL,
    commands_ran INTEGER NOT NULL DEFAULT 0,
  	PRIMARY KEY(user_id, guild_id)
);

-- modlog schema
CREATE TABLE IF NOT EXISTS mod_log (
    id BIGINT NOT NULL,
    guild_id BIGINT NOT NULL,
    user_id BIGINT NOT NULL,
    moderator_id BIGINT NOT NULL,
    action_type TEXT NOT NULL,
    action_duration INTEGER,
    reason TEXT NOT NULL DEFAULT "No reason provided",
    time_created TEXT NOT NULL,
    PRIMARY KEY (guild_id, user_id, id)
);

-- admin role schema
CREATE TABLE IF NOT EXISTS admin_roles (
    guild_id BIGINT NOT NULL,
    role_id BIGINT NOT NULL,
    PRIMARY KEY (guild_id, role_id)
);

-- admin user schema
CREATE TABLE IF NOT EXISTS admin_users (
    guild_id BIGINT NOT NULL,
    user_id BIGINT NOT NULL,
    PRIMARY KEY (guild_id, user_id)
);

-- bot stats schema
CREATE TABLE IF NOT EXISTS bot_stats (
    guild_id BIGINT NOT NULL DEFAULT 0,
    commands_ran BIGINT NOT NULL DEFAULT 0,
    songs_played BIGINT NOT NULL DEFAULT 0,
  	PRIMARY KEY(guild_id)
);

-- music settings schema (TBD)
/*
CREATE TABLE IF NOT EXISTS music_settings (
    guild_id BIGINT NOT NULL,
    volume INTEGER NOT NULL DEFAULT 59, -- max volume is 100, also, add 10 to DEFAULT to get funny number
    loop_mode TEXT NOT NULL DEFAULT "off",
    autoplay INTEGER NOT NULL DEFAULT 0,
    DEFAULT_search TEXT NOT NULL DEFAULT "youtube",
    PRIMARY KEY (guild_id)
)


-- snipes schema
CREATE TABLE IF NOT EXISTS snipes (
    guild_id BIGINT NOT NULL,
    channel_id BIGINT NOT NULL,
    id INTEGER NOT NULL UNIQUE DEFAULT 0,
    message_content TEXT NOT NULL DEFAULT "No message found",
    message_attachment TEXT NOT NULL DEFAULT "No attachment found"
);

*/