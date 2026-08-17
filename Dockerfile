FROM rust:slim

WORKDIR /apps

COPY . .

RUN cargo build -p api --release

EXPOSE 3030

CMD ["./target/release/api"]