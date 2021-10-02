FROM rust:1.43 as builder

COPY . .
RUN cargo clippy
RUN cargo test
RUN cargo build --release

FROM debian:buster-slim
RUN mkdir -p /data
ENV RESPONSES_FILENAME /data/server_responses.txt
COPY ./server_responses.txt ${RESPONSES_FILENAME}
COPY --from=builder ./target/release/server /usr/local/bin
COPY --from=builder ./target/release/client /usr/local/bin
RUN chmod +x /usr/local/bin/server /usr/local/bin/client

ENV PORT 4444
ENV HOST 0.0.0.0
EXPOSE 4444

CMD ["server"]
