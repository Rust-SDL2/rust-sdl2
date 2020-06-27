In this file will be listed the changes, especially the breaking ones that one should be careful of
when upgrading from a version of rust-sdl2 to another.

### v0.34.2

[PR #1012](https://github.com/Rust-SDL2/rust-sdl2/pull/1012)
Fix use after free with AudioDevice::open and AudioDevice::open\_queue

[PR #1010](https://github.com/Rust-SDL2/rust-sdl2/pull/1010)
Fix compilation with gcc10.

[PR #1009](https://github.com/Rust-SDL2/rust-sdl2/pull/1009)
Add support for linking to development libraries from vcpkg, and automatically setting up a vcpkg installation using `cargo-vcpkg`.

### v0.34.1

[PR #1004](https://github.com/Rust-SDL2/rust-sdl2/pull/1004) + [PR #1005](https://github.com/Rust-SDL2/rust-sdl2/pull/1005):
Add convenience functions for Window and Event.

[PR #996](https://github.com/Rust-SDL2/rust-sdl2/pull/996):
Impl `From<i32>` for `WindowPos`.

[PR #988](https://github.com/Rust-SDL2/rust-sdl2/pull/988):
Add convenience functions for `Texture`.

### v0.34.0

[PR #962](https://github.com/Rust-SDL2/rust-sdl2/pull/962):
Added `raw-window-handle` support for Windows, Linux (X11 and Wayland) and macOS.

[PR #964](https://github.com/Rust-SDL2/rust-sdl2/pull/964):
**Breaking change**: Change joystick instance IDs from i32 to u32.

[PR #965](https://github.com/Rust-SDL2/rust-sdl2/pull/965):
Added invert method for Color, and added more Color names as constants.

[PR #968](https://github.com/Rust-SDL2/rust-sdl2/pull/968)
Pass SDL2 include directories to `sdl2-sys`'s dependant crates through `DEP_SDL2_INCLUDE`.

[PR #970](https://github.com/Rust-SDL2/rust-sdl2/pull/970)
Add `Chunk::from_buffer` in mixer.

[PR #976](https://github.com/Rust-SDL2/rust-sdl2/pull/976)
Fix compilation with `use-bingen` feature.

[PR #977](https://github.com/Rust-SDL2/rust-sdl2/pull/977)
Add `cpuinfo::has_avx2` and `cpuinfo::has_avx512f`

[PR #982](https://github.com/Rust-SDL2/rust-sdl2/pull/982)
Support for loading Opus format in mixer.

[PR #980](https://github.com/Rust-SDL2/rust-sdl2/pull/980)
Fix compilation on Windows.

### v0.33

[PR #956](https://github.com/Rust-SDL2/rust-sdl2/pull/956) + [PR #960](https://github.com/Rust-SDL2/rust-sdl2/pull/960) + [PR #951](https://github.com/Rust-SDL2/rust-sdl2/pull/951):
Fix some build targets.

[PR #948](https://github.com/Rust-SDL2/rust-sdl2/pull/948) + [PR #957](https://github.com/Rust-SDL2/rust-sdl2/pull/957):
Remove the `num` dependency.

[PR #947](https://github.com/Rust-SDL2/rust-sdl2/pull/947):
Upgraded the "bundled" version of SDL2 to 2.0.10

[PR #940](https://github.com/Rust-SDL2/rust-sdl2/pull/940):
**Breaking change** Removed the `rand` dependency for random colors. You will have to implement your own random colors from now on.

[PR #933](https://github.com/Rust-SDL2/rust-sdl2/pull/933):
**Breaking change** Removed AudioFormatNum::zero(), use AudioFormatNum::SILENCE constant instead.

[PR #907](https://github.com/Rust-SDL2/rust-sdl2/pull/907):
Changed the data type to i32 for the `which` field for the events `ControllerDeviceAdded` and `JoyDeviceAdded`.

[PR #882](https://github.com/Rust-SDL2/rust-sdl2/pull/882):
Ignore unknown bits in `SDL_Keysym`'s `mod` field (key modifiers) when constructing `Event::KeyDown` and `Event::KeyUp`. Deprecate `sdl2::event::Event::unwrap_keymod`, which had been made public accidentally.

[PR #898](https://github.com/Rust-SDL2/rust-sdl2/pull/898):
Implements `TryFrom<PixelFormatEnum>` for `PixelFormat`

### v0.32.2

[PR #868](https://github.com/Rust-SDL2/rust-sdl2/pull/868):
Added inplace operations for `rect::Point`.

[PR #827](https://github.com/Rust-SDL2/rust-sdl2/pull/827):
Added 32-bit array pixelformats

[PR #824](https://github.com/Rust-SDL2/rust-sdl2/pull/824):
Added `controller::set_rumble` and `joystick::set_rumble`, wrappers for `SDL_GameControllerRumble` and `SDL_JoystickRumble` respectively.

[PR #867](https://github.com/Rust-SDL2/rust-sdl2/pull/867):
Added `Window::opacity` and `Window::set_opacity`, wrappers for `SDL_GetWindowOpacity` and `SDL_SetWindowOpacity` respectively. This bumps the minimum `SDL2` version requirement from `2.0.4` to `2.0.5`.

### v0.32

[PR #790](https://github.com/Rust-SDL2/rust-sdl2/pull/790): Added missing `window_id` field to `Event::DropFile`

[PR #789](https://github.com/Rust-SDL2/rust-sdl2/pull/789): Audio Safety Fixes

[PR #785](https://github.com/Rust-SDL2/rust-sdl2/pull/785): Vulkan Support

[PR #782](https://github.com/Rust-SDL2/rust-sdl2/pull/782)
* Move ffi of features (mixer, ...) into `sys`
* Updated SDL2's default version to 2.0.8

[PR #780](https://github.com/Rust-SDL2/rust-sdl2/pull/780): Fixed a panic in `keyboard::Mod`

[PR #775](https://github.com/Rust-SDL2/rust-sdl2/pull/775): Added `get_platform`

[PR #774](https://github.com/Rust-SDL2/rust-sdl2/pull/774): `add_timer` is now must_use

[PR #764](https://github.com/Rust-SDL2/rust-sdl2/pull/764): impl `Hash` for `Point` and `Rect`

[PR #763](https://github.com/Rust-SDL2/rust-sdl2/pull/763): Allow `-sys` to build for `windows-gnu` target

[PR #751](https://github.com/Rust-SDL2/rust-sdl2/pull/751):
**Breaking change** `gl_setswap_interval` now returns a `Result` instead of a `bool`.

[PR #759](https://github.com/Rust-SDL2/rust-sdl2/pull/759): Expose Joystick power level

[PR #751](https://github.com/Rust-SDL2/rust-sdl2/pull/751)
* Fix memory leak in `filesystem::base_path()`
* Fix memory leak on `ClipboardUtil::clipboard_text()`

[PR #740](https://github.com/Rust-SDL2/rust-sdl2/pull/740): Implement Debug for Event

[PR #737](https://github.com/Rust-SDL2/rust-sdl2/pull/737):
Fix `ClipboardUtil::set_clipboard_text` to return an Ok when it went well.

[PR #733](https://github.com/Rust-SDL2/rust-sdl2/pull/733):
Add `video::border_size -> Result<(u16, u16, u16, u16), String>` equivalent of `SDL_GetWindowBorderSize()`

[PR #732](https://github.com/Rust-SDL2/rust-sdl2/pull/732):
Implemented `From<(u8, u8, u8)>` and `From<(u8, u8, u8, u8)>` for `pixels::Color`.
  `Canvas.set_draw_color` can now be called with tuples or other types which implements `Into<pixels::Color>`

[PR #279](https://github.com/Rust-SDL2/rust-sdl2/pull/729)

* **Breaking change** set\_video\_minimize\_on\_focus\_lost was renamed to …minimize\_on\_focus\_loss, as it should be. As a bonus, it works now.
* Although this is a breaking change, this function was not working in the first place, so this is hardly a true breaking change

### v0.31

[PR #693](https://github.com/Rust-SDL2/rust-sdl2/pull/693), [PR #720](https://github.com/Rust-SDL2/rust-sdl2/pull/720)

* Adds feature "bundled" as well as "static-link". Entirely compatible wit hthe "use-bindgen" feature. Windows-gnu is not supported yet, any help is needed on that side, but every other major platform should be supported.

[PR #711](https://github.com/Rust-SDL2/rust-sdl2/pull/711)

* **Breaking change** Change HatState::Leftdown casing into HatState::LeftDown to be consistent with naming.

[PR #695](https://github.com/Rust-sdl2/rust-sdl2/pull/695)

* sdl2-sys can now be generated at compile time by bindgen (Opt-in required
  with "use-bindgen" feature)
* The new sdl2-sys source code is generated by bindgen, hence **almost everything
  that uses sdl2-sys will be broken**. The fixes are small but still preset.

[PR #673](https://github.com/Rust-sdl2/rust-sdl2/pull/673)

* Support Audio Capture in AudioCallbacks.

[PR #684](https://github.com/Rust-sdl2/rust-sdl2/pull/684)

* **Breaking change** Make get\_swap\_interval return an enum instead of i32
* The signature of set\_swap\_interval has been changed as well, but it shouldn't
  breaking existing code too much.

[PR #683](https://github.com/Rust-sdl2/rust-sdl2/pull/683)

* Adds the `unsafe_textures` feature to this crate, allowing to get rid of the lifetimes
  in `Texture`s in the `render` module.

[PR #704](https://github.com/Rust-SDL2/rust-sdl2/pull/704)

* Adds the `Music::from_static_bytes` function, which creates a Music instance with the
  static lifetime from a buffer that also has a static lifetime.

[PR #708](https://github.com/Rust-SDL2/rust-sdl2/pull/708)

* Makes the fields of the `sdl2::mixer::Channel(i32)` and `sdl::mixer::Group(i32)` structs
  public so they can be instantiated directly, and deprecates `sdl2::mixer::channel(i32)`.

[PR #714](https://github.com/Rust-SDL2/rust-sdl2/pull/714)

* **Breaking change** Updates the `which` fields of `sdl2::Event::ControllerDeviceAdded` and `sdl2::Event::JoyDeviceAdded` to be `u32`s so they can be used with `sdl2::GameControllerSubsystem` and `sdl::JoystickSubsystem` methods directly.
* **Breaking change** Updates `sdl2::HapticSubsystem::open_from_joystick_id` to correctly advertise `joystick_index` as being a `u32`.
* This should only mean removing type conversions which were previously needed to use these values, or changing incorrect assumptions in existing code.

### v0.30

Re-exported sdl2\_sys as sdl2::sys

[PR #632](https://github.com/AngryLawyer/rust-sdl2/pull/632)

The path of soundness isn't an easy one; sometimes ease of use has to be sacrificed
for soundness and safety! This change is a breaking change, and a **huge** one at that.

**You will probably have to refactor some parts of your code**, but this is the price to
pay for soundness and runtime safety.

**Breaking Changes:**

* `Renderer` has been renamed and split into `Canvas` and `TextureCreator`.
* `Canvas` can store a `Surface` or a `Window`, and can be used to render into these as well.
  `TextureCreator` creates `Texture`s and is used by Texture to make sure they don't live
  longer than expected.
* `set_render_target` has been removed and has been replaced with `Canvas::with_texture_canvas`
  and `Canvas::with_multiple_texture_canvas`
* Deleted `WindowRef`, it wasn't useful anymore.

Other Changes:

* Added `PixelFormatEnum::supports_alpha(&self) -> bool` method.
* A single Game Of Life example has been added to show the basic capabilities of the new `Canvas`
  and `TextureCreator` structs, as well as adding a very basic game to show how to handle input / game
  changes in a basic game.

You won't have to worry about what target your `Renderer` has at runtime anymore, everything
is done at compile time now !

[PR #628](https://github.com/AngryLawyer/rust-sdl2/pull/628)

* Changed signature of `Surface::fill_rects` ([old](https://docs.rs/sdl2/0.29.1/sdl2/surface/struct.SurfaceRef.html#method.fill_rects) | [new](https://docs.rs/sdl2/0.30.0/sdl2/surface/struct.SurfaceRef.html#method.fill_rects))
* Changed various `Option<T>` parameters  into `Into<Option<T>>` parameters. For instance, it is now possible to do this:

```rust
surface.blit(None,Rect::new(5,5,5,5)); // instead of surface.blit(None,Some(Rect::new(5,5,5,5)));
```

[PR #639](https://github.com/AngryLawyer/rust-sdl2/pull/639)

* Added hint-specific functions to `sdl2::hint`

```rust
sdl2::hint::set_video_minimize_on_focus_lost(bool) -> bool;
sdl2::hint::set_video_minimize_on_focus_lost_with_priority(bool, sdl2::hint::Hint) -> bool;
sdl2::hint::get_video_minimize_on_focus_lost() -> bool;
```

[PR #629](https://github.com/AngryLawyer/rust-sdl2/pull/629)

* **Breaking Change: Changed Color to be a struct rather than an enum.**
* Takes less space, easier to use, old constructors are still available.
* Matching is no longer necessary to read the component values.
* Struct rather than variant construction is required in static initializers.

```rust
let color = Color { r: 255, g: 0, b: 0, a: 255 };
let color = Color::RGBA(255, 0, 0, 255);
let color = Color::RGB(255, 0, 0);
let (r, g, b) = color.rgb();
let (r, g, b, a) = color.rgba();
```

[PR #651](https://github.com/AngryLawyer/rust-sdl2/pull/629)

* **VideoSystem::display_name** now returns a `Result<String, String>` instead of a `String`.
* This prevents a segfault when the requested display index is out of bounds

[commit e9681a0fe](https://github.com/AngryLawyer/rust-sdl2/commit/e9681a0fe)

* `window.surface()` has been changed and is now more intuitive to use.
* The example in `examples/no-renderer.rs` shows how to use this feature.

[PR #635](https://github.com/AngryLawyer/rust-sdl2/pull/635)

* **Deprecated `Rect::contains`**, added `Rect::contains_point` and `Rect::contains_rect`. **`contains` and `contains_point` are close but different ! See [here](https://github.com/AngryLawyer/rust-sdl2/issues/569)**
