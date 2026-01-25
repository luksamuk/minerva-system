## Compilation and preparation

# Base container for compilation
FROM rust:1.93.0-alpine3.23 AS rust_base
WORKDIR /
RUN apk add --no-cache cmake make musl-dev file build-base libpq libpq-dev openssl-dev postgresql-dev git
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true
RUN cargo install cargo-binstall
RUN cargo binstall cargo-chef
ENV RUSTFLAGS="-C target-feature=-crt-static"

# Cargo Chef recipe generation
FROM rust_base AS chef
RUN mkdir /minerva
WORKDIR /minerva
RUN cargo new --lib minerva-broker \
    && cargo new --lib minerva-cache \
    && cargo new --lib minerva-data \
    && cargo new --bin minerva-dispatch \
    && cargo new --bin minerva-rest \
    && cargo new --lib minerva-rpc \
    && cargo new --bin minerva-runonce \
    && cargo new --bin minerva-session \
    && cargo new --bin minerva-user
COPY ./Cargo.toml ./Cargo.toml
COPY ./minerva-broker/Cargo.toml ./minerva-broker/Cargo.toml
COPY ./minerva-cache/Cargo.toml ./minerva-cache/Cargo.toml
COPY ./minerva-data/Cargo.toml ./minerva-data/Cargo.toml
COPY ./minerva-dispatch/Cargo.toml ./minerva-dispatch/Cargo.toml
COPY ./minerva-rest/Cargo.toml ./minerva-rest/Cargo.toml
COPY ./minerva-rpc/Cargo.toml ./minerva-rpc/Cargo.toml
COPY ./minerva-runonce/Cargo.toml ./minerva-runonce/Cargo.toml
COPY ./minerva-session/Cargo.toml ./minerva-session/Cargo.toml
COPY ./minerva-user/Cargo.toml ./minerva-user/Cargo.toml
RUN cargo chef prepare --recipe-path recipe.json

# Cook cache
FROM rust_base AS cook
RUN mkdir /minerva
WORKDIR /minerva
COPY --from=chef /minerva/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Build all modules
FROM cook AS builder
COPY . .
RUN apk add --no-cache protobuf protobuf-dev
RUN cargo build --release

# Base container for deploying services
FROM alpine:3.23 AS deploy
ARG APP=/usr/src/app
RUN apk add --no-cache ca-certificates tzdata libpq
COPY --from=builder /usr/lib/libgcc_s.so.1 /usr/lib/
COPY --from=builder /usr/lib/libgcc_s.so /usr/lib/
COPY --from=builder /usr/lib/libstdc++.so.6.0.29 /usr/lib/
COPY --from=builder /usr/lib/libstdc++.so.6 /usr/lib/
COPY --from=builder /usr/lib/libstdc++.so /usr/lib/
ENV TZ=Etc/UTC \
    APP_USER=appuser
RUN addgroup -g 1000 $APP_USER \
    && mkdir -p $APP \
    && mkdir -p $APP/log \
    && adduser -u 1000 -G $APP_USER -h $APP -D $APP_USER
RUN chown -R $APP_USER ${APP}
USER $APP_USER
ENV USER_SERVICE_PORT=9010
ENV SESSION_SERVICE_PORT=9011
ENV PRODUCT_SERVICE_PORT=9012
ENV STOCK_SERVICE_PORT=9013
ENV REPORT_SERVICE_PORT=9014
ENV USER_SERVICE_SERVER=user
ENV SESSION_SERVICE_SERVER=session
ENV DATABASE_SERVICE_SERVER=postgresql
ENV MONGO_SERVICE_SERVER=mongodb
ENV REDIS_SERVICE_SERVER=redis
WORKDIR ${APP}

## Service containers

# REST
FROM deploy AS minerva_rest
EXPOSE 9000
COPY ./tenancy.toml ./tenancy.toml
COPY ./minerva-rest/Rocket.toml ./Rocket.toml
COPY --from=builder /minerva/target/release/minerva-rest ./minerva-rest
CMD ["./minerva-rest"]


# RUNONCE
FROM deploy AS minerva_runonce
COPY ./tenancy.toml ./tenancy.toml
COPY ./minerva-runonce/migrations ./migrations
COPY --from=builder /minerva/target/release/minerva-runonce ./minerva-runonce
CMD ["./minerva-runonce"]


# SESSION
FROM deploy AS minerva_session
EXPOSE 9011
COPY ./tenancy.toml ./tenancy.toml
COPY --from=builder /minerva/target/release/minerva-session ./minerva-session
CMD ["./minerva-session"]


# USER
FROM deploy AS minerva_user
EXPOSE 9010
COPY ./tenancy.toml ./tenancy.toml
COPY --from=builder /minerva/target/release/minerva-user ./minerva-user
CMD ["./minerva-user"]


# DISPATCH
FROM deploy AS minerva_dispatch
EXPOSE 9010
COPY ./tenancy.toml ./tenancy.toml
COPY --from=builder /minerva/target/release/minerva-dispatch ./minerva-dispatch
CMD ["./minerva-dispatch"]

# Front-End
FROM ubuntu:22.04 AS frontend_builder
RUN apt update \
    && apt install -y curl git unzip xz-utils zip libglu1-mesa
WORKDIR /
RUN git clone https://github.com/flutter/flutter.git -b stable --depth 1
ENV PATH=$PATH:/flutter/bin
RUN flutter config \
    --enable-web \
    --no-enable-android \
    --no-enable-linux-desktop \
    --no-enable-windows-desktop \
    --no-enable-macos-desktop
RUN flutter doctor
RUN flutter precache
RUN mkdir /app
ADD ./minerva_frontend/ /app
WORKDIR /app
RUN flutter pub get
RUN flutter build web

FROM nginx:stable-alpine AS minerva_frontend
ENV TZ=America/Sao_Paulo
COPY --from=frontend_builder /app/build/web /usr/share/nginx/html
