FROM rust:1.40 as builder
WORKDIR /usr/src/scaap
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
WORKDIR /etc/scaap-app
COPY --from=builder /usr/local/cargo/bin/scaap .
COPY --from=builder /usr/src/scaap/start.sh .
WORKDIR /etc/scaap
RUN chmod +x /etc/scaap-app/start.sh
CMD ["/etc/scaap-app/start.sh"]