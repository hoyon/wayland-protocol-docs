#!/bin/bash

rm -rf data
mkdir data
cd data

mkdir -p wayland
wget https://gitlab.freedesktop.org/wayland/wayland/raw/master/protocol/wayland.xml -O wayland/wayland.xml

mkdir -p wayland-protocols
mkdir -p wlroots
CURRENT_DIR=`pwd`
DIR=`mktemp -d`

pushd $DIR
git clone git://anongit.freedesktop.org/wayland/wayland-protocols
cp -r wayland-protocols/stable wayland-protocols/unstable -t $CURRENT_DIR/wayland-protocols

git clone https://github.com/swaywm/wlr-protocols.git
cp -r wlr-protocols/unstable -t $CURRENT_DIR/wlroots

popd
touch ../src/main.rs
