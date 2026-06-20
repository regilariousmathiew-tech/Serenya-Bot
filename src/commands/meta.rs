use crate::utils::{Context, Error};

/// Show bot latency.
#[poise::command(slash_command, prefix_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let latency = ctx.ping().await;
    let response = format!("🏓 Pong! Latency: {latency:.0?}");
    ctx.say(response).await?;
    Ok(())
}

/// Show bot information.
#[poise::command(slash_command, prefix_command)]
pub async fn about(ctx: Context<'_>) -> Result<(), Error> {
    let config = ctx.data().config();

    let embed = poise::serenity_prelude::CreateEmbed::new()
        .title(format!("🤖 About {}", config.bot.display_name))
        .description("Serenya là một bot nhạc Discord chất lượng cao, mang lại trải nghiệm âm thanh mượt mà và giao diện tương tác trực quan nhất.")
        .field("Người Tạo", "💙 **Herzchen**", true)
        .field("GitHub Repository", "[🔗 Herzchens/Serenya-Bot](https://github.com/Herzchens/Serenya-Bot)", true)
        .field(
            "Khả Năng & Tính Năng",
            "• Phát nhạc cực nhanh từ **YouTube** và **Spotify**\n\
             • Hỗ trợ quản lý hàng chờ nâng cao (chuyển bài, tua nhanh, lặp bài)\n\
             • Quản lý playlist cá nhân và đồng bộ dữ liệu thông minh\n\
             • Tìm kiếm lời bài hát trực tiếp trên Discord",
            false,
        )
        .color(0x5865F2);

    let reply = poise::CreateReply::default().embed(embed);
    ctx.send(reply).await?;
    Ok(())
}

/// Show help menu for commands.
#[poise::command(slash_command, prefix_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help for"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            extra_text_at_bottom: "Type /help <command> for more details.",
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}

/// Reload the configuration and clear all caches.
#[allow(deprecated)]
#[poise::command(slash_command, prefix_command)]
pub async fn reload(ctx: Context<'_>) -> Result<(), Error> {
    let owner_id = {
        let config =
            ctx.data().config.read().map_err(|_| {
                crate::utils::SerenyaError::Config("config lock is poisoned".into())
            })?;
        config.bot.owner
    };

    let author_id = ctx.author().id.get();
    let mut is_allowed = author_id == owner_id;

    if !is_allowed {
        if let Some(guild_id) = ctx.guild_id() {
            if let Ok(member) = guild_id.member(ctx, ctx.author().id).await {
                if let Ok(permissions) = member.permissions(ctx) {
                    if permissions.administrator() {
                        is_allowed = true;
                    }
                }
            }
        }
    }

    if !is_allowed {
        return Err(crate::utils::SerenyaError::Permission(
            "This command is restricted to the bot owner or server administrators.".into(),
        )
        .into());
    }

    ctx.defer().await?;

    let new_config = std::sync::Arc::new(crate::config::load_config("config.yml")?);
    let old_prefix = {
        let current =
            ctx.data().config.read().map_err(|_| {
                crate::utils::SerenyaError::Config("config lock is poisoned".into())
            })?;
        current.bot.prefix.clone()
    };
    let new_prefix = new_config.bot.prefix.clone();
    tracing::info!(
        "config_reload old_prefix={} new_prefix={}",
        old_prefix,
        new_prefix
    );

    // Register secrets for redaction on reload
    crate::logging::register_secret_to_redact(&new_config.bot.token);
    if let Some(ref secret) = new_config.spotify.client_secret {
        crate::logging::register_secret_to_redact(secret);
    }
    if let Some(ref url) = new_config.logging.webhook_url {
        crate::logging::register_secret_to_redact(url);
    }
    if let Some(ref url) = new_config.bot.log_webhook_url {
        crate::logging::register_secret_to_redact(url);
    }

    let resolver_config = new_config.resolver.clone();
    let token_changed = {
        let current =
            ctx.data().config.read().map_err(|_| {
                crate::utils::SerenyaError::Config("config lock is poisoned".into())
            })?;
        current.bot.token != new_config.bot.token
    };

    {
        let mut config_write =
            ctx.data().config.write().map_err(|_| {
                crate::utils::SerenyaError::Config("config lock is poisoned".into())
            })?;
        *config_write = new_config.clone();
    }

    crate::audio::runtime::configure(&resolver_config, &new_config.spotify);
    let (metadata_len, stream_len) = crate::audio::source::clear_caches();

    let restart_note = if token_changed {
        "Changed bot.token value requires a process restart."
    } else {
        "No restart-only bot fields changed (prefix updated dynamically)."
    };

    let embed = poise::serenity_prelude::CreateEmbed::new()
        .title("🔄 System Reload Complete")
        .description("`config.yml` has been hot reloaded and resolver settings were applied.")
        .field("Config File", "`config.yml` reloaded", true)
        .field("Resolver Runtime", "Limits and timeouts applied", true)
        .field(
            "Metadata Cache",
            format!("Cleared **{}** entries", metadata_len),
            true,
        )
        .field(
            "Stream URL Cache",
            format!("Cleared **{}** entries", stream_len),
            true,
        )
        .field("Restart Note", restart_note, false)
        .color(0x3498DB);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}
