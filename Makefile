RUSTC ?= rustc
RUSTFLAGS ?=

RUST_SRC = $(shell find src/. -type f -name '*.rs') \
	src/generated/keycode.rs                        \
	src/generated/scancode.rs

.PHONY: all
all: libsdl2.dummy

UNAME=$(shell uname)

ifeq ($(UNAME),Darwin)
  # If the user wasn't explicit, see if SDL2 library exists
  ifeq ($(SDL_MODE),)
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

src/codegen/codegen: $(wildcard src/codegen/*.rs)
	$(RUSTC) $(RUSTFLAGS) $@.rs

src/generated/%.rs: src/codegen/codegen
	src/codegen/codegen $(patsubst src/generated/%,%,$@) src/generated/

libsdl2.dummy: src/sdl2.rc $(RUST_SRC)
	$(RUSTC) $(RUSTFLAGS) $< -o $@
	touch $@

demos: demo/demo.rc libsdl2.dummy
	$(RUSTC) -L . $< -o $@

demo: demos
	./demos

.PHONY: clean
clean:
	rm -f *.so *.dylib *.dll *.dummy demos src/codegen/codegen
	rm -rf *.dSYM src/generated/ src/codegen/codegen.dSYM

