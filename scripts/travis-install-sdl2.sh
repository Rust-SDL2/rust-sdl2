#!/usr/bin/env bash

set -xueo pipefail

RUST_HOST=$(rustup show | grep -F "Default host" | sed "s/Default host: //")
RUST_TOOLCHAIN=$(rustup show | grep -F "(default)" | sed "s/ (default)//")

MSBUILD=/C/Program\ Files\ \(x86\)/Microsoft\ Visual\ Studio/2017/BuildTools/MSBuild/15.0/Bin/MSBuild.exe

if [[ "$TRAVIS_OS_NAME" == "windows" ]]; then
    ls -l ${MSBUILD}
    ${MSBUILD} /help
    EXT=.zip
    EXTRACT=unzip
    PREFIX=/C/Users/travis/.rustup/toolchains/${RUST_TOOLCHAIN}/lib/rustlib/${RUST_HOST}/
    export PATH=$PATH:/C/Program\ Files\ \(x86\)/Microsoft\ Visual\ Studio/2017/BuildTools/MSBuild/15.0/Bin:${PREFIX}/bin
else
    EXT=.tar.gz
    EXTRACT="tar xzf"
fi

function build() {
    pushd $1
    if [[ "$TRAVIS_OS_NAME" == "windows" ]]; then
        if [[ "$TRAVIS_RUST_VERSION" == *"-gnu" ]]; then
            LD_LIBRARY_PATH=${PREFIX}/lib
            ./configure --build=x86_64-mingw32 --prefix=${PREFIX} || return 1
            mingw32-make SHELL=/C/Program\ Files/Git/usr/bin/sh || return 1
            mingw32-make install SHELL=/C/Program\ Files/Git/usr/bin/sh || return 1
        else
            cd VisualC
            ${MSBUILD} /p:Configuration=Release /p:Platform=x64 /p:PlatformToolset=v141 /p:WindowsTargetPlatformVersion=10.0.17763.0
            cp x64/Release/*.lib x64/Release/*.dll ${PREFIX}/lib/
        fi
    else
        ./configure
        make
        sudo make install
    fi
    popd
}

wget https://www.libsdl.org/release/SDL2-2.0.9.tar.gz -O sdl2.tar.gz
tar xzf sdl2.tar.gz
build SDL2-* || exit 1
if [[ "$CI_BUILD_FEATURES" == *"ttfBROKEN"* ]]; then
    wget -q https://www.libsdl.org/projects/SDL_ttf/release/SDL2_ttf-2.0.14${EXT}
    ${EXTRACT} SDL2_ttf-*${EXT} && rm SDL2_ttf-*${EXT}
    build SDL2_ttf-*
fi
if [[ "$CI_BUILD_FEATURES" == *"image"* ]]; then
    wget -q https://www.libsdl.org/projects/SDL_image/release/SDL2_image-2.0.1${EXT}
    ${EXTRACT} SDL2_image-*${EXT} && rm SDL2_image-*${EXT}
    build SDL2_image-*
fi
if [[ "$CI_BUILD_FEATURES" == *"mixer"* ]]; then
    wget -q https://www.libsdl.org/projects/SDL_mixer/release/SDL2_mixer-2.0.2${EXT}
    ${EXTRACT} SDL2_mixer-*${EXT} && rm SDL2_mixer-*${EXT}
    build SDL2_mixer-*
fi
if [[ "$CI_BUILD_FEATURES" == *"gfx"* ]]; then
    wget -q -O SDL2_gfx-1.0.1${EXT} https://sourceforge.net/projects/sdl2gfx/files/SDL2_gfx-1.0.1${EXT}/download
    ${EXTRACT} SDL2_gfx-*${EXT} && rm SDL2_gfx-*${EXT}
    build SDL2_gfx-*
fi
