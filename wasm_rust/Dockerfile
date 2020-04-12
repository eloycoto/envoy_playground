FROM rust:1.42.0 as basetools

COPY . /code
WORKDIR /code/filter

RUN rustup target install wasm32-unknown-unknown
RUN cargo build --target=wasm32-unknown-unknown --release
RUN find . | grep "wasm$"

# FROM envoyproxy/envoy:latest
FROM istio/proxyv2:1.5.0
COPY --from=basetools /code/filter/target/wasm32-unknown-unknown/release/filter.wasm /opt/filter.wasm
ENTRYPOINT /usr/local/bin/envoy -c /etc/envoy.yaml -l debug --service-cluster proxy

