In this file will be listed the changes, especially the breaking ones that one should be careful of
when upgrading from a version of rust-sdl2 to another.

### v0.31

[PR #673](https://github.com/Rust-sdl2/rust-sdl2/pull/673)

* Support Audio Capture in AudioCallbacks.

[PR #684](https://github.com/Rust-sdl2/rust-sdl2/pull/684)

* **Breaking change** Make get\_swap\_interval return an enum instead of i32
* The signature of set\_swap\_interval has been changed as well, but it shouldn't
breaking exisitng code too much.

[PR #683](https://github.com/Rust-sdl2/rust-sdl2/pull/684)

* Adds the `unsafe_textures` feature to this crate, allowing to get rid of the lifetimes
in `Texture`s in the `render` module.

[PR #704](https://github.com/Rust-SDL2/rust-sdl2/pull/704)

* Adds the `Music::from_static_bytes` function, which creates a Music instance with the
static lifetime from a buffer that also has a static lifetime.

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
