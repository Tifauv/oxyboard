FROM rust:1.54 as builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev

# First, create an empty project to build all dependencies
RUN USER=root cargo new --bin oxyboard
WORKDIR ./oxyboard
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --target x86_64-unknown-linux-musl --release

# Then, add our sources and build them
RUN rm src/*.rs
ADD ./src ./src
RUN rm ./target/x86_64-unknown-linux-musl/release/deps/oxyboard*
RUN cargo build --target x86_64-unknown-linux-musl --release


FROM alpine
MAINTAINER Olivier Serve <tifauv@gmail.com>
LABEL org.opencontainers.image.authors="tifauv.gmail.com"

ENV APP_HOME=/app \
    APP_USER=oxyboard \
	APP_GROUP=oxyboard

RUN addgroup --gid 1042 ${APP_GROUP} && \
	adduser --disabled-password --uid 1042 --ingroup ${APP_GROUP} --gecos 'Oxyboard service account' --home "${APP_HOME}" ${APP_USER} && \
	mkdir -pv "${APP_HOME}/bin" && \
	mkdir -pv "${APP_HOME}/data" && \
	chown -R ${APP_USER}:${APP_GROUP} "${APP_HOME}"

USER ${APP_USER}:${APP_GROUP}
WORKDIR ${APP_HOME}

COPY --from=builder /oxyboard/target/x86_64-unknown-linux-musl/release/oxyboard ./bin
COPY config    ./config
COPY templates ./templates

# Default port is 8080/tcp
EXPOSE 8080

# Data is stored in a "data" directory
VOLUME ${APP_HOME}/data

STOPSIGNAL SIGINT
ENTRYPOINT ["/app/bin/oxyboard"]
