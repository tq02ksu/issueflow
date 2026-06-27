FROM rust:1-slim AS build

RUN apt-get update && apt-get install -y --no-install-recommends \
    nodejs npm \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

COPY web/package.json web/package-lock.json ./web/
RUN cd web && npm ci

COPY web ./web
RUN cd web && npm run build

RUN cargo build --release --locked && \
    cp target/release/issueflow /usr/local/bin/issueflow

FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=build /usr/local/bin/issueflow /usr/local/bin/issueflow
COPY --from=build /app/web/dist /app/web/dist
COPY internal/pages/templates /app/internal/pages/templates

EXPOSE 8080

ENTRYPOINT ["/usr/local/bin/issueflow"]
