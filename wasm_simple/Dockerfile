FROM node:13 as filter

COPY . /code
WORKDIR /code/simple_filter/

RUN npm install
RUN npm run asbuild
RUN find .


# FROM envoyproxy/envoy:latest
FROM istio/proxyv2:1.5.0
COPY --from=chain_filter /code/simple_filter/build/optimized.wasm /opt/optimized.wasm
ENTRYPOINT /usr/local/bin/envoy -c /etc/envoy.yaml -l debug --service-cluster proxy
