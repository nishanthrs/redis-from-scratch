[![progress-banner](https://backend.codecrafters.io/progress/redis/ee724e62-6778-4d2f-9202-b29048c893b6)](https://app.codecrafters.io/users/nishanthrs?r=2qF)

This is a starting point for Rust solutions to the
["Build Your Own Redis" Challenge](https://codecrafters.io/challenges/redis).

In this challenge, you'll build a toy Redis clone that's capable of handling
basic commands like `PING`, `SET` and `GET`. Along the way we'll learn about
event loops, the Redis protocol and more.

**Note**: If you're viewing this repo on GitHub, head over to
[codecrafters.io](https://codecrafters.io) to try the challenge.

# Running the Server

The entry point for your Redis implementation is in `src/main.rs`. Study and
uncomment the relevant code, and push your changes to pass the first stage:

Run `./spawn_redis_server.sh` to run your Redis server, which is implemented
in `src/main.rs`. This command compiles your Rust project, so it might be
slow the first time you run it. Subsequent runs will be fast.
Push to origin to test changes: `git push origin master`.
