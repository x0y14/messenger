FROM rust:latest

COPY . .

RUN cargo build --release

EXPOSE 3030

CMD ["./target/release/idgen"]