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
  docker run -it --rm -p 7500:7500 mcgill.wtf

download start='0':
  RUST_LOG=info just run download --starting-page {{start}} && \
    prettier --write data.json

fmt:
  cargo fmt

fmt-check:
  cargo fmt --all -- --check

run *args:
  cargo run -- {{args}}

serve datasource='data.json':
  RUST_LOG=info just run serve --local --datasource {{datasource}}

test:
  cargo test

watch +COMMAND='test':
  cargo watch --clear --exec "{{COMMAND}}"
