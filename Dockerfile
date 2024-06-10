FROM rust

WORKDIR /usr/src/gecko-server

COPY . .

RUN cargo build --release --target-dir /usr/src/gecko-server/output/

CMD [ "/usr/src/gecko-server/output/release/gecko-server" ]
