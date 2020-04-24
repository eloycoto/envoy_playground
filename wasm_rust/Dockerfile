FROM rust:1.42.0 as basetools

RUN rustup target install wasm32-unknown-unknown

# For caching
COPY filter/Cargo.toml filter/Cargo.lock /code/filter/

RUN mkdir -p /code/filter/src/
RUN touch /code/filter/src/lib.rs
WORKDIR /code/filter/
RUN cargo build --target=wasm32-unknown-unknown --release

# For compile the code
COPY . /code
WORKDIR /code/filter/
RUN cargo build --target=wasm32-unknown-unknown --release

# FROM envoyproxy/envoy:latest
FROM istio/proxyv2:1.5.0
COPY --from=basetools /code/filter/target/wasm32-unknown-unknown/release/filter.wasm /opt/filter.wasm
ENTRYPOINT /usr/local/bin/envoy -c /etc/envoy.yaml -l debug --service-cluster proxy

