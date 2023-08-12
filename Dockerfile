FROM rust:1.69 as builder
WORKDIR /usr/src/indian-flag
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
COPY --from=builder /usr/local/cargo/bin/indian-flag /usr/local/bin/indian-flag
CMD ["indian-flag"]
