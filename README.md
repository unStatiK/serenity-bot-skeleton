# serenity-bot-skeleton
Bot template with serenity rust library

# Commands

* ping - pong reply
* hello - hello with nickname reply
* uptime - get bot start time

# Build

RUSTFLAGS="-C target-cpu=native" CARGO_HOME="<path_to_cargo_dir>" RUSTUP_HOME="<path_to_rustup_dir>" cargo.exe build --release

# Run

DISCORD_TOKEN=<discord_bot_token> ./target/release/bot.exe
