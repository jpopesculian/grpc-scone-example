FROM rust:latest

# DEPS
RUN rustup target add x86_64-unknown-linux-musl

RUN apt-get update && apt-get install -y \
    make cmake zsh vim build-essential musl-tools curl unzip

# PROTOBUF
ENV PROTOBUF_VER=3.8.0
RUN curl -OL https://github.com/google/protobuf/releases/download/v$PROTOBUF_VER/protoc-$PROTOBUF_VER-linux-x86_64.zip
RUN unzip protoc-$PROTOBUF_VER-linux-x86_64.zip -d protoc-$PROTOBUF_VER
RUN mv protoc-$PROTOBUF_VER/bin/* /usr/local/bin/
RUN mv protoc-$PROTOBUF_VER/include/* /usr/local/include/

# APP
WORKDIR build
COPY . .

RUN make release

# RUNTIME

FROM alpine:latest

COPY --from=0 /build/target/x86_64-unknown-linux-musl/release/grpc-example .

CMD ["./grpc-example"]
