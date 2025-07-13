default: check
alias al := actionlint
alias b := build
alias c := check
alias d := deny
alias e := expand
alias f := fmt
alias fc := fmt-check
alias i := install
alias jp := publish-dry
alias l := lint
alias lf := lint-fix
alias r := run
alias re := review
alias t := test
alias yf := yaml-fmt

aud:
  cargo audit --all-targets

build:
  cargo build

check:
  cargo check --all-targets

cov:
  cargo llvm-cov

deny:
  cargo deny check --hide-inclusion-graph

expand:
  cargo expand

fmt:
  cargo fmt --all

fmt-check:
  cargo fmt --all -- --check

install:
  cargo install --path .

lint:
  cargo clippy --all-targets

lint-fix:
  cargo clippy --fix  --allow-dirty --allow-staged

publish-dry:
  cargo publish --dry-run --allow-dirty

test:
  cargo nextest run

run *FLAGS:
  cargo run {{FLAGS}}

review:
  cargo insta test --review

actionlint:
  actionlint

yaml-fmt:
  yamlfmt $(git ls-files '*.yml')

a:
  cargo check --all-targets
  cargo fmt --all
  cargo clippy --all-targets
  cargo nextest run
