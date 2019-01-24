#!/usr/bin/env bash

set -xueo pipefail

wget https://www.libsdl.org/release/SDL2-2.0.9.tar.gz -O sdl2.tar.gz
tar xzf sdl2.tar.gz
pushd SDL2-* && ./configure && make && sudo make install && popd
if [[ "$TRAVIS_OS_NAME" == "windows" ]]; then
    EXT=.zip
    EXTRACT=unzip
else
    EXT=.tar.gz
    EXTRACT=tar xzf
fi
wget -q https://www.libsdl.org/projects/SDL_ttf/release/SDL2_ttf-2.0.14${EXT}
wget -q https://www.libsdl.org/projects/SDL_image/release/SDL2_image-2.0.1${EXT}
wget -q https://www.libsdl.org/projects/SDL_mixer/release/SDL2_mixer-2.0.2${EXT}
wget -q -O SDL2_gfx-1.0.1${EXT} https://sourceforge.net/projects/sdl2gfx/files/SDL2_gfx-1.0.1${EXT}/download
${EXTRACT} SDL2_ttf-*${EXT}
${EXTRACT} SDL2_image-*${EXT}
${EXTRACT} SDL2_mixer-*${EXT}
${EXTRACT} SDL2_gfx-*${EXT}
pushd SDL2_ttf-* && ./configure && make && sudo make install && popd
pushd SDL2_image-* && ./configure && make && sudo make install && popd
pushd SDL2_mixer-* && ./configure && make && sudo make install && popd
pushd SDL2_gfx-* && ./autogen.sh && ./configure && make && sudo make install && popd
