# Hotel Bot

Bot for the server version of an offtopic hangout area for the Rust Programming Language Community Server.

## Features of the bot

- `/room_create`: Creates a new room for a guest.

- `/room_key_create`: Creates a new key for an existing room.

- `/room_name_update`: Updates the name of an existing room.

- `/room_open`: Opens a room's door, allowing everyone to view and connect. It sets the necessary permissions to enable viewing and connecting for the everyone role.

- `/room_close`: Closes a room's door, denying everyone from viewing and connecting. It sets the necessary permissions to disable viewing and connecting for the everyone role.

## Future Features

1. All commands will be organized into groups
2. Timer to purge all invite links after 7 days
3. Room key revoking command
4. Room auto-renaming prevention

## Invite Link

This contains the bot's necessary permissions.

https://discord.com/oauth2/authorize?client_id=<bot_id>&scope=bot&permissions=532844768464

## How To Run

Running the bot locally requires shuttle-rs. Make sure it is installed.

Additionally, running the bot locally requires a `Secrets.dev.toml` file. A template for it
is provided in the repository.

```commandline
cargo shuttle run
```
