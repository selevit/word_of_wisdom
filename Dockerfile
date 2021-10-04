FROM rust:1.43 as builder

COPY . .
RUN cargo test
RUN cargo build --release

FROM debian:buster-slim
RUN apt-get update && \
  apt-get install -y dumb-init && \
  apt-get clean && \
  rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/* &&

RUN mkdir -p /data
ENV RESPONSES_FILENAME /data/server_responses.txt
COPY ./server_responses.txt ${RESPONSES_FILENAME}
COPY --from=builder ./target/release/server /usr/local/bin
COPY --from=builder ./target/release/client /usr/local/bin
RUN chmod +x /usr/local/bin/server /usr/local/bin/client

ENV PORT 4444
ENV HOST 0.0.0.0
EXPOSE 4444

# If we run the server with PID=1, then the process will not be terminated on SIGTERM / SIGINT,
# because in this case we will have to declare the signal handler explicitly for the process.
# And this is tricky to interrupt an std::net::TCPListener.incoming() stream,
# because it calls the blocking accept() operation,
# and we have to deal with platform-specific file-descriptor magic
# to interrupt it explicitly from a signal handler.
# That's why we just use dumb-init as the init process,
# so SIGTERM will be handled with the default way for a child process.
ENTRYPOINT ["dumb-init"]
CMD ["server"]
