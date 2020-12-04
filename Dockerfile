FROM --platform=$BUILDPLATFORM rust:1.40 as builder
WORKDIR /usr/src/scaap
COPY . .
RUN cargo install --path .

FROM --platform=$BUILDPLATFORM debian:buster-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/scaap /usr/local/bin/scaap
CMD ["scaap"]