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

WORKDIR /usr/src/app

COPY . .
RUN cargo build --release

# ---

FROM debian:buster-slim
COPY --from=builder /usr/src/app/target/release/api_actix .

EXPOSE 80
CMD ["./api-actix"]

# RUN update-ca-certificates

# ENV USER=myip
# ENV UID=10001

# RUN adduser \
#     --disabled-password \
#     --gecos "" \
#     --home "/nonexistent" \
#     --shell "/sbin/nologin" \
#     --no-create-home \
#     # --uid  \
#     "dvapi"

# WORKDIR /dvapi

# COPY ./.env ./.env
# COPY ./.cargo/ ./.cargo/
# COPY ./Cargo.toml ./Cargo.toml

# COPY ./api-srv ./api-srv
# COPY ./api-srv/Cargo.toml ./api-srv/Cargo.toml

# COPY ./api-data/src ./api-data/src
# COPY ./api-core/Cargo.toml ./api-core/Cargo.toml

# COPY ./api-redis/src ./api-redis/src
# COPY ./api-redis/Cargo.toml ./api-redis/Cargo.toml

# COPY ./api-redis/src ./api-redis/src
# COPY ./api-redis/Cargo.toml ./api-redis/Cargo.toml

# COPY ./api-common/src ./api-common/src
# COPY ./api-common/Cargo.toml ./api-common/Cargo.toml

# COPY ./api-cloud/src ./api-cloud/src
# COPY ./api-cloud/Cargo.toml ./api-cloud/Cargo.toml

# COPY ./api-http/src ./api-http/src
# COPY ./api-http/Cargo.toml ./api-http/Cargo.toml

# COPY ./api-exec/src ./api-exec/src
# COPY ./api-exec/Cargo.toml ./api-exec/Cargo.toml

# COPY ./api-grpc/src ./api-grpc/src
# COPY ./api-grpc/Cargo.toml ./api-grpc/Cargo.toml

# COPY ./api-lang/src ./api-lang/src
# COPY ./api-lang/Cargo.toml ./api-lang/Cargo.toml

# COPY ./api-rt/src ./api-rt/src
# COPY ./api-rt/Cargo.toml ./api-rt/Cargo.toml

# RUN cargo build --release

# FROM docker.io/debian:buster-slim

# RUN apt-get update -y
# RUN apt-get install libssl-dev -y

# # Import from builder.
# COPY --from=builder /etc/passwd /etc/passwd
# COPY --from=builder /etc/group /etc/group

# WORKDIR /dvapi

# # Copy our build
# COPY --from=builder /dvapi/target/release/dvapi ./

# # Use an unprivileged user.
# USER dvapi:dvapi

# CMD ["/dvapi/dvapi"]
