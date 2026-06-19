use crate::audio::{TrackEndHandler, resolve_input};
use crate::core::{GuildPlayer, PlaybackStatus};
use crate::utils::{Context, Error, SerenyaError};
use songbird::input::YoutubeDl;

/// Play a song or playlist.
#[poise::command(slash_command, prefix_command)]
pub async fn play(
    ctx: Context<'_>,
    #[description = "Search query, URL, or playlist name"] query: String,
) -> Result<(), Error> {
    let guild_id = ctx
        .guild_id()
        .ok_or_else(|| SerenyaError::Config("This command can only be used in a server.".into()))?;

    // 1. Check if user is in a voice channel
    let user_channel_id = {
        let guild = ctx
            .guild()
            .ok_or_else(|| SerenyaError::NotFound("Guild not found".into()))?;
        guild
            .voice_states
            .get(&ctx.author().id)
            .and_then(|state| state.channel_id)
            .ok_or_else(|| {
                SerenyaError::Voice("You must be in a voice channel to use this command.".into())
            })?
    };

    let manager = songbird::get(ctx.serenity_context())
        .await
        .ok_or_else(|| SerenyaError::Voice("Songbird manager not initialized.".into()))?
        .clone();

    // 2. Auto-join voice channel if not already connected
    let call_lock: std::sync::Arc<tokio::sync::Mutex<songbird::Call>> =
        if let Some(call) = manager.get(guild_id) {
            call
        } else {
            manager
                .join(guild_id, user_channel_id)
                .await
                .map_err(|e| SerenyaError::Voice(format!("Failed to join voice channel: {}", e)))?
        };

    // 3. Defer response because metadata lookup can take time
    ctx.defer().await?;

    // 4. Resolve input tracks
    let user_id = ctx.author().id.get();
    let mut tracks = resolve_input(
        &query,
        user_id,
        &ctx.data().database,
        &ctx.data().http_client,
    )
    .await?;

    if tracks.is_empty() {
        ctx.say("No tracks found for the query.").await?;
        return Ok(());
    }

    // 5. Get/Create guild player
    let player_lock = ctx
        .data()
        .guild_players
        .entry(guild_id)
        .or_insert_with(|| std::sync::Arc::new(tokio::sync::RwLock::new(GuildPlayer::new())))
        .clone();

    let mut player = player_lock.write().await;
    player.voice_channel = Some(user_channel_id);
    player.announce_channel = Some(ctx.channel_id());

    // 6. Add to queue or play immediately
    let config = &ctx.data().config;
    let max_queue_size = config.playback.max_queue_size;

    if player.playback_status == PlaybackStatus::Idle && player.now_playing.is_none() {
        // Play first track immediately
        let mut first_track = tracks.remove(0);
        first_track.requester_name = ctx.author().name.clone();

        player.now_playing = Some(first_track.clone());
        player.playback_status = PlaybackStatus::Playing;

        let mut call = call_lock.lock().await;
        let source: songbird::input::Input =
            YoutubeDl::new(ctx.data().http_client.clone(), first_track.url.clone()).into();
        let handle = call.play_input(source);

        // Register event handler
        let _ = handle.add_event(
            songbird::Event::Track(songbird::TrackEvent::End),
            TrackEndHandler {
                guild_id,
                database: ctx.data().database.clone(),
                guild_players: ctx.data().guild_players.clone(),
                http_client: ctx.data().http_client.clone(),
                serenity_ctx: ctx.serenity_context().clone(),
            },
        );

        player.current_track_handle = Some(handle);

        // Queue remaining tracks (if any)
        let added = player.queue.push_batch(tracks, max_queue_size)?;

        if added > 0 {
            ctx.say(format!(
                "🎶 **Now Playing:** {}\nEnqueued {} other tracks.",
                first_track.title, added
            ))
            .await?;
        } else {
            ctx.say(format!("🎶 **Now Playing:** {}", first_track.title))
                .await?;
        }
    } else {
        // Enqueue all tracks
        let track_count = tracks.len();
        let first_title = tracks.first().map(|t| t.title.clone()).unwrap_or_default();

        // Populate requester names
        for t in &mut tracks {
            t.requester_name = ctx.author().name.clone();
        }

        let added = player.queue.push_batch(tracks, max_queue_size)?;

        if added == 0 {
            ctx.say("Queue is full! Could not add any tracks.").await?;
        } else if added == 1 && track_count == 1 {
            ctx.say(format!("📝 **Enqueued:** {}", first_title)).await?;
        } else {
            ctx.say(format!("📝 **Enqueued {} tracks**", added)).await?;
        }
    }

    Ok(())
}
