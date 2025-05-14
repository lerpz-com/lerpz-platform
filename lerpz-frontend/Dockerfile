FROM rust:slim-bookworm AS chef

RUN apt-get update && apt-get upgrade -y
RUN apt-get install -y --no-install-recommends \
    build-essential npm

RUN npm install -g pnpm
RUN --mount=type=cache,target=/root/.cargo/registry \
    cargo install cargo-chef cargo-leptos


FROM chef AS planner
WORKDIR /chef

COPY . .
RUN cargo chef prepare --recipe-path recipe.json


FROM chef AS builder
WORKDIR /build

COPY --from=planner /chef/recipe.json ./recipe.json
RUN --mount=type=cache,target=/build/target \
    cargo chef cook --release --recipe-path recipe.json

RUN --mount=type=bind,source=package.json,target=package.json \
    --mount=type=bind,source=pnpm-lock.yaml,target=pnpm-lock.yaml \
    --mount=type=cache,target=/root/.pnpm-store \
    pnpm install --frozen-lockfile

COPY . .

RUN cargo leptos build --release


FROM debian:bookworm-slim AS runtime
WORKDIR /var/app

RUN groupadd -r server && \
    useradd -r -g server -s /usr/sbin/nologin -c "The docker user" docker && \
    chown -R docker:server /var/app

COPY --chown=docker:server --from=builder /build/target/site ./site
COPY --chown=docker:server --from=builder /build/target/release/lerpz-frontend ./

USER docker

EXPOSE 3000

ENTRYPOINT ["/var/app/lerpz-frontend"]
