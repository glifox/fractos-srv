FROM rust:1.95-alpine AS builder

RUN apk add --no-cache musl-dev pkgconfig openssl-dev

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
# Creamos un dummy main para pre-compilar dependencias
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release --target x86_64-unknown-linux-musl

COPY . .
RUN touch src/main.rs && cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:3.20

WORKDIR /app

# Copiamos el binario desde la etapa de builder
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/app /usr/local/bin/app

ENV USER_NAME=admin
ENV USER_PASW=password
ENV PORT=3030
ENV DIR=/data
ENV RUST_LOG=warn

EXPOSE $PORT

CMD ["app"]
