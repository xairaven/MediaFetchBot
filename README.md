<h1 align="center">Media Fetch Bot</h1>

Looking for an easy way to **download TikTok, Instagram or YouTube content?**
I'm proud to present **an open-source Telegram bot** written in 🦀**Rust**🦀 using the **Teloxide**🤖 library.
This bot allows you to effortlessly download content by **simply sending a link**.

![](https://img.shields.io/github/actions/workflow/status/xairaven/MediaFetchBot/release.yml?style=plastic)
![](https://img.shields.io/github/v/release/xairaven/MediaFetchBot?style=plastic)
![](https://img.shields.io/github/commit-activity/m/xairaven/MediaFetchBot?style=plastic)
![](https://img.shields.io/github/license/xairaven/MediaFetchBot?style=plastic)
![](https://img.shields.io/github/issues/xairaven/MediaFetchBot?style=plastic)

---

<h2>What do you need to start:</h2>

- **[TikTok Download API Key](https://rapidapi.com/yi005/api/tiktok-download-without-watermark)**
- **[Instagram Photos Downloader API](https://rapidapi.com/easeapi-easeapi-default/api/instagram-photos-downloader-api)**
- **[Instagram Reels Downloader API](https://rapidapi.com/easeapi-easeapi-default/api/instagram-reels-downloader-api)**
- **[Instagram Story Downloader API](https://rapidapi.com/easeapi-easeapi-default/api/instagram-story-downloader-api)**
- **[YouTube Video & MP3 Downloader API](https://rapidapi.com/easeapi-easeapi-default/api/youtube-video-mp3-downloader-api)**
- **[Telegram Bot Token](https://t.me/BotFather)**
- **Windows 10 x64 or Ubuntu x64** *<optional>*
- **Docker** *<optional>*

The bot relies exclusively on free APIs, meaning they might stop functioning at any moment.

You can create your own module to integrate with another API - refer to the `api` module for implementation details 
and the `config` module for integration. 
API data, after processing, is passed through the `Response` structure.

<h2>Quick Start</h2>

Firstly, create a file named `.env` with this structure:

```.env
BOT_TOKEN=...
BOT_NAME=your_telegram_bot_username
LOG_LEVEL=...
WHITELIST=ON/OFF

# Optional
LOG_FORMAT="..."

# Optional
TIKTOK_API_KEY=...

# Optional
INSTAGRAM_PHOTOS_API_KEY=...
INSTAGRAM_REELS_API_KEY=...
INSTAGRAM_STORIES_API_KEY=...

# Optional
YOUTUBE_API_KEY=...
```

Place it in the location specified in the instruction for the preferred method.

### Log Level:
If you want to change the log level (default is `ERROR`), set the LOG_LEVEL variable to the desired level (`error`, `warn`, `info`, `debug`, `trace`, `off`).

### Whitelist:
If you want to enable the whitelist, change the value of `WHITELIST` to `ON` instead of `OFF`.
You also need to create a file named `whitelist.json` next to the `.env` file and fill it in the following format:

```json
[
  123123,   // Some id
  -1231231  // Another id for example
]
```

### Log Format:
You can change the log format if desired.

Parameters:
- `%MESSAGE` - The message.
- `%LEVEL` - The verbosity level of the message.
- `%TARGET` - The name of the target of the directive.
- `%Y` - Year.
- `%m` - Month.
- `%D` - Day.
- `%H` - Hours.
- `%M` - Minutes.
- `%S` - Seconds.

For example, the default format is `[%Y-%m-%D %H-%M-%S %LEVEL %TARGET] %MESSAGE`.

Usage example: `[2024-11-02 00-03-24 INFO media_fetch_bot] Started bot`.

---

Then clone the repo if you want to use method 2 or 3:

```
git clone https://github.com/xairaven/MediaFetchBot.git
```

<h3>Method 1. Manual Start</h3>

Download the archive from the [latest release](github.com/xairaven/MediaFetchBot/releases/latest) corresponding to your OS.

In the archives, there is only one file — the executable. Place `.env` in the same folder.

<h3>Method 2. Docker</h3>

Place `.env` in `<repo_folder>/media_fetch_bot/`.

Change `bot_username` in Dockerfile:

```Dockerfile
# Your app name (write same name to both)
ARG APP_NAME_ARG=...
ENV APP_NAME_ENV=...same
```

Then, in the repo folder:

```sh
cd media_fetch_bot
docker build -t <come-up-with-image-name> .
docker run --name <come-up-with-container-name> <your-image-name>
```

<h3>Method 3. Compile by yourself</h3>

For this, you'll need `rust` compiler and `cargo`.
You can find the installation instructions [`here`](https://doc.rust-lang.org/cargo/getting-started/installation.html).

Place `.env` in `<repo_folder>/media_fetch_bot/`.

After that, when you open the folder:

```sh
cd media_fetch_bot
cargo run
```

Congrats!🥳 The bot is working now.

<h2>Available Inputs:</h2>

- `/help` — Instruction. You can also change localization text. For that, change the text in `media_fetch_bot/locales/en.json`.
- TikTok, Instagram or YouTube Link — Get your content! Enjoy 🎉

<h2>Contributions</h2>

**Contributions are welcome!** 🎉

It's better to create an issue with description of your **bug/feature** before creating pull requests.

<h3>About branching</h3>

This project uses **TBD git strategy**.

Each contributor should have a branch. **Naming example:** ```feat/<nickname>```