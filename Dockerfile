# select build image
FROM rust:1.35-stretch as build

# create a new empty shell project
RUN USER=root cargo new --bin server
WORKDIR /server

# Initial update crates.io index for container image cache
RUN cargo search --limit 0

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

ARG BUILD_MODE=release
ENV BUILD_MODE=${BUILD_MODE}

RUN echo "BUILD_MODE: ${BUILD_MODE}"

# this build step will cache your dependencies
RUN if [ "$BUILD_MODE" = "release" ] ; then cargo build --release ; else cargo build ; fi
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build
RUN rm ./target/${BUILD_MODE}/deps/server*
RUN if [ "$BUILD_MODE" = "release" ] ; then cargo build --release ; else cargo build ; fi
RUN mkdir -p /out
RUN cp ./target/${BUILD_MODE}/server /out/server  ; fi

################################################################################
# our final base
FROM debian:stretch

ENV RUST_LOG_STYLE=never
ENV RUST_LOG="server=info,actix_web=info"

RUN set -eux; \
    dpkgArch="$(dpkg --print-architecture)"; \
    apt-get update -yqq \
    && apt-get install -yqq --no-install-recommends \
       libssl1.1 ca-certificates

RUN set -eux \
    && apt-get clean \
    && rm -rf \
        /var/lib/apt/lists/* \
        /tmp/* \
        /var/tmp/* \
        /usr/share/man \
        /usr/share/doc \
        /usr/share/doc-base

# Add Tini
ENV TINI_VERSION v0.18.0
ADD https://github.com/krallin/tini/releases/download/${TINI_VERSION}/tini /tini
RUN chmod +x /tini
ENTRYPOINT ["/tini", "--", "./server"]

# copy the build artifact from the build stage
COPY --from=build /out/server .

EXPOSE 18080

# set the startup command to run your binary
CMD ["--port", "18080", "--workers", "1024"]

