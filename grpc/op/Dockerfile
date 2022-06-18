# https://dev.to/rogertorres/first-steps-with-docker-rust-30oi
FROM rust:1.61 as builder
# for protoc
RUN apt-get update && apt-get -y install cmake

# 使い捨てクレートを作り、キャッシュさせる
RUN USER=root cargo new --bin op
WORKDIR /op

COPY ./grpc/op/Cargo.lock ./Cargo.lock
COPY ./grpc/op/Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

# build prepare
COPY ./grpc/proto /proto
COPY ./grpc/op/build.rs ./build.rs
COPY ./grpc/op/src ./src

RUN rm ./target/release/deps/op*
RUN cargo build --release

#FROM rust:1.61-slim-buster
FROM debian:buster-slim
COPY --from=builder /op/target/release/op .

EXPOSE 50051
CMD ["./op"]
