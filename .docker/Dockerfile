FROM rust:1.63

USER app
WORKDIR /app

EXPOSE 9000

RUN cargo build --release
CMD ["./target/release/click_memer"]