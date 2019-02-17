#!/usr/bin/env bash

set -xueo pipefail

RUST_HOST=$(rustup show | grep -F "Default host" | sed "s/Default host: //")
RUST_TOOLCHAIN=$(rustup show | grep -F "(default)" | sed "s/ (default)//")

MSBUILD='/C/Program Files (x86)/Microsoft Visual Studio/2017/BuildTools/MSBuild/15.0/Bin/MSBuild.exe'

if [[ "$TRAVIS_OS_NAME" == "windows" ]]; then
    EXT=.zip
    EXTRACT="unzip -q"
    PREFIX=/C/Users/travis/.rustup/toolchains/${RUST_TOOLCHAIN}/lib/rustlib/${RUST_HOST}/
    cat "/C/Program Files (x86)/Microsoft Visual Studio/2017/BuildTools/Common7/IDE/VC/VCTargets/Microsoft.Cpp.WindowsSDK.targets"
    WINSDK_ROOT="/C/Program Files (x86)/Windows Kits/10/DesignTime/CommonConfiguration/Neutral/UAP"
    ls -l "${WINSDK_ROOT}"
    for WINSDK_MAYBE in $(ls "${WINSDK_ROOT}"); do
        ls -l "${WINSDK_ROOT}/${WINSDK_MAYBE}"
        if [[ -f "${WINSDK_ROOT}/${WINSDK_MAYBE}/UAP.props" ]]; then
            export WINSDK=${WINSDK_MAYBE}
        fi
    done
    TOOLSET=$(grep -m 1 "PlatformToolset" "/C/Program Files (x86)/Microsoft Visual Studio/2017/BuildTools/Common7/IDE/VC/VCWizards/default.vcxproj" | sed "s/    <PlatformToolset>//" | sed "s!</PlatformToolset>!!")
    export PATH=$PATH:/C/Program\ Files\ \(x86\)/Microsoft\ Visual\ Studio/2017/BuildTools/MSBuild/15.0/Bin:${PREFIX}/bin
else
    EXT=.tar.gz
    EXTRACT="tar xzf"
fi

function nuke_bin_in_path() {
    echo $PATH | tr ":" "\n" | grep -v /usr/bin | tr "\n" ":"
}

function build() {
    pushd $1
    if [[ "$TRAVIS_OS_NAME" == "windows" ]]; then
        if [[ "$TRAVIS_RUST_VERSION" == *"-gnu" ]]; then
            LD_LIBRARY_PATH=${PREFIX}/lib
            CONFIG_SHELL="/C/Program\\ Files/Git/bin/bash"
            ./configure --build=x86_64-mingw32 --prefix=${PREFIX} || return 1
            sed -i 's!/bin/sh!"/C/Program Files/Git/bin/bash"!' Makefile
            cat Makefile
            mingw32-make
            mingw32-make install || return 1
        else
            cd VisualC
            export INCLUDE=../../SDL-2.0.9/include
            pwd
            ls ..
            ls ../..
            ls ../../SDL-2.0.9
            ls ${INCLUDE}
            export LIB=${PREFIX}/lib
            export UseEnv=true
            cmd <<EOF
"C:\\Program Files (x86)\\Microsoft Visual Studio\\2017\\BuildTools\\VC\\Auxiliary\\Build\\vcvars64.bat"
echo %INCLUDE%
echo %UseEnv%
msbuild $2 -p:Configuration=Release -p:Platform=x64 -p:PlatformToolset=${TOOLSET} -p:WindowsTargetPlatformVersion=${WINSDK}
EOF
            echo
            ls *
            cp SDL/x64/Release/*.lib SDL/x64/Release/*.dll ${PREFIX}/lib/ || return 1
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
build SDL2-* SDL\\SDL.vcxproj || exit 1
if [[ "$CI_BUILD_FEATURES" == *"ttfBROKEN"* ]]; then
    wget -q https://www.libsdl.org/projects/SDL_ttf/release/SDL2_ttf-2.0.14${EXT}
    ${EXTRACT} SDL2_ttf-*${EXT} && rm SDL2_ttf-*${EXT}
    build SDL2_ttf-*
fi
if [[ "$CI_BUILD_FEATURES" == *"image"* ]]; then
    wget -q https://www.libsdl.org/projects/SDL_image/release/SDL2_image-2.0.1${EXT}
    ${EXTRACT} SDL2_image-*${EXT} && rm SDL2_image-*${EXT}
    build SDL2_image-* SDL_image.vcxproj
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
