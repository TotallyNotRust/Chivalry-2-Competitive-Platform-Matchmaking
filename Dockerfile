FROM totallynotrust/rust-default:latest

COPY . /app
WORKDIR /app

RUN ~/.cargo/bin/rustup default nightly

RUN ~/.cargo/bin/cargo build --release

EXPOSE 8081

ENTRYPOINT [ "./target/release/c2cp-matchmaker" ]
#ENTRYPOINT [ "sleep", "infinity" ]