FROM rust as basetools

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

FROM envoyproxy/envoy-debug-dev
COPY --from=basetools /code/filter/target/wasm32-unknown-unknown/release/filter.wasm /opt/filter.wasm
ADD  https://raw.githubusercontent.com/envoyproxy/envoy/master/tools/stack_decode.py /
RUN chmod 777 stack_decode.py
RUN apt-get update && apt-get install -y python3 binutils
ENTRYPOINT /stack_decode.py /usr/local/bin/envoy -c /etc/envoy.yaml -l debug --service-cluster proxy

