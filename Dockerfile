# THIS Dockerfile represents the default, full-featured Devisa API
# containerized deployment procedure, producing a core REST
# API powered by Actix-web in Rust. This API is supplemented by a
# host of other libraries and services, including a real-time
# websocket-based streaming data feed produced by Warp.
#
# Alternatively, experimental approaches (strictly NOT for the purpose
# of fully carrying the responsibilities of the full core Actix-powered
# API, but instead for experimental and research-driven ventures) can
# be used with Docker, by looking for the service type's corresponding
# Dockerfile under scripts/dockerfiles/ (for example,
# ./scripts/dockerfiles/Dockerfile.warp for the Warp-backed
# experimental web service.)
#
# DEVISA LLC. 2021

FROM docker.io/rust:latest AS builder

RUN update-ca-certificates

ENV USER=root
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    # --uid  \
    "di-api"

WORKDIR /di-api

COPY ./ ./

RUN cargo build --release --package di-api

################3333

FROM docker.io/debian:buster-slim

RUN apt-get update -y
RUN apt-get install libssl-dev -y

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /di-api

# Copy our build
COPY --from=builder /di-api/target/release/di-api ./

# Use an unprivileged user.
USER di-api:di-api

CMD ["/di-api/di-api"]
