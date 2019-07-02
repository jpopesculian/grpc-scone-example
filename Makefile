CARGO ?= cargo

DOCKERFILE ?= docker/Dockerfile
DOCKERTAG ?= grpc-example

GENERATED_PROTOS = $(filter-out src/proto/mod.rs, $(wildcard src/proto/*.rs))

.PHONY: all build run check watch docker docker-run

all: build

build:
	@$(CARGO) build $(CARGO_BUILD_FLAGS)

musl: CC=musl-gcc ./configure
musl: CARGO_BUILD_FLAGS += --target x86_64-unknown-linux-musl
musl: build

release: CARGO_BUILD_FLAGS += --release
release: musl

run:
	@$(CARGO) run

check:
	@$(CARGO) clippy

watch:
	@$(CARGO) watch -w src -x run

clean:
	rm -f $(GENERATED_PROTOS)
	@$(CARGO) clean

docker: clean
	docker build -f $(DOCKERFILE) -t ${DOCKERTAG} .

docker-run:
	@docker run -it --rm --device=/dev/isgx -p 3000:3000 ${DOCKERTAG}

scone: DOCKERFILE := $(DOCKERFILE).sgx
scone: DOCKERTAG := $(DOCKERTAG)-scone
scone: docker

scone-run: DOCKERTAG := $(DOCKERTAG)-scone
scone-run: docker-run
