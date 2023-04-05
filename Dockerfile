FROM node:16-alpine AS client

WORKDIR /app
COPY package.json package-lock.json ./
RUN npm ci
COPY . .
RUN npm run build

FROM rust:latest as builder

WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM redis/redis-stack-server:7.0.2-RC1 AS redis-stack

FROM redis:7-bullseye

RUN ln -sf /bin/bash /bin/sh

RUN apt-get update && \
  apt-get install -y ca-certificates procps && \
  apt-get clean

COPY --from=redis-stack /opt/redis-stack/lib/redisearch.so /opt/redis-stack/lib/redisearch.so
COPY --from=redis-stack /opt/redis-stack/lib/rejson.so /opt/redis-stack/lib/rejson.so
COPY --from=builder /usr/src/app/bin/serve /usr/bin
COPY --from=builder /usr/src/app/target/release/server /usr/bin
COPY --from=client /app/client/dist assets

# https://community.fly.io/t/swap-memory/2749

CMD if [[ ! -z "$SWAP" ]]; then \
  fallocate -l $(($(stat -f -c "(%a*%s/10)*7" .))) _swapfile && \
  mkswap _swapfile && swapon _swapfile && ls -hla; \
  fi; \
  free -m; \
  serve -a assets -d https://s3.amazonaws.com/mcgill.wtf/data.json
