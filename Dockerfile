# https://hub.docker.com/_/rust?tab=tags
FROM rust:1.37.0 as builder

# musl-gcc
RUN apt-get update \
	&& apt-get install -y \
		musl-dev \
		musl-tools \
	&& apt-get clean \
	&& rm -rf /var/lib/apt/lists/*

WORKDIR /app
RUN rustup target add x86_64-unknown-linux-musl
# cache deps https://blog.jawg.io/docker-multi-stage-build/
# RUN USER=root cargo init
# COPY Cargo.toml .
# RUN cargo build --target x86_64-unknown-linux-musl --release
# COPY src src
COPY . .
RUN cargo build --target x86_64-unknown-linux-musl --release \
	&& strip /app/target/x86_64-unknown-linux-musl/release/action-skip-ci

FROM scratch

# https://help.github.com/en/articles/metadata-syntax-for-github-actions#about-yaml-syntax-for-github-actions
LABEL version="0.1.0" \
  repository="https://github.com/meetup/action-skip-ci/" \
  homepage="https://github.com/meetup/action-skip-ci" \
  maintainer="Meetup" \
  "com.github.actions.name"="SkipCI" \
  "com.github.actions.description"="Bails out of CI on common commit message pattern" \
  "com.github.actions.icon"="skip-forward" \
  "com.github.actions.color"="green"

COPY --from=builder \
	/app/target/x86_64-unknown-linux-musl/release/action-skip-ci \
	/action-skip-ci
ENTRYPOINT ["/action-skip-ci"]