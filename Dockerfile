FROM rustlang/rust:nightly-buster-slim

RUN apt-get update && apt-get install -y clang
WORKDIR /build
COPY . .
RUN rustup target add wasm32-unknown-unknown && cargo build --release

FROM debian:buster-slim

ENV RUST_BACKTRACE 1

RUN apt-get update && apt-get upgrade -y && apt-get install -y \
    libssl1.1 \
    ca-certificates \
    curl && \
	apt-get autoremove -y && \
	apt-get clean && \
	find /var/lib/apt/lists/ -type f -not -name lock -delete && \
	useradd -m -u 1000 -U -s /bin/sh -d /meta-athlete substrate

COPY --from=0 /build/target/release/meta-athlete-node /usr/local/bin/meta-athlete

USER substrate

RUN /usr/local/bin/meta-athlete --version

EXPOSE 30333 9933 9944
VOLUME ["/meta-athlete"]

ENTRYPOINT ["/usr/local/bin/meta-athlete"]
