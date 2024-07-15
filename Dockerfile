FROM rust AS build_host
WORKDIR /src

RUN USER=root cargo new --bin metrics
WORKDIR /src/metrics

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs
RUN rm ./target/release/deps/metrics*

COPY ./src ./src
RUN cargo build --release

WORKDIR /src

FROM rust:slim

RUN apt-get update
RUN apt-get install -y libpq-dev

WORKDIR /src

COPY --from=build_host /src/metrics/target/release/metrics ./metrics

CMD ["./metrics"]
