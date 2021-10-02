# Exersice

Design and implement “Word of Wisdom” tcp server.

- TCP server should be protected from DDOS attacks with the [Prof of Work](https://en.wikipedia.org/wiki/Proof_of_work), the challenge-response protocol should be used.
- The choice of the POW algorithm should be explained.
- After Prof Of Work verification, the server should send one of the quotes from “Word of wisdom” book or any other collection of the quotes.
- Docker file should be provided both for the server and for the client that solves the POW challenge


## How to run

You can easily try how it works with docker:

```bash
docker-compose up -d
docker-compose run client
```

Also, if you pass `RUST_LOG=debug` to the client, you will be able to see the hashes it tries to solve the puzzle:

```
RUST_LOG=debug docker-compose run client
```
