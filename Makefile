RUSTFLAGS ?=

RUST_SRC = $(shell find src/. -type f -name '*.rs') \
	src/sdl2/generated/keycode.rs                   \
	src/sdl2/generated/scancode.rs

.PHONY: all
all: libsdl2.dummy

UNAME=$(shell uname)

ifeq ($(UNAME),Darwin)
  # If the user wasn't explicit, see if SDL2 library exists
  ifeq ($(strip $(SDL_MODE)),"")
    SDL_CHECK=$(shell pkg-config --exists sdl2)
    ifeq ($(SDL_CHECK),0)
      SDL_MODE = dylib
    else
      SDL_MODE = framework
    endif
  endif

  ifeq ($(SDL_MODE),framework)
    RUSTFLAGS+=--cfg mac_framework
  else
    RUSTFLAGS+=--cfg mac_dylib
  endif
endif

bin/codegen: $(wildcard src/codegen/*.rs)
	rustpkg install codegen $(RUSTFLAGS)

src/sdl2/generated/%.rs: bin/codegen
	bin/codegen $(patsubst src/sdl2/generated/%,%,$@) src/sdl2/generated/

libsdl2.dummy: src/sdl2/lib.rs $(RUST_SRC)
	rustpkg build sdl2 $(RUSTFLAGS)
	touch $@

compile_demo: src/demo/main.rs src/demo/video.rs libsdl2.dummy
	rustpkg install demo

demo: compile_demo
	./bin/demo

.PHONY: clean
clean:
	rustpkg clean codegen
	rustpkg uninstall codegen
	rustpkg clean sdl2
	rustpkg clean demo
	rustpkg uninstall demo
	rm -f *.dummy
	rm -rf src/sdl2/generated

