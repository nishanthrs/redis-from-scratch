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


## Future Features
* [ ] Active expiration
* [ ] Read over [Tokio tutorial](https://tokio.rs/tokio/tutorial) to learn more about concurrent programming in Rust
  * Other resources:
    * [Send and Sync traits](https://stackoverflow.com/questions/59428096/understanding-the-send-trait)
    * [Send and Sync traits: Jon Gjengset](https://www.youtube.com/watch?v=yOezcP-XaIw)
    * [Async/Await](https://www.youtube.com/watch?v=ThjvMReOXYM)
* [ ] Implement other commands:
  * [x] PING
  * [x] ECHO
  * [x] GET
  * [x] SET
    - [ ] EX
    - [ ] EXAT
    - [ ] PXAT
  * [ ] Sorted set commands
* [ ] Add config settings on Redis (type of cache, default expiration, etc.)
* [ ] Implement hashmap as LRU and LFU cache for smart eviction
* [ ] Store data in hashmap as vector of bytes
* [ ] Write unit tests
* [ ] Write/run load-testing workloads
* [ ] Support multiple clients (data structure per client)

## Other Resources
* [Redis Rust clone](https://github.com/seppo0010/rsedis/blob/e50029606295cb9e0980c04a09f5bf888afa309a/util/src/lib.rs#L171)
* [Build your Own Redis Challenge: Coding Challenges](https://codingchallenges.fyi/challenges/challenge-redis/)
* [Build your Own Redis Challenge: Build your Own](https://build-your-own.org/redis/)
