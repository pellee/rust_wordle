FROM rust:1.81.0

WORKDIR /app

COPY src/ /app/src/
COPY Cargo.toml /app 

RUN cargo build --release

EXPOSE 8080

CMD [ "cargo", "run", "--release"]
