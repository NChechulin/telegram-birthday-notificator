# Birthday notificator bot

A telegram bot which sends a Happy Birthday message to specified chats

## Setup

1. Install rust and `cargo`
2. Edit the `config.json`:
   1. `sqlite_db_path` - path to an sqlite `.db` or `.sqlite` file
   2. `telegram_bot_token` can be obtained by writing a message to `@botfather`
   3. `telegram_chat_ids` - `id`s of chats where the bot sends messages. Both personal and group chats are OK. [How to get the chat id](https://sean-bradley.medium.com/get-telegram-chat-id-80b575520659)
3. Create an SQLite database. It should contain a table `Users`:

```SQLite
CREATE TABLE "Users" (
   "name" TEXT NOT NULL UNIQUE,
   "handle" TEXT NOT NULL UNIQUE,
   "birth_date" TEXT,
   PRIMARY KEY("name")
)
```
Example of a record: `Nikolay Chechulin; @nchechulin; 2002-04-18` (handle is telegram username, birthday is in format `yyyy-mm-dd`)

4. Clone the repo, `cd` into the diretory
5. Ensure you have `libssl-dev`/`openssl-devel` and `pkg-config` installed
6. Compile the project: `cargo build --release`
7. Place the executable in the same directory as `config.json`
8. Create a `cron` job which runs the program every 24 hours
9. Enjoy!
