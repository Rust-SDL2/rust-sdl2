pub mod ll {

    use std::libc::{c_int, c_char};
    use std::libc::{uint32_t};
    //SDL_render.h
    pub enum SDL_RendererFlags {
        SDL_RENDERER_SOFTWARE = 0x00000001,
        SDL_RENDERER_ACCELERATED = 0x00000002,
        SDL_RENDERER_PRESENTVSYNC = 0x00000004,
        SDL_RENDERER_TARGETTEXTURE = 0x00000008
    }

    pub struct SDL_RendererInfo
    {
        name: *c_char,
        flags: uint32_t,
        num_texture_formats: uint32_t,
        texture_formats: [uint32_t, ..16],
        max_texture_width: c_int,
        max_texture_height: c_int,
    }

    pub enum SDL_TextureAccess {
        SDL_TEXTUREACCESS_STATIC,
        SDL_TEXTUREACCESS_STREAMING,
        SDL_TEXTUREACCESS_TARGET
    }

    pub enum SDL_TextureModulate {
        SDL_TEXTUREMODULATE_NONE = 0x00000000,
        SDL_TEXTUREMODULATE_COLOR = 0x00000001,
        SDL_TEXTUREMODULATE_ALPHA = 0x00000002
    }

    pub enum SDL_RendererFlip {
        SDL_FLIP_NONE = 0x00000000,
        SDL_FLIP_HORIZONTAL = 0x00000001,
        SDL_FLIP_VERTICAL = 0x00000002
    }

    pub struct SDL_Renderer;
    pub struct SDL_Texture;
}
