use crate::utils::{Context, Error};

/// Get the bot's invite link.
#[poise::command(slash_command, prefix_command)]
pub async fn invite(ctx: Context<'_>) -> Result<(), Error> {
    let invite_link = if let Some(url) = &ctx.data().config().bot.invite_url {
        url.clone()
    } else {
        let client_id = ctx.cache().current_user().id.get();
        format!(
            "https://discord.com/api/oauth2/authorize?client_id={}&permissions=8&scope=bot%20applications.commands",
            client_id
        )
    };

    ctx.say(format!("🔗 **Invite me to your server:**\n<{invite_link}>"))
        .await?;
    Ok(())
}

/// Get the support server link.
#[poise::command(slash_command, prefix_command)]
pub async fn support(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Cảm ơn bạn đã ủng hộ nhưng chúng tôi không có server riêng 💙")
        .await?;
    Ok(())
}
