ARG BINARY_NAME_DEFAULT=boilerplate

FROM clux/muslrust:stable as builder
ARG BINARY_NAME_DEFAULT
ENV BINARY_NAME=$BINARY_NAME_DEFAULT
# Build the project with target x86_64-unknown-linux-musl

# Build dummy main with the project's Cargo lock and toml
# This is a docker trick in order to avoid downloading and building
# dependencies when lock and toml not is modified.
COPY Cargo.lock .
COPY Cargo.toml .
RUN mkdir -p src/bin \
    && echo "fn main() {print!(\"Dummy main\");} // dummy file" > src/bin/main.rs
RUN set -x && cargo build --target x86_64-unknown-linux-musl --release
RUN set -x && rm target/x86_64-unknown-linux-musl/release/deps/$BINARY_NAME*

# Now add the rest of the project and build the real main
COPY src ./src
COPY config ./config
COPY migrations ./migrations
RUN set -x && cargo build --target x86_64-unknown-linux-musl --release
RUN mkdir -p /build-out
RUN set -x && cp target/x86_64-unknown-linux-musl/release/$BINARY_NAME /build-out/ && cp -R ./config /build-out/ && cp -R ./migrations /build-out/

# Create a minimal docker image
FROM alpine:3.9
RUN apk --no-cache add ca-certificates

ARG BINARY_NAME_DEFAULT
ENV BINARY_NAME=$BINARY_NAME_DEFAULT

COPY --from=builder /build-out/$BINARY_NAME /
COPY --from=builder /build-out/config /config
COPY --from=builder /build-out/migrations /migrations

# Start with an execution list (there is no sh in a scratch image)
# No shell => no variable expansion, |, <, >, etc
CMD ["/boilerplate"]