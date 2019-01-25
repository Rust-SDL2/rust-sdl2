#!/usr/bin/env bash

set -xueo pipefail
RUST_TOOLCHAIN=$(rustup show | grep -A 3 active | tail -1 | sed "s/ (default)//")
RUST_HOST=$(rustup show | grep "Default host" | sed "s/Default host: //")

rustup which cargo

function build() {
    if [[ "$TRAVIS_OS_NAME" == "windows" ]]; then
        if [[ "$TRAVIS_RUST_VERSION" == *"-gnu" ]]; then
            ./configure --build=x86_64-mingw32 --prefix=/c/Users/travis/.rustup/toolchains/${RUST_TOOLCHAIN}/lib/rustlib/${RUST_HOST}/
            mingw32-make V=1
            mingw32-make install
        else
            cd VisualC
            msbuild
        fi
    else
        ./configure
        make
        sudo make install
    fi
}

wget https://www.libsdl.org/release/SDL2-2.0.9.tar.gz -O sdl2.tar.gz
tar xzf sdl2.tar.gz
(pushd SDL2-* && build && popd) || exit
if [[ "$TRAVIS_OS_NAME" == "windows" ]]; then
    EXT=.zip
    EXTRACT=unzip
else
    EXT=.tar.gz
    EXTRACT="tar xzf"
fi
wget -q https://www.libsdl.org/projects/SDL_ttf/release/SDL2_ttf-2.0.14${EXT}
wget -q https://www.libsdl.org/projects/SDL_image/release/SDL2_image-2.0.1${EXT}
wget -q https://www.libsdl.org/projects/SDL_mixer/release/SDL2_mixer-2.0.2${EXT}
wget -q -O SDL2_gfx-1.0.1${EXT} https://sourceforge.net/projects/sdl2gfx/files/SDL2_gfx-1.0.1${EXT}/download
${EXTRACT} SDL2_ttf-*${EXT} && rm SDL2_ttf-*${EXT}
${EXTRACT} SDL2_image-*${EXT} && rm SDL2_image-*${EXT}
${EXTRACT} SDL2_mixer-*${EXT} && rm SDL2_mixer-*${EXT}
${EXTRACT} SDL2_gfx-*${EXT} && rm SDL2_gfx-*${EXT}
# pushd SDL2_ttf-* && build && popd
pushd SDL2_image-* && build && popd
pushd SDL2_mixer-* && build && popd
pushd SDL2_gfx-* && ./autogen.sh && build && popd
