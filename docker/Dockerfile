FROM rust:latest

# DEPS
RUN rustup target add x86_64-unknown-linux-musl

RUN apt-get update && apt-get install -y \
    make cmake zsh vim build-essential musl-tools curl unzip

# PROTOBUF
RUN curl -OL https://github.com/google/protobuf/releases/download/v3.5.1/protoc-3.5.1-linux-x86_64.zip
RUN unzip protoc-3.5.1-linux-x86_64.zip -d protoc3
RUN mv protoc3/bin/* /usr/local/bin/
RUN mv protoc3/include/* /usr/local/include/

# APP
WORKDIR build
COPY . .

RUN make release

# RUNTIME

FROM alpine:latest

COPY --from=0 /build/target/x86_64-unknown-linux-musl/release/zenroom-service .

CMD ["./zenroom-service"]
