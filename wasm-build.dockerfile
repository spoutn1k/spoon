FROM ubuntu:22.10

ARG DEBIAN_FRONTEND=noninteractive
ENV TZ=Etc/PDT
ENV TZ=Etc/PDT
ENV PATH="${PATH}:/root/.cargo/bin"

RUN apt update && apt install -y gcc curl libssl-dev make perl

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > sh.rustup.rs
RUN sh sh.rustup.rs -y
RUN $HOME/.cargo/bin/rustup target add wasm32-unknown-unknown
RUN $HOME/.cargo/bin/cargo install wasm-pack
RUN $HOME/.cargo/bin/cargo install --locked trunk
