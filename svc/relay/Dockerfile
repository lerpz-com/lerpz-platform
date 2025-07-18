FROM rust:slim-bookworm AS chef

RUN apt-get update && apt-get upgrade -y
RUN apt-get install -y --no-install-recommends \
    build-essential pkg-config libssl-dev

RUN --mount=type=cache,target=/root/.cargo/registry \
    cargo install cargo-chef


FROM chef AS planner
WORKDIR /chef

COPY . .

RUN cargo chef prepare --recipe-path recipe.json


FROM chef AS builder
WORKDIR /build

COPY --from=planner /chef/recipe.json ./recipe.json
RUN --mount=type=cache,target=/build/target \
    cargo chef cook --release --recipe-path recipe.json

COPY . .

RUN cargo build --release --bin lerpz-relay


FROM debian:bookworm-slim AS runtime
WORKDIR /var/app

RUN groupadd -r server && \
    useradd -r -g server -s /usr/sbin/nologin -c "The docker user" docker && \
    chown -R docker:server /var/app

COPY --chown=docker:server --from=builder /build/target/release/lerpz-relay ./

USER docker

EXPOSE 3002

ENTRYPOINT ["/var/app/lerpz-relay"]
