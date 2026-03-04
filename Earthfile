VERSION 0.8

# Build the same toolchain environment as the devcontainer without hardcoding
# the upstream image in two places.
devcontainer:
    FROM DOCKERFILE .devcontainer
    WORKDIR /workspace

certs:
    FROM alpine:3.19
    RUN apk add --no-cache ca-certificates
    SAVE ARTIFACT /etc/ssl/certs/ca-certificates.crt /ca-certificates.crt

# Run the Rust checks that CI enforces inside the shared devcontainer toolchain.
checks:
    FROM +devcontainer
    WORKDIR /workspace
    COPY . .
    RUN cargo fmt --check
    RUN cargo clippy --workspace --all-targets -- -D warnings

# Compile the workspace once as static musl binaries, then export all of the
# known binaries from the shared release output.
build:
    FROM +devcontainer
    WORKDIR /workspace
    COPY . .
    RUN rustup target add x86_64-unknown-linux-musl
    RUN cargo build --workspace --release --target x86_64-unknown-linux-musl
    SAVE ARTIFACT target/x86_64-unknown-linux-musl/release/octo /octo
    SAVE ARTIFACT target/x86_64-unknown-linux-musl/release/telegram-ingress-polling /telegram-ingress-polling
    SAVE ARTIFACT target/x86_64-unknown-linux-musl/release/telegram-egress /telegram-egress
    SAVE ARTIFACT target/x86_64-unknown-linux-musl/release/agent-runtime /agent-runtime

# Package a selected binary into a scratch image tagged with the binary name.
image:
    ARG BINARY=octo
    ARG REGISTRY=your-registry
    ARG TAG=latest
    FROM scratch
    COPY +certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
    COPY +build/$BINARY /app
    USER 65532:65532
    ENTRYPOINT ["/app"]
    SAVE IMAGE --push $REGISTRY/$BINARY:$TAG

# Package the dbmate migrations into a one-shot image that runs `dbmate up`
# at startup. Attach this via Stack's `init` section so migrations complete
# before the main service starts.
migration-image:
    ARG REGISTRY=your-registry
    ARG TAG=latest
    FROM ghcr.io/amacneil/dbmate:2.26.0
    COPY crates/db/migrations /migrations
    ENTRYPOINT ["dbmate", "--no-dump-schema", "--migrations-dir", "/migrations", "up"]
    SAVE IMAGE --push $REGISTRY/octo-migrations:$TAG

release-candidate:
    ARG REGISTRY=ghcr.io/purton-tech
    ARG TAG
    BUILD +checks
    BUILD +image --BINARY=octo --REGISTRY=$REGISTRY --TAG=$TAG
    BUILD +image --BINARY=telegram-ingress-polling --REGISTRY=$REGISTRY --TAG=$TAG
    BUILD +image --BINARY=telegram-egress --REGISTRY=$REGISTRY --TAG=$TAG
    BUILD +image --BINARY=agent-runtime --REGISTRY=$REGISTRY --TAG=$TAG
    BUILD +migration-image --REGISTRY=$REGISTRY --TAG=$TAG

# Build all currently known binary crates. Add one BUILD line per new bin.
all:
    ARG REGISTRY=ghcr.io/purton-tech
    BUILD +checks
    BUILD +image --BINARY=octo --REGISTRY=$REGISTRY --TAG=latest
    BUILD +image --BINARY=telegram-ingress-polling --REGISTRY=$REGISTRY --TAG=latest
    BUILD +image --BINARY=telegram-egress --REGISTRY=$REGISTRY --TAG=latest
    BUILD +image --BINARY=agent-runtime --REGISTRY=$REGISTRY --TAG=latest
    BUILD +migration-image --REGISTRY=$REGISTRY --TAG=latest
