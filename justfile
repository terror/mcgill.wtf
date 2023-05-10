default:
  just --list

alias b := build
alias f := fmt
alias r := run

all: build test clippy fmt-check

build:
  cargo build

clippy:
  cargo clippy --all-targets --all-features

container:
  docker build -t mcgill.wtf .

develop:
  docker run -d --rm --name mcgill.wtf -p 7500:7500 mcgill.wtf

download start='0':
  RUST_LOG=info just run download --starting-page {{start}} && \
    prettier --write data.json

fmt:
  cargo fmt
  prettier --write .

fmt-check:
  cargo fmt --all -- --check

run *args:
  cargo run -- {{args}}

serve datasource='data.json':
  RUST_LOG=info ./bin/serve -l -d {{datasource}}

test:
  cargo test

upload:
  just download
  aws s3 cp data.json s3://mcgill.wtf/data.json

watch +COMMAND='test':
  cargo watch --clear --exec "{{COMMAND}}"
