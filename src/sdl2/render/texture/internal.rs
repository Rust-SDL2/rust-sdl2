use std::intrinsics::transmute;
use std::mem::{self, MaybeUninit};
use std::ptr;

use crate::common::validate_int;
use crate::get_error;
use crate::pixels::PixelFormatEnum;
use crate::rect::Rect;
use crate::render::{
    BlendMode, TextureAccess, TextureQuery, UpdateTextureError, UpdateTextureYUVError,
};
use crate::sys::SDL_BlendMode;

pub struct InternalTexture {
    raw: *mut sys::SDL_Texture,
}

impl InternalTexture {
    pub(super) fn new(raw: *mut sys::SDL_Texture) -> Self {
        Self { raw }
    }

    #[doc(alias = "SDL_QueryTexture")]
    pub fn query(&self) -> TextureQuery {
        let mut format = 0;
        let mut access = 0;
        let mut width = 0;
        let mut height = 0;

        let ret = unsafe {
            sys::SDL_QueryTexture(self.raw, &mut format, &mut access, &mut width, &mut height)
        };
        // Should only fail on an invalid texture
        if ret != 0 {
            panic!(get_error())
        } else {
            use std::convert::TryFrom;
            TextureQuery {
                format: PixelFormatEnum::try_from(format as u32).unwrap(),
                access: TextureAccess::try_from(access as u32).unwrap(),
                width: width as u32,
                height: height as u32,
            }
        }
    }

    #[doc(alias = "SDL_SetTextureColorMod")]
    pub fn set_color_mod(&mut self, red: u8, green: u8, blue: u8) {
        let ret = unsafe { sys::SDL_SetTextureColorMod(self.raw, red, green, blue) };

        if ret != 0 {
            panic!("Error setting color mod: {}", get_error())
        }
    }

    #[doc(alias = "SDL_GetTextureColorMod")]
    pub fn color_mod(&self) -> (u8, u8, u8) {
        let (mut r, mut g, mut b) = (0, 0, 0);
        let ret = unsafe { sys::SDL_GetTextureColorMod(self.raw, &mut r, &mut g, &mut b) };

        // Should only fail on an invalid texture
        if ret != 0 {
            panic!(get_error())
        } else {
            (r, g, b)
        }
    }

    #[doc(alias = "SDL_SetTextureAlphaMod")]
    pub fn set_alpha_mod(&mut self, alpha: u8) {
        let ret = unsafe { sys::SDL_SetTextureAlphaMod(self.raw, alpha) };

        if ret != 0 {
            panic!("Error setting alpha mod: {}", get_error())
        }
    }

    #[doc(alias = "SDL_GetTextureAlphaMod")]
    pub fn alpha_mod(&self) -> u8 {
        let mut alpha = 0;
        let ret = unsafe { sys::SDL_GetTextureAlphaMod(self.raw, &mut alpha) };

        // Should only fail on an invalid texture
        if ret != 0 {
            panic!(get_error())
        } else {
            alpha
        }
    }

    #[doc(alias = "SDL_SetTextureBlendMode")]
    pub fn set_blend_mode(&mut self, blend: BlendMode) {
        let ret = unsafe { sys::SDL_SetTextureBlendMode(self.raw, transmute(blend as u32)) };

        if ret != 0 {
            panic!("Error setting blend: {}", get_error())
        }
    }

    #[doc(alias = "SDL_GetTextureBlendMode")]
    pub fn blend_mode(&self) -> BlendMode {
        let mut blend: MaybeUninit<SDL_BlendMode> = mem::MaybeUninit::uninit();
        let ret = unsafe { sys::SDL_GetTextureBlendMode(self.raw, blend.as_mut_ptr()) };

        // Should only fail on an invalid texture
        if ret != 0 {
            panic!(get_error())
        } else {
            use std::convert::TryFrom;
            let blend = unsafe { blend.assume_init() };
            BlendMode::try_from(blend as u32).unwrap()
        }
    }

