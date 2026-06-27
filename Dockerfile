FROM rust:1-slim AS rust-build

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release --locked && \
    cp target/release/issueflow /usr/local/bin/issueflow

FROM node:20-slim AS web-build

WORKDIR /app

COPY web/package.json web/package-lock.json ./
RUN npm ci

COPY web ./
RUN npm run build

FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=rust-build /usr/local/bin/issueflow ./issueflow
COPY --from=web-build /app/dist ./web/dist
COPY internal/pages/templates ./internal/pages/templates

EXPOSE 8080

ENTRYPOINT ["./issueflow"]
