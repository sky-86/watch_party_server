FROM rustlang/rust:nightly

ARG PORT
ARG ADDRESS

#ENV ADDRESS=0.0.0.0
#ENV ADDRESS=127.0.0.1
#ENV PORT=42069

WORKDIR /app
COPY . .

RUN cargo build --release

CMD PORT=$PORT ADDRESS=$ADDRESS ./target/release/server
