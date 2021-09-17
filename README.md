# TileWorldRust

git clone

cargo run

using Docker:

docker build -t tileworld_rust .  

docker run -ti -e DISPLAY=$DISPLAY -v /tmp/.X11-unix:/tmp/.X11-unix:rw --volume="$HOME/.Xauthority:/root/.Xauthority:rw" --privileged --rm --init tileworld_rust