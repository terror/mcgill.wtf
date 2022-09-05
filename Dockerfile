FROM rust:1.63.0 as builder

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

COPY --from=builder /usr/src/app/target/release/server /usr/bin
COPY --from=builder /usr/src/app/data.json .

CMD server serve --datasource ./data.json
