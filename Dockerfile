# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------

FROM rust:latest as cargo-build

RUN apt-get update

RUN apt-get install libboost-all-dev git -y

WORKDIR /var/tmp

RUN git clone https://github.com/VROOM-Project/vroom.git

RUN cd vroom && git checkout v1.5.0

RUN cd vroom/src && make

RUN rustup target add x86_64-unknown-linux-gnu

WORKDIR vrp

COPY . .

RUN cargo build --release --target=x86_64-unknown-linux-gnu

RUN rm -f target/x86_64-unknown-linux/release/deps/vroom*

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

FROM alpine:latest

RUN addgroup -g 1000 vrp

RUN adduser -D -s /bin/sh -u 1000 -G vrp vrp

WORKDIR /home/vrp/bin/

COPY --from=cargo-build /var/tmp/vrp/target/x86_64-unknown-linux-gnu/release/vrp .
COPY --from=cargo-build /var/tmp/vroom/bin/vroom /var/tmp/vroom

RUN chown vrp:vrp vrp

USER vrp

CMD ["./vrp"]
