# Exersice

![Continuous integration](https://github.com/selevit/word_of_wisdom/workflows/Continuous%20integration/badge.svg)

Design and implement “Word of Wisdom” tcp server.

- TCP server should be protected from DDOS attacks with the [Prof of Work](https://en.wikipedia.org/wiki/Proof_of_work), the challenge-response protocol should be used.
- The choice of the POW algorithm should be explained.
- After Prof Of Work verification, the server should send one of the quotes from “Word of wisdom” book or any other collection of the quotes.
- Docker file should be provided both for the server and for the client that solves the POW challenge


## How to run

```bash
docker-compose up -d
docker-compose run client
```

The server uses the [Hashcash](https://en.wikipedia.org/wiki/Hashcash) proof of work algorithm with default complexity equals to 4. When running a server, you can set custom puzzle complexity:

```bash
PUZZLE_COMPLEXITY=5 docker-compose up -d
```

If you pass `RUST_LOG=debug` to the client, you will be able to see the hashes it tries to solve the puzzle. But it can slow down the solving process.

```
RUST_LOG=debug docker-compose run client
```

## Why Hashcash?

It uses a commonly known `sha256` hash function, which makes the client implementation very simple. Also this is the most known proof-of-work algorithm, which is used in Bitcoin's mining.
