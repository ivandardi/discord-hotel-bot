use dotenv_codegen::dotenv;
use poise::serenity_prelude as serenity;

use anyhow::Context as _;
use shuttle_poise::ShuttlePoise;
use shuttle_secrets::SecretStore;

// Custom user data passed to all command functions
pub struct Data {
    pub discord_role_everyone: u64,
    pub discord_role_hotel_member: u64,
    pub discord_category_rooms: u64,
    pub discord_guild: u64,
}

impl Data {
    pub fn new(secret_store: &SecretStore) -> Self {
        Self {
            discord_role_everyone: secret_store.get("DISCORD_ROLE_EVERYONE")
                .context("Failed to get 'DISCORD_ROLE_EVERYONE' from the secret store")
                .expect("Failed to get 'DISCORD_ROLE_EVERYONE' from the secret store")
                .parse()
                .context("Failed to parse 'DISCORD_ROLE_EVERYONE' as u64")
                .expect("Failed to parse 'DISCORD_ROLE_EVERYONE' as u64"),
            discord_role_hotel_member: secret_store.get("DISCORD_ROLE_HOTEL_MEMBER")
                .context("Failed to get 'DISCORD_ROLE_HOTEL_MEMBER' from the secret store")
                .expect("Failed to get 'DISCORD_ROLE_HOTEL_MEMBER' from the secret store")
                .parse()
                .context("Failed to parse 'DISCORD_ROLE_HOTEL_MEMBER' as u64")
                .expect("Failed to parse 'DISCORD_ROLE_HOTEL_MEMBER' as u64"),
            discord_category_rooms: secret_store.get("DISCORD_CATEGORY_ROOMS")
                .context("Failed to get 'DISCORD_CATEGORY_ROOMS' from the secret store")
                .expect("Failed to get 'DISCORD_CATEGORY_ROOMS' from the secret store")
                .parse()
                .context("Failed to parse 'DISCORD_CATEGORY_ROOMS' as u64")
                .expect("Failed to parse 'DISCORD_CATEGORY_ROOMS' as u64"),
            discord_guild: secret_store.get("DISCORD_GUILD")
                .context("Failed to get 'DISCORD_GUILD' from the secret store")
                .expect("Failed to get 'DISCORD_GUILD' from the secret store")
                .parse()
                .context("Failed to parse 'DISCORD_GUILD' as u64")
                .expect("Failed to parse 'DISCORD_GUILD' as u64"),
        }
    }
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
