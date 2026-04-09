FROM rust:1.90-slim-bookworm AS chef
LABEL authors="subbar"

RUN cargo install cargo-chef --locked
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

RUN apt-get update -y && apt-get install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*

RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

RUN cargo binstall cargo-leptos -y

RUN rustup target add wasm32-unknown-unknown

WORKDIR /app

COPY --from=planner /app/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json
RUN cargo chef cook --release --target wasm32-unknown-unknown --recipe-path recipe.json

COPY . .

RUN cargo leptos build --release -vv

FROM debian:bookworm-slim AS runtime

RUN apt-get update -y && apt-get install -y \
    ca-certificates \
    libsqlite3-0 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Salin binary server hasil kompilasi dari tahap builder
COPY --from=builder /app/target/release/fitbek /app/fitbek

# Salin aset frontend (Wasm, CSS, JS, HTML) hasil kompilasi dari tahap builder
COPY --from=builder /app/target/release/fitbek /app/fitbek
COPY --from=builder /app/target/site /app/site

# Konfigurasi Environment Variables wajib untuk Leptos SSR
ENV LEPTOS_SITE_ROOT="site"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8090"
ENV LEPTOS_OUTPUT_NAME="fitbek"

# (Opsional) Set environment aplikasi ke production
ENV APP_ENV="production"

# Ekspos port sesuai dengan LEPTOS_SITE_ADDR
EXPOSE 8090

# Eksekusi binary server saat kontainer berjalan
CMD ["/app/fitbek"]