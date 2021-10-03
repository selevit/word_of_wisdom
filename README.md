# Exersice

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

Also, if you pass `RUST_LOG=debug` to the client, you will be able to see the hashes it tries to solve the puzzle:

```
RUST_LOG=debug docker-compose run client
```

The server uses the [Hashcash](https://en.wikipedia.org/wiki/Hashcash) proof of work algorithm with default complexity equals to 4. Also, when running a server, you can set custom puzzle complexity.

```bash
PUZZLE_COMPLEXITY=6 docker-compose up -d
```

## Why Hashcash?

It uses a commonly known `sha256` hash function, which makes the client implementation very simple. Also this is the most known proof-of-work algorithm, as it's used in Bitcoin mining.
