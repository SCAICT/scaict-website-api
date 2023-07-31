FROM rust:1.70.0-slim-buster AS builder

RUN update-ca-certificates

WORKDIR /scaict-website-api

COPY . .

RUN cargo build --release


FROM gcr.io/distroless/cc

WORKDIR /scaict-website-api

ENV INTEGRATION_SECRET = your_deploy_secret
ENV MEMBER_DATABASE_ID = your_deploy_id
ENV GROUP_DATABASE_ID = your_deploy_id
ENV CLUB_DATABASE_ID = your_deploy_id
ENV EVENT_DATABASE_ID = your_deploy_id
ENV ARTICLE_DATABASE_ID = your_deploy_id
ENV SPONSOR_DATABASE_ID = your_deploy_id
ENV SSL_CERT_PATH = /etc/letsencrypt/live/api.scaict.org/fullchain.pem
ENV SSL_CERT_KEY_PATH = /etc/letsencrypt/live/api.scaict.org/privkey.pem

COPY --from=builder /scaict-website-api/target/release/scaict-website-api .

CMD ["/scaict-website-api/scaict-website-api"]

EXPOSE 8080
