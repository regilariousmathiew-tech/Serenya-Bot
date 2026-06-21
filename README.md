# Serenya Bot

Serenya is a Rust-based, multi-guild Discord music bot designed for high performance, safety, and stability. 
Built using `serenity` and `poise`, this bot aims to be incredibly robust while delivering features suitable for large, bustling servers.

## Features

- **Slash + Prefix Commands**: Support for modern Discord interaction.
- **High-Fidelity Audio Architecture**: Highly optimized audio streaming bypassing memory bottlenecks, with support for Deezer, Apple Music, Spotify, YouTube, and SoundCloud.
- **Audio Filters**: 8D, Bassboost, Nightcore, Pitch, Speed tuning out of the box using FFmpeg.
- **Graceful Shutdown**: Safe teardown sequence via cancellation tokens to stop background tasks and write final database states to disk.
- **Atomic Persistence**: Thread-safe atomic file writes with `.tmp` and `.bak` backup strategies to prevent data corruption.
- **Strict Error Handling**: Custom typed domain errors (`SerenyaError`) propagated safely up to Poise boundaries.
- **Low Memory Footprint**: Tailored for minimal resource environments (runs easily on a 1GB VPS). **Note**: The **only** bottleneck for speed is the local execution of `yt-dlp` to extract direct media streams, which is network and CPU dependent.

## Tech Stack

- **Runtime**: Rust (latest stable), Tokio (async runtime)
- **Discord Integration**: Poise + Serenity
- **Voice / Audio**: Songbird (bypassing Symphonia for raw FFmpeg pipelines)
- **Serialization**: Serde, serde-saphyr (YAML)
- **Logging**: Tracing

## Available Commands

Serenya comes packed with essential and advanced playback tools:

### 🎵 Playback
- `/play [query/url]` - Plays a song or playlist from Spotify, Deezer, Apple Music, YouTube, or SoundCloud.
- `/search [query]` - Shows a dropdown of top search results from various providers to choose from.
- `/pause` - Pauses the current track.
- `/resume` - Resumes a paused track.
- `/stop` - Stops playback and clears the queue.

### 📋 Queue Management
- `/queue` - Displays the current server queue (paginated).
- `/skip` - Skips the currently playing track.
- `/previous` - Plays the previously played track.
- `/jump [position]` - Skips to a specific track in the queue.
- `/remove [position]` - Removes a specific track from the queue.
- `/move [from] [to]` - Moves a track from one position to another.
- `/clear` - Clears all tracks from the queue except the one currently playing.
- `/shuffle` - Shuffles the upcoming tracks in the queue.
- `/loop [mode]` - Changes the loop mode (`Off`, `Track`, `Queue`).

### 🎛️ Audio Effects
- `/8d [on|off]` - Toggles the 8D audio effect.
- `/bassboost [level]` - Toggles or sets the bassboost level.
- `/nightcore [on|off]` - Toggles the nightcore (speed + pitch) effect.
- `/speed [multiplier]` - Adjusts the playback speed.
- `/pitch [multiplier]` - Adjusts the playback pitch.
- `/filter clear` - Clears all active audio filters.

### ℹ️ Information & Utilities
- `/nowplaying` - Displays detailed information about the currently playing track.
- `/lyrics [query]` - Fetches lyrics for the current song or a specific query.
- `/songinfo` - Displays raw metadata for the current track.
- `/ping` - Checks the bot's latency and connection status.

### ⚙️ Settings
- `/announce [on|off]` - Toggles "Now Playing" announcements in the channel.
- `/stay_in_voice [on|off]` - Toggles whether the bot should stay in the voice channel when the queue ends.
- `/default_volume [0-100]` - Sets the default playback volume for the server.

## Getting Started

### Prerequisites

- Rust 1.85+
- Visual Studio Build Tools (on Windows) or GCC/Clang (on Linux)
- CMake
- `ffmpeg` and `yt-dlp` installed and present in the system `PATH`.
  - On Windows, you can optionally place the executables in a `bin/` subdirectory in the project root.
  - On Linux (e.g. VPS Ubuntu), install them globally via package managers (e.g., `apt update && apt install ffmpeg python3 python3-pip -y && pip3 install -U yt-dlp`) and ensure they are accessible.

### Configuration

Create a `config.yml` based on the `config.example.yml` template.
Ensure your `DISCORD_TOKEN` environment variable is set or place it directly into the config file.

Spotify imports use an `sp_dc` Web Player session cookie to request a user-scoped Spotify Web API token. Keep the real cookie out of Git.

### Running the Bot

Run the bot using:

```bash
cargo run --release
```

### Verification & Benchmarking

To run the test suite:

```bash
cargo test
```

A benchmark script `track_perf.ps1` is provided to monitor Serenya's CPU and memory footprint over time to prove its lightweight characteristics. Run it in a PowerShell environment:

```powershell
./track_perf.ps1
```

### Benchmark Results

Serenya has been rigorously benchmarked using a custom Global Allocator Tracker to eliminate memory churn (allocations on the Heap) during intensive track searching and ranking. 

#### Jaro-Winkler Similarity (10,000 iterations):
- **Allocations**: 0 Bytes (Using static Stack arrays)
- **Execution Time**: ~12.20 ms

#### Token Overlap (10,000 iterations):
- **Allocations**: 0 Bytes (Using Zero-allocation Iterators)
- **Execution Time**: ~4.05 ms

The bot can serve thousands of concurrent users with effectively zero memory leaks and highly optimized search ranking.
