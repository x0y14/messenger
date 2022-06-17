FROM rust:latest

RUN apt-get update && apt-get -y install cmake

COPY ./grpc /tmp/grpc
WORKDIR /tmp/grpc
RUN cargo build

EXPOSE 50050
CMD ["./target/debug/talk"]