    #[doc(alias = "SDL_UpdateTexture")]
    pub fn update<R>(
        &mut self,
        rect: R,
        pixel_data: &[u8],
        pitch: usize,
    ) -> Result<(), UpdateTextureError>
    where
        R: Into<Option<Rect>>,
    {
        use self::UpdateTextureError::*;
        let rect = rect.into();
        let rect_raw_ptr = match rect {
            Some(ref rect) => rect.raw(),
            None => ptr::null(),
        };

        // Check if the rectangle's position or size is odd, and if the pitch is odd.
        // This needs to be done in case the texture's pixel format is planar YUV.
        // See issue #334 for details.
        let TextureQuery { format, .. } = self.query();
        match format {
            PixelFormatEnum::YV12 | PixelFormatEnum::IYUV => {
                if let Some(r) = rect {
                    if r.x() % 2 != 0 {
                        return Err(XMustBeMultipleOfTwoForFormat(r.x(), format));
                    } else if r.y() % 2 != 0 {
                        return Err(YMustBeMultipleOfTwoForFormat(r.y(), format));
                    } else if r.width() % 2 != 0 {
                        return Err(WidthMustBeMultipleOfTwoForFormat(r.width(), format));
                    } else if r.height() % 2 != 0 {
                        return Err(HeightMustBeMultipleOfTwoForFormat(r.height(), format));
                    }
                };
                if pitch % 2 != 0 {
                    return Err(PitchMustBeMultipleOfTwoForFormat(pitch, format));
                }
            }
            _ => {}
        }

        let pitch = match validate_int(pitch as u32, "pitch") {
            Ok(p) => p,
            Err(_) => return Err(PitchOverflows(pitch)),
        };

        let result = unsafe {
            sys::SDL_UpdateTexture(
                self.raw,
                rect_raw_ptr,
                pixel_data.as_ptr() as *const _,
                pitch,
            )
        };

        if result != 0 {
            Err(SdlError(get_error()))
        } else {
            Ok(())
        }
    }

    #[doc(alias = "SDL_UpdateYUVTexture")]
    pub fn update_yuv<R>(
        &mut self,
        rect: R,
        y_plane: &[u8],
        y_pitch: usize,
        u_plane: &[u8],
        u_pitch: usize,
        v_plane: &[u8],
        v_pitch: usize,
    ) -> Result<(), UpdateTextureYUVError>
    where
        R: Into<Option<Rect>>,
    {
        use self::UpdateTextureYUVError::*;

        let rect = rect.into();

        let rect_raw_ptr = match rect {
            Some(ref rect) => rect.raw(),
            None => ptr::null(),
        };

        if let Some(ref r) = rect {
            if r.x() % 2 != 0 {
                return Err(XMustBeMultipleOfTwoForFormat(r.x()));
            } else if r.y() % 2 != 0 {
                return Err(YMustBeMultipleOfTwoForFormat(r.y()));
            } else if r.width() % 2 != 0 {
                return Err(WidthMustBeMultipleOfTwoForFormat(r.width()));
            } else if r.height() % 2 != 0 {
                return Err(HeightMustBeMultipleOfTwoForFormat(r.height()));
            }
        };

        // If the destination rectangle lies outside the texture boundaries,
        // SDL_UpdateYUVTexture will write outside allocated texture memory.
        let tex_info = self.query();
        if let Some(ref r) = rect {
            let tex_rect = Rect::new(0, 0, tex_info.width, tex_info.height);
            let inside = match r.intersection(tex_rect) {
                Some(intersection) => intersection == *r,
                None => false,
            };
            // The destination rectangle cannot lie outside the texture boundaries
            if !inside {
                return Err(RectNotInsideTexture(*r));
            }
        }

        // We need the height in order to check the array slice lengths.
        // Checking the lengths can prevent buffer overruns in SDL_UpdateYUVTexture.
        let height = match rect {
            Some(ref r) => r.height(),
            None => tex_info.height,
        } as usize;

        //let wrong_length =
        if y_plane.len() != (y_pitch * height) {
            return Err(InvalidPlaneLength {
                plane: "y",
                length: y_plane.len(),
                pitch: y_pitch,
                height,
            });
        }
        if u_plane.len() != (u_pitch * height / 2) {
            return Err(InvalidPlaneLength {
                plane: "u",
                length: u_plane.len(),
                pitch: u_pitch,
                height: height / 2,
            });
        }
        if v_plane.len() != (v_pitch * height / 2) {
            return Err(InvalidPlaneLength {
                plane: "v",
                length: v_plane.len(),
                pitch: v_pitch,
                height: height / 2,
            });
        }

        let y_pitch = match validate_int(y_pitch as u32, "y_pitch") {
            Ok(p) => p,
            Err(_) => {
                return Err(PitchOverflows {
                    plane: "y",
                    value: y_pitch,
                })
            }
        };
        let u_pitch = match validate_int(u_pitch as u32, "u_pitch") {
            Ok(p) => p,
            Err(_) => {
                return Err(PitchOverflows {
                    plane: "u",
                    value: u_pitch,
                })
            }
        };
        let v_pitch = match validate_int(v_pitch as u32, "v_pitch") {
            Ok(p) => p,
            Err(_) => {
                return Err(PitchOverflows {
                    plane: "v",
                    value: v_pitch,
                })
            }
        };

        let result = unsafe {
            sys::SDL_UpdateYUVTexture(
                self.raw,
                rect_raw_ptr,
                y_plane.as_ptr(),
                y_pitch,
                u_plane.as_ptr(),
                u_pitch,
                v_plane.as_ptr(),
                v_pitch,
            )
        };
        if result != 0 {
            Err(SdlError(get_error()))
        } else {
            Ok(())
        }
    }

