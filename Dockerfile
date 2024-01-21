# First stage: build the Rust application
FROM rust:alpine AS builder
LABEL authors="Sam Huddart"

RUN apk add build-base

WORKDIR /work

COPY src/ /work/src/
COPY web/ /work/web/
COPY Cargo.toml /work/
COPY Cargo.lock /work/

# Run cargo build with the release flag
RUN cargo build --release

# Second stage: create a minimal image with the compiled binary
FROM alpine
LABEL name="QrService" \
      maintainer="sam.fucked.up@samh.dev" \
      vendor="SamHDev" \
      version="0.0.1" \
      summary="Redirect Service for static QR codes" \
      description="URL Redirect and Web Configuration Service designed for static QR codes"

# Set environment variables
ENV BIND="0.0.0.0:8080"
ENV CONFIG="config.toml"

# Copy the binary from the first stage
COPY --from=builder /work/target/release/qr_service /usr/local/bin/
RUN chmod +x /usr/local/bin/qr_service

RUN ldd /usr/local/bin/qr_service

# Run the binary
ENTRYPOINT ["/usr/local/bin/qr_service"]