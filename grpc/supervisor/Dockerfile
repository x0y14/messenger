# https://dev.to/rogertorres/first-steps-with-docker-rust-30oi
FROM rust:1.61 as builder
# for protoc
RUN apt-get update && apt-get -y install cmake

# 使い捨てクレートを作り、キャッシュさせる
RUN USER=root cargo new --bin op
WORKDIR /op

COPY ./grpc/supervisor/Cargo.lock ./Cargo.lock
COPY ./grpc/supervisor/Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

# build prepare
COPY ./grpc/proto /proto
COPY ./grpc/supervisor/build.rs ./build.rs
COPY ./grpc/supervisor/src ./src

RUN rm ./target/release/deps/supervisor*
RUN cargo build --release

#FROM rust:1.61-slim-buster
FROM debian:buster-slim
COPY --from=builder /supervisor/target/release/supervisor .

EXPOSE 50051
CMD ["./supervisor"]
