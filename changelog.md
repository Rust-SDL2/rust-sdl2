In this file will be listed the changes, especially the breaking ones that one should be careful of
when upgrading from a version of rust-sdl2 to another.

### v0.30

[PR #628](https://github.com/AngryLawyer/rust-sdl2/pull/628)

* Changed signature of `Surface::fill_rects` ([old](https://docs.rs/sdl2/0.29.1/sdl2/surface/struct.SurfaceRef.html#method.fill_rects) | [new](https://docs.rs/sdl2/0.30.0/sdl2/surface/struct.SurfaceRef.html#method.fill_rects))
* Changed various `Option<T>` parameters  into `Into<Option<T>>` parameters. For instance, it is now possible to do this:

```rust
surface.blit(None,Rect::new(5,5,5,5)); // instead of surface.blit(None,Some(Rect::new(5,5,5,5)));
```
