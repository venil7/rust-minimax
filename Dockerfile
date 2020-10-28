FROM rust:1.47

WORKDIR /usr/darkruby/tictactoe
COPY . .

RUN cargo build --release --bin server

CMD ["./target/release/server"]