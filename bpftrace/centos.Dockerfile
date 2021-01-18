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

# -----
FROM centos:8

RUN dnf update -y
RUN yum install -y yum-utils
RUN yum-config-manager --add-repo https://getenvoy.io/linux/centos/tetrate-getenvoy.repo
RUN yum-config-manager --enable tetrate-getenvoy-nightly
RUN dnf install -y getenvoy-envoy bpftrace

COPY --from=basetools /code/filter/target/wasm32-unknown-unknown/release/filter.wasm /opt/filter.wasm

ENTRYPOINT /usr/bin/envoy -c /etc/envoy.yaml -l debug --service-cluster proxy
