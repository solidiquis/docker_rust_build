FROM rust:latest AS chef
WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    mold clang lld pkg-config ca-certificates \
 && rm -rf /var/lib/apt/lists/*

RUN cargo install cargo-chef --locked

# Tell rust/cargo to use clang + mold
ENV CC=clang \
    CXX=clang++ \
    RUSTFLAGS="-C link-arg=-fuse-ld=mold"

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .

ARG BIN=csv
RUN cargo build --release --bin ${BIN}

FROM debian:bookworm-slim AS runtime
ARG BIN=csv
COPY --from=builder /app/target/release/${BIN} /usr/local/bin/app
ENTRYPOINT ["/usr/local/bin/app"]
