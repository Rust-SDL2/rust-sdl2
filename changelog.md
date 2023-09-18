In this file will be listed the changes, especially the breaking ones that one should be careful of
when upgrading from a version of rust-sdl2 to another.

### Unreleased

[PR #1270](https://github.com/Rust-SDL2/rust-sdl2/pull/1270) **BREAKING CHANGE** Remove &mut self requirement in `TimerSubsystem::delay`; Add `TimerSubsystem::ticks64`

[PR #1225](https://github.com/Rust-SDL2/rust-sdl2/pull/1225) Update wgpu to 0.12 and fix raw-window-handle-with-wgpu example
[PR #1250](https://github.com/Rust-SDL2/rust-sdl2/pull/1250) Add `lib64` to native library search path when using bundled feature

[PR #1240](https://github.com/Rust-SDL2/rust-sdl2/pull/1240) **BREAKING CHANGE** Take `PixelMasks` by refrence

[PR #1254](https://github.com/Rust-SDL2/rust-sdl2/pull/1254) **BREAKING CHANGE** Make `SdlDrop` and `SubsystemDrop` safer; forbid external code from constructing `SdlDrop`

[PR #1318](https://github.com/Rust-SDL2/rust-sdl2/pull/1318) Add NV12, NV21 to PixelFormatEnum

[PR #1332](https://github.com/Rust-SDL2/rust-sdl2/pull/1332) Fix `size_hint` implementations for `{audio,video,render}::DriverIterator`

[PR #1333](https://github.com/Rust-SDL2/rust-sdl2/pull/1333) Implement `FusedIterator`, `DoubleEndedIterator`, `and nth[_back]` for `{audio,video,render}::DriverIterator`

[PR #1337](https://github.com/Rust-SDL2/rust-sdl2/pull/1337) Fix "Cannot initialize Sdl from more than one thread" for tests / CI

### v0.35.2

[PR #1173](https://github.com/Rust-SDL2/rust-sdl2/pull/1173) Fix segfault when using timer callbacks

[PR #1183](https://github.com/Rust-SDL2/rust-sdl2/pull/1183) WinRT support for raw-window-handle

[PR #1182](https://github.com/Rust-SDL2/rust-sdl2/pull/1182) Updated raw-window-handle to 0.4

[PR #1189](https://github.com/Rust-SDL2/rust-sdl2/pull/1189) Added `AudioQueue::queue_audio` and deprecated `AudioQueue::queue`

[PR #1164](https://github.com/Rust-SDL2/rust-sdl2/pull/1164) Added raw-window-handle support for Android

[PR #1165](https://github.com/Rust-SDL2/rust-sdl2/pull/1165) Added binding for `SDL_GetDisplayOrientation` and `SDL_DISPLAYEVENT`

### v0.35.0

* **BREAKING CHANGE** Update `sdl2-sys/sdl_bindings.rs` to use enums instead of consts. If you were using `sdl2-sys`'s
enum variants directly in your project, you may be affected. If you only used sdl2 calls, there should not be any problems.

* **BREAKING CHANGE** SDL 2.0.14 or higher is now recommended due to the new binding being added for `SDL_OpenURL`. If you get linking errors, upgrade your SDL2 libraries, or swap to using the `bundled` feature.

[PR #1138](https://github.com/Rust-SDL2/rust-sdl2/pull/1138) Added binding for `SDL_OpenURL`

[PR #1150](https://github.com/Rust-SDL2/rust-sdl2/pull/1150) Do not download SDL2 sources when using bundled feature

[PR #1112](https://github.com/Rust-SDL2/rust-sdl2/pull/1112) Add wrapper functions for `SDL_RenderSetIntegerScale` and `SDL_RenderGetIntegerScale`

[PR #1156](https://github.com/Rust-SDL2/rust-sdl2/pull/1156) **Maybe breaking change**: new variants to enum `GameController`, and prevent panic with unrecognized buttons.

[PR #1153](https://github.com/Rust-SDL2/rust-sdl2/pull/1153) `SDL_GL_GetCurrentContext` and `SDL_RenderFlush` added.

[PR #1131](https://github.com/Rust-SDL2/rust-sdl2/pull/1131) Added Sensor API.

### v0.34.5

[PR #1100](https://github.com/Rust-SDL2/rust-sdl2/pull/1100) Added binding for `SDL_GetDisplayUsableBounds`

[PR #1102](https://github.com/Rust-SDL2/rust-sdl2/pull/1102) Correctly se linux and macSO built libraries when using bundled without static-link.

[PR #1098](https://github.com/Rust-SDL2/rust-sdl2/pull/1098) Fix potential heap corruption when using AudioCVT::convert

[PR #1088](https://github.com/Rust-SDL2/rust-sdl2/pull/1088) Rollback of PR #1081: Broke dynamic linking on Windows

Various fixes to CI.

### v0.34.4

[PR #1086](https://github.com/Rust-SDL2/rust-sdl2/pull/1086) Update bundled to use SDL2 2.0.14

[PR #1033](https://github.com/Rust-SDL2/rust-sdl2/pull/1033) Changed signature of TimerSubsystem::ticks to accept `&self`.

[PR #1057](https://github.com/Rust-SDL2/rust-sdl2/pull/1057): fix memory safety bug in set_error

[PR #1081](https://github.com/Rust-SDL2/rust-sdl2/pull/1081): Allow bundled build to be built in debug mode.  Fixes issue when linking binary with mixed debug+release CRT dependencies.

[PR #1080](https://github.com/Rust-SDL2/rust-sdl2/pull/1080): Fix line endings of patches to lf so patching of sources works on Windows.

[PR #1031](https://github.com/Rust-SDL2/rust-sdl2/pull/1031): Add patch to fix metal detection (https://bugzilla.libsdl.org/show_bug.cgi?id=4988)

### v0.34.3

[PR #1027](https://github.com/Rust-SDL2/rust-sdl2/pull/1027): upgrade "bundled" version of SDL2 to 2.0.12
[PR #1020](https://github.com/Rust-SDL2/rust-sdl2/pull/1020): revert undefined behavior introduction.
[Commit](https://github.com/Rust-SDL2/rust-sdl2/commit/9d1851b7dfa53168d22c5c17ca941088e9ab4b34): Fix clippy issue
[Commit](https://github.com/Rust-SDL2/rust-sdl2/commit/eaa01c597e08f08962b47a862dfdc2b5a0f63b5d): Temporarily suppress dangerous clippy hints
[PR#1014](https://github.com/Rust-SDL2/rust-sdl2/pull/1014): add `load_texture_bytes` interface for load texture from buffer

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
