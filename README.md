# discord-ani-schedule
discord-ani-schedule

## Prerequisites
- **Rust**: Required for local builds. Install Rust by following the official instructions here: [Install Rust](https://www.rust-lang.org/tools/install).
- **Docker**: Required for containerized builds. All dependencies are already defined in the Dockerfile.

## Main Lib/Crate Used
- https://github.com/serenity-rs/serenity
- https://github.com/serenity-rs/poise

## Build Instructions

Create a `config.json` file first.
```json
{
  "discord_token": "your_discord_token_here"
}
```

### Local Build with Cargo
Execute the following commands to build and run locally:
```sh
cargo build && \
./target/debug/discord-ani-schedule
```

### Docker - Debug Build
To create and run a debug build using Docker:
```sh
docker build --target build-debug -t discord-ani-schedule:debug . && \
    docker run --init -it --rm -v ./config.json:/config.json discord-ani-schedule:debug
```

### Docker - Production Release
For an optimized production build in a minimal container:
```sh
docker build --target release -t discord-ani-schedule:latest . && \
    docker run --init -it --rm discord-ani-schedule:latest
```
