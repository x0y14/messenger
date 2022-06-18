FROM rust:latest

COPY ./idgen /tmp/idgen
WORKDIR /tmp/idgen
RUN cargo build --release

CMD ["./target/release/idgen"]