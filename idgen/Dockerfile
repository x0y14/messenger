FROM rust:latest

COPY ./idgen /tmp/idgen
WORKDIR /tmp/idgen
RUN cargo build --release

EXPOSE 3030
CMD ["./target/release/idgen"]