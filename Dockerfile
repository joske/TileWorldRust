FROM rust

RUN apt update && apt install -y libgtk-3-dev

WORKDIR /tile_world_rust

COPY . /tile_world_rust/

RUN cargo build --release

RUN cargo install --path .

CMD ["target/release/tile_world_rust"]