ARG BASE_IMAGE=lukemathwalker/cargo-chef:0.1.62-rust-alpine3.18

FROM $BASE_IMAGE as chef
WORKDIR /app
RUN apk update && apk add lld clang pkgconfig libressl-dev

FROM chef as planner
COPY . .
# Compute a lock-like file for our project
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
# Build our project dependencies, not our application!
RUN cargo chef cook --release --recipe-path recipe.json
# Up to this point, if our dependency tree stays the same,
# all layers should be cached.
COPY . .
# Build our project
RUN cargo build --release --bin insta

FROM alpine:latest
ENV BASE_DIR=/
RUN mkdir /files

WORKDIR /app

# Install Vector
RUN apk add --no-cache curl bash && \
    curl --proto '=https' --tlsv1.2 -sSfL https://sh.vector.dev | bash -s -- -y --prefix /usr/local
COPY vector.toml .
COPY config ./config
RUN mkdir /var/lib/vector && mkdir logs

COPY --from=builder /app/target/release/insta .

CMD vector --config vector.toml | ./insta