    #[doc(alias = "SDL_LockTexture")]
    pub fn with_lock<F, R, R2>(&mut self, rect: R2, func: F) -> Result<R, String>
    where
        F: FnOnce(&mut [u8], usize) -> R,
        R2: Into<Option<Rect>>,
    {
        // Call to SDL to populate pixel data
        let loaded = unsafe {
            let q = self.query();
            let mut pixels = ptr::null_mut();
            let mut pitch = 0;

            let (rect_raw_ptr, height) = match rect.into() {
                Some(ref rect) => (rect.raw(), rect.height() as usize),
                None => (ptr::null(), q.height as usize),
            };

            let ret = sys::SDL_LockTexture(self.raw, rect_raw_ptr, &mut pixels, &mut pitch);
            if ret == 0 {
                let size = q
                    .format
                    .byte_size_from_pitch_and_height(pitch as usize, height);
                Ok((
                    ::std::slice::from_raw_parts_mut(pixels as *mut u8, size),
                    pitch,
                ))
            } else {
                Err(get_error())
            }
        };

        match loaded {
            Ok((interior, pitch)) => {
                let result;
                unsafe {
                    result = func(interior, pitch as usize);
                    sys::SDL_UnlockTexture(self.raw);
                }
                Ok(result)
            }
            Err(e) => Err(e),
        }
    }

    pub unsafe fn gl_bind_texture(&mut self) -> (f32, f32) {
        let mut texw = 0.0;
        let mut texh = 0.0;

        if sys::SDL_GL_BindTexture(self.raw, &mut texw, &mut texh) == 0 {
            (texw, texh)
        } else {
            panic!("OpenGL texture binding not supported");
        }
    }

    pub unsafe fn gl_unbind_texture(&mut self) {
        if sys::SDL_GL_UnbindTexture(self.raw) != 0 {
            panic!("OpenGL texture unbinding not supported");
        }
    }

    #[doc(alias = "SDL_GL_BindTexture")]
    pub fn gl_with_bind<R, F: FnOnce(f32, f32) -> R>(&mut self, f: F) -> R {
        unsafe {
            let mut texw = 0.0;
            let mut texh = 0.0;

            if sys::SDL_GL_BindTexture(self.raw, &mut texw, &mut texh) == 0 {
                let return_value = f(texw, texh);

                if sys::SDL_GL_UnbindTexture(self.raw) == 0 {
                    return_value
                } else {
                    // This should never happen...
                    panic!();
                }
            } else {
                panic!("OpenGL texture binding not supported");
            }
        }
    }
}
