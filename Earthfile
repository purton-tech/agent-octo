VERSION 0.8

# Build the same toolchain environment as the devcontainer without hardcoding
# the upstream image in two places.
devcontainer:
    FROM DOCKERFILE .devcontainer
    WORKDIR /workspace

# Run the Rust checks that CI enforces inside the shared devcontainer toolchain.
checks:
    FROM +devcontainer
    WORKDIR /workspace
    COPY . .
    RUN cargo fmt --check
    RUN cargo clippy --workspace --all-targets -- -D warnings

# Compile the workspace once as static musl binaries, then export the
# requested binary from the shared release output.
build:
    ARG BINARY=octo
    FROM +devcontainer
    WORKDIR /workspace
    COPY . .
    RUN rustup target add x86_64-unknown-linux-musl
    RUN cargo build --workspace --release --target x86_64-unknown-linux-musl
    SAVE ARTIFACT target/x86_64-unknown-linux-musl/release/$BINARY /$BINARY

# Package a selected binary into a scratch image tagged with the binary name.
image:
    ARG BINARY=octo
    ARG REGISTRY=your-registry
    ARG TAG=latest
    FROM scratch
    COPY (+build/$BINARY --BINARY=$BINARY) /$BINARY
    ENTRYPOINT ["/$BINARY"]
    SAVE IMAGE $REGISTRY/$BINARY:$TAG

release-candidate:
    ARG REGISTRY=ghcr.io/purton-tech
    ARG VERSION
    BUILD +checks
    BUILD +image --BINARY=octo --REGISTRY=$REGISTRY --TAG=$VERSION-rc

# Build all currently known binary crates. Add one BUILD line per new bin.
all:
    ARG REGISTRY=ghcr.io/purton-tech
    BUILD +checks
    BUILD +image --BINARY=octo --REGISTRY=$REGISTRY --TAG=latest
