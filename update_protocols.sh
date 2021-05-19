#!/bin/bash

rm -rf data
mkdir data
cd data

mkdir -p wayland
wget https://gitlab.freedesktop.org/wayland/wayland/raw/master/protocol/wayland.xml -O wayland/wayland.xml

mkdir -p wayland-protocols
mkdir -p wlroots
mkdir -p kde
CURRENT_DIR=`pwd`
DIR=`mktemp -d`

pushd $DIR
git clone git://anongit.freedesktop.org/wayland/wayland-protocols
find wayland-protocols -name "*.xml" | xargs cp -t $CURRENT_DIR/wayland-protocols

git clone https://github.com/swaywm/wlr-protocols.git
cp -r wlr-protocols/unstable/* -t $CURRENT_DIR/wlroots

git clone https://github.com/KDE/plasma-wayland-protocols.git
cp -r plasma-wayland-protocols/src/protocols/* -t $CURRENT_DIR/kde

popd

mkdir -p other

pushd other
wget https://raw.githubusercontent.com/swaywm/wlroots/master/protocol/input-method-unstable-v2.xml 
wget https://raw.githubusercontent.com/swaywm/wlroots/master/protocol/virtual-keyboard-unstable-v1.xml
popd

touch ../src/main.rs
