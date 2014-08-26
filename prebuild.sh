#!/bin/sh

codegen=src/codegen/target/codegen
src_dir=src/sdl2/generated

cargo build --manifest-path src/codegen/Cargo.toml
mkdir -p ${src_dir}
${codegen} keycode.rs ${src_dir}
${codegen} scancode.rs ${src_dir}
