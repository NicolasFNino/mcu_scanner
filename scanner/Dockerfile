FROM rust:latest

COPY ./ ./

RUN cargo build --release --bin scanner

RUN mv /target/release/scanner /run_scanner

CMD [ "./run_scanner" ]