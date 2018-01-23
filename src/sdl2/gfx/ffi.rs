
use libc::*;
use sys::SDL_Renderer;
use sys::SDL_Surface;

#[repr(C)]
pub struct FPSmanager {
    pub framecount: uint32_t,
    pub rateticks: c_float,
    pub baseticks: uint32_t,
    pub lastticks: uint32_t,
    pub rate: uint32_t,
}

extern "C" {
    pub fn SDL_initFramerate(manager: *mut FPSmanager);
    pub fn SDL_setFramerate(manager: *mut FPSmanager, rate: uint32_t) -> c_int;
    pub fn SDL_getFramerate(manager: *mut FPSmanager) -> c_int;
    pub fn SDL_getFramecount(manager: *mut FPSmanager) -> c_int;
    pub fn SDL_framerateDelay(manager: *mut FPSmanager) -> uint32_t;
}

extern "C" {
    pub fn SDL_imageFilterMMXdetect() -> c_int;
    pub fn SDL_imageFilterMMXoff();
    pub fn SDL_imageFilterMMXon();
    pub fn SDL_imageFilterAdd(Src1: *mut u8, Src2: *mut u8,
                                Dest: *mut u8, length: c_uint) -> c_int;
    pub fn SDL_imageFilterMean(Src1: *mut u8, Src2: *mut u8,
                                Dest: *mut u8, length: c_uint) -> c_int;
    pub fn SDL_imageFilterSub(Src1: *mut u8, Src2: *mut u8,
                                Dest: *mut u8, length: c_uint) -> c_int;
    pub fn SDL_imageFilterAbsDiff(Src1: *mut u8, Src2: *mut u8,
                                    Dest: *mut u8, length: c_uint) ->
        c_int;
    pub fn SDL_imageFilterMult(Src1: *mut u8, Src2: *mut u8,
                                Dest: *mut u8, length: c_uint) -> c_int;
    pub fn SDL_imageFilterMultNor(Src1: *mut u8, Src2: *mut u8,
                                    Dest: *mut u8, length: c_uint) ->
        c_int;
    pub fn SDL_imageFilterMultDivby2(Src1: *mut u8, Src2: *mut u8,
                                        Dest: *mut u8, length: c_uint) ->
        c_int;
    pub fn SDL_imageFilterMultDivby4(Src1: *mut u8, Src2: *mut u8,
                                        Dest: *mut u8, length: c_uint) ->
        c_int;
    pub fn SDL_imageFilterBitAnd(Src1: *mut u8, Src2: *mut u8,
                                    Dest: *mut u8, length: c_uint) -> c_int;
    pub fn SDL_imageFilterBitOr(Src1: *mut u8, Src2: *mut u8,
                                Dest: *mut u8, length: c_uint) -> c_int;
    pub fn SDL_imageFilterDiv(Src1: *mut u8, Src2: *mut u8,
                                Dest: *mut u8, length: c_uint) -> c_int;
    pub fn SDL_imageFilterBitNegation(Src1: *mut u8, Dest: *mut u8,
                                        length: c_uint) -> c_int;
    pub fn SDL_imageFilterAddByte(Src1: *mut u8, Dest: *mut u8,
                                    length: c_uint, C: u8) -> c_int;
    pub fn SDL_imageFilterAddUint(Src1: *mut u8, Dest: *mut u8,
                                    length: c_uint, C: c_uint) -> c_int;
    pub fn SDL_imageFilterAddByteToHalf(Src1: *mut u8,
                                        Dest: *mut u8, length: c_uint,
                                        C: u8) -> c_int;
    pub fn SDL_imageFilterSubByte(Src1: *mut u8, Dest: *mut u8,
                                    length: c_uint, C: u8) -> c_int;
    pub fn SDL_imageFilterSubUint(Src1: *mut u8, Dest: *mut u8,
                                    length: c_uint, C: c_uint) -> c_int;
    pub fn SDL_imageFilterShiftRight(Src1: *mut u8, Dest: *mut u8,
                                        length: c_uint, N: u8) -> c_int;
    pub fn SDL_imageFilterShiftRightUint(Src1: *mut u8,
                                            Dest: *mut u8, length: c_uint,
                                            N: u8) -> c_int;
    pub fn SDL_imageFilterMultByByte(Src1: *mut u8, Dest: *mut u8,
                                        length: c_uint, C: u8) -> c_int;
    pub fn SDL_imageFilterShiftRightAndMultByByte(Src1: *mut u8,
                                                    Dest: *mut u8,
                                                    length: c_uint, N: u8,
                                                    C: u8) -> c_int;
    pub fn SDL_imageFilterShiftLeftByte(Src1: *mut u8,
                                        Dest: *mut u8, length: c_uint,
                                        N: u8) -> c_int;
    pub fn SDL_imageFilterShiftLeftUint(Src1: *mut u8,
                                        Dest: *mut u8, length: c_uint,
                                        N: u8) -> c_int;
    pub fn SDL_imageFilterShiftLeft(Src1: *mut u8, Dest: *mut u8,
                                    length: c_uint, N: u8) -> c_int;
    pub fn SDL_imageFilterBinarizeUsingThreshold(Src1: *mut u8,
                                                    Dest: *mut u8,
                                                    length: c_uint, T: u8)
                                                    -> c_int;
    pub fn SDL_imageFilterClipToRange(Src1: *mut u8, Dest: *mut u8,
                                        length: c_uint, Tmin: u8,
                                        Tmax: u8) -> c_int;
    pub fn SDL_imageFilterNormalizeLinear(Src: *mut u8,
                                            Dest: *mut u8, length: c_uint,
                                            Cmin: c_int, Cmax: c_int,
                                            Nmin: c_int, Nmax: c_int) -> c_int;
}

extern "C" {
    pub fn pixelColor(renderer: *const SDL_Renderer,
                        x: int16_t,
                        y: int16_t,
                        color: uint32_t)
                        -> c_int;
    pub fn pixelRGBA(renderer: *const SDL_Renderer,
                        x: int16_t,
                        y: int16_t,
                        r: uint8_t,
                        g: uint8_t,
                        b: uint8_t,
                        a: uint8_t)
                        -> c_int;
    pub fn hlineColor(renderer: *const SDL_Renderer,
                        x1: int16_t,
                        x2: int16_t,
                        y: int16_t,
                        color: uint32_t)
                        -> c_int;
    pub fn hlineRGBA(renderer: *const SDL_Renderer,
                        x1: int16_t,
                        x2: int16_t,
                        y: int16_t,
                        r: uint8_t,
                        g: uint8_t,
                        b: uint8_t,
                        a: uint8_t)
                        -> c_int;
    pub fn vlineColor(renderer: *const SDL_Renderer,
                        x: int16_t,
                        y1: int16_t,
                        y2: int16_t,
                        color: uint32_t)
                        -> c_int;
    pub fn vlineRGBA(renderer: *const SDL_Renderer,
                        x: int16_t,
                        y1: int16_t,
                        y2: int16_t,
                        r: uint8_t,
                        g: uint8_t,
                        b: uint8_t,
                        a: uint8_t)
                        -> c_int;
    pub fn rectangleColor(renderer: *const SDL_Renderer,
                            x1: int16_t,
                            y1: int16_t,
                            x2: int16_t,
                            y2: int16_t,
                            color: uint32_t)
                            -> c_int;
    pub fn rectangleRGBA(renderer: *const SDL_Renderer,
                            x1: int16_t,
                            y1: int16_t,
                            x2: int16_t,
                            y2: int16_t,
                            r: uint8_t,
                            g: uint8_t,
                            b: uint8_t,
                            a: uint8_t)
                            -> c_int;
    pub fn roundedRectangleColor(renderer: *const SDL_Renderer,
                                    x1: int16_t,
                                    y1: int16_t,
                                    x2: int16_t,
                                    y2: int16_t,
                                    rad: int16_t,
                                    color: uint32_t)
                                    -> c_int;
    pub fn roundedRectangleRGBA(renderer: *const SDL_Renderer,
                                x1: int16_t,
                                y1: int16_t,
                                x2: int16_t,
                                y2: int16_t,
                                rad: int16_t,
                                r: uint8_t,
                                g: uint8_t,
                                b: uint8_t,
                                a: uint8_t)
                                -> c_int;
    pub fn boxColor(renderer: *const SDL_Renderer,
                    x1: int16_t,
                    y1: int16_t,
                    x2: int16_t,
                    y2: int16_t,
                    color: uint32_t)
                    -> c_int;
    pub fn boxRGBA(renderer: *const SDL_Renderer,
                    x1: int16_t,
                    y1: int16_t,
                    x2: int16_t,
                    y2: int16_t,
                    r: uint8_t,
                    g: uint8_t,
                    b: uint8_t,
                    a: uint8_t)
                    -> c_int;
    pub fn roundedBoxColor(renderer: *const SDL_Renderer,
                            x1: int16_t,
                            y1: int16_t,
                            x2: int16_t,
                            y2: int16_t,
                            rad: int16_t,
                            color: uint32_t)
                            -> c_int;
    pub fn roundedBoxRGBA(renderer: *const SDL_Renderer,
                            x1: int16_t,
                            y1: int16_t,
                            x2: int16_t,
                            y2: int16_t,
                            rad: int16_t,
                            r: uint8_t,
                            g: uint8_t,
                            b: uint8_t,
                            a: uint8_t)
                            -> c_int;
    pub fn lineColor(renderer: *const SDL_Renderer,
                        x1: int16_t,
                        y1: int16_t,
                        x2: int16_t,
                        y2: int16_t,
                        color: uint32_t)
                        -> c_int;
    pub fn lineRGBA(renderer: *const SDL_Renderer,
                    x1: int16_t,
                    y1: int16_t,
                    x2: int16_t,
                    y2: int16_t,
                    r: uint8_t,
                    g: uint8_t,
                    b: uint8_t,
                    a: uint8_t)
                    -> c_int;
    pub fn aalineColor(renderer: *const SDL_Renderer,
                        x1: int16_t,
                        y1: int16_t,
                        x2: int16_t,
                        y2: int16_t,
                        color: uint32_t)
                        -> c_int;
    pub fn aalineRGBA(renderer: *const SDL_Renderer,
                        x1: int16_t,
                        y1: int16_t,
                        x2: int16_t,
                        y2: int16_t,
                        r: uint8_t,
                        g: uint8_t,
                        b: uint8_t,
                        a: uint8_t)
                        -> c_int;
    pub fn thickLineColor(renderer: *const SDL_Renderer,
                            x1: int16_t,
                            y1: int16_t,
                            x2: int16_t,
                            y2: int16_t,
                            width: uint8_t,
                            color: uint32_t)
                            -> c_int;
    pub fn thickLineRGBA(renderer: *const SDL_Renderer,
                            x1: int16_t,
                            y1: int16_t,
                            x2: int16_t,
                            y2: int16_t,
                            width: uint8_t,
                            r: uint8_t,
                            g: uint8_t,
                            b: uint8_t,
                            a: uint8_t)
                            -> c_int;
    pub fn circleColor(renderer: *const SDL_Renderer,
                        x: int16_t,
                        y: int16_t,
                        rad: int16_t,
                        color: uint32_t)
                        -> c_int;
    pub fn circleRGBA(renderer: *const SDL_Renderer,
                        x: int16_t,
                        y: int16_t,
                        rad: int16_t,
                        r: uint8_t,
                        g: uint8_t,
                        b: uint8_t,
                        a: uint8_t)
                        -> c_int;
    pub fn arcColor(renderer: *const SDL_Renderer,
                    x: int16_t,
                    y: int16_t,
                    rad: int16_t,
                    start: int16_t,
                    end: int16_t,
                    color: uint32_t)
                    -> c_int;
    pub fn arcRGBA(renderer: *const SDL_Renderer,
                    x: int16_t,
                    y: int16_t,
                    rad: int16_t,
                    start: int16_t,
                    end: int16_t,
                    r: uint8_t,
                    g: uint8_t,
                    b: uint8_t,
                    a: uint8_t)
                    -> c_int;
    pub fn aacircleColor(renderer: *const SDL_Renderer,
                            x: int16_t,
                            y: int16_t,
                            rad: int16_t,
                            color: uint32_t)
                            -> c_int;
    pub fn aacircleRGBA(renderer: *const SDL_Renderer,
                        x: int16_t,
                        y: int16_t,
                        rad: int16_t,
                        r: uint8_t,
                        g: uint8_t,
                        b: uint8_t,
                        a: uint8_t)
                        -> c_int;
    pub fn filledCircleColor(renderer: *const SDL_Renderer,
                                x: int16_t,
                                y: int16_t,
                                r: int16_t,
                                color: uint32_t)
                                -> c_int;
    pub fn filledCircleRGBA(renderer: *const SDL_Renderer,
                            x: int16_t,
                            y: int16_t,
                            rad: int16_t,
                            r: uint8_t,
                            g: uint8_t,
                            b: uint8_t,
                            a: uint8_t)
                            -> c_int;
    pub fn ellipseColor(renderer: *const SDL_Renderer,
                        x: int16_t,
                        y: int16_t,
                        rx: int16_t,
                        ry: int16_t,
                        color: uint32_t)
                        -> c_int;
    pub fn ellipseRGBA(renderer: *const SDL_Renderer,
                        x: int16_t,
                        y: int16_t,
                        rx: int16_t,
                        ry: int16_t,
                        r: uint8_t,
                        g: uint8_t,
                        b: uint8_t,
                        a: uint8_t)
                        -> c_int;
    pub fn aaellipseColor(renderer: *const SDL_Renderer,
                            x: int16_t,
                            y: int16_t,
                            rx: int16_t,
                            ry: int16_t,
                            color: uint32_t)
                            -> c_int;
    pub fn aaellipseRGBA(renderer: *const SDL_Renderer,
                            x: int16_t,
                            y: int16_t,
                            rx: int16_t,
                            ry: int16_t,
                            r: uint8_t,
                            g: uint8_t,
                            b: uint8_t,
                            a: uint8_t)
                            -> c_int;
    pub fn filledEllipseColor(renderer: *const SDL_Renderer,
                                x: int16_t,
                                y: int16_t,
                                rx: int16_t,
                                ry: int16_t,
                                color: uint32_t)
                                -> c_int;
    pub fn filledEllipseRGBA(renderer: *const SDL_Renderer,
                                x: int16_t,
                                y: int16_t,
                                rx: int16_t,
                                ry: int16_t,
                                r: uint8_t,
                                g: uint8_t,
                                b: uint8_t,
                                a: uint8_t)
                                -> c_int;
    pub fn pieColor(renderer: *const SDL_Renderer,
                    x: int16_t,
                    y: int16_t,
                    rad: int16_t,
                    start: int16_t,
                    end: int16_t,
                    color: uint32_t)
                    -> c_int;
    pub fn pieRGBA(renderer: *const SDL_Renderer,
                    x: int16_t,
                    y: int16_t,
                    rad: int16_t,
                    start: int16_t,
                    end: int16_t,
                    r: uint8_t,
                    g: uint8_t,
                    b: uint8_t,
                    a: uint8_t)
                    -> c_int;
    pub fn filledPieColor(renderer: *const SDL_Renderer,
                            x: int16_t,
                            y: int16_t,
                            rad: int16_t,
                            start: int16_t,
                            end: int16_t,
                            color: uint32_t)
                            -> c_int;
    pub fn filledPieRGBA(renderer: *const SDL_Renderer,
                            x: int16_t,
                            y: int16_t,
                            rad: int16_t,
                            start: int16_t,
                            end: int16_t,
                            r: uint8_t,
                            g: uint8_t,
                            b: uint8_t,
                            a: uint8_t)
                            -> c_int;
    pub fn trigonColor(renderer: *const SDL_Renderer,
                        x1: int16_t,
                        y1: int16_t,
                        x2: int16_t,
                        y2: int16_t,
                        x3: int16_t,
                        y3: int16_t,
                        color: uint32_t)
                        -> c_int;
    pub fn trigonRGBA(renderer: *const SDL_Renderer,
                        x1: int16_t,
                        y1: int16_t,
                        x2: int16_t,
                        y2: int16_t,
                        x3: int16_t,
                        y3: int16_t,
                        r: uint8_t,
                        g: uint8_t,
                        b: uint8_t,
                        a: uint8_t)
                        -> c_int;
    pub fn aatrigonColor(renderer: *const SDL_Renderer,
                            x1: int16_t,
                            y1: int16_t,
                            x2: int16_t,
                            y2: int16_t,
                            x3: int16_t,
                            y3: int16_t,
                            color: uint32_t)
                            -> c_int;
    pub fn aatrigonRGBA(renderer: *const SDL_Renderer,
                        x1: int16_t,
                        y1: int16_t,
                        x2: int16_t,
                        y2: int16_t,
                        x3: int16_t,
                        y3: int16_t,
                        r: uint8_t,
                        g: uint8_t,
                        b: uint8_t,
                        a: uint8_t)
                        -> c_int;
    pub fn filledTrigonColor(renderer: *const SDL_Renderer,
                                x1: int16_t,
                                y1: int16_t,
                                x2: int16_t,
                                y2: int16_t,
                                x3: int16_t,
                                y3: int16_t,
                                color: uint32_t)
                                -> c_int;
    pub fn filledTrigonRGBA(renderer: *const SDL_Renderer,
                            x1: int16_t,
                            y1: int16_t,
                            x2: int16_t,
                            y2: int16_t,
                            x3: int16_t,
                            y3: int16_t,
                            r: uint8_t,
                            g: uint8_t,
                            b: uint8_t,
                            a: uint8_t)
                            -> c_int;
    pub fn polygonColor(renderer: *const SDL_Renderer,
                        vx: *const int16_t,
                        vy: *const int16_t,
                        n: c_int,
                        color: uint32_t)
                        -> c_int;
    pub fn polygonRGBA(renderer: *const SDL_Renderer,
                        vx: *const int16_t,
                        vy: *const int16_t,
                        n: c_int,
                        r: uint8_t,
                        g: uint8_t,
                        b: uint8_t,
                        a: uint8_t)
                        -> c_int;
    pub fn aapolygonColor(renderer: *const SDL_Renderer,
                            vx: *const int16_t,
                            vy: *const int16_t,
                            n: c_int,
                            color: uint32_t)
                            -> c_int;
    pub fn aapolygonRGBA(renderer: *const SDL_Renderer,
                            vx: *const int16_t,
                            vy: *const int16_t,
                            n: c_int,
                            r: uint8_t,
                            g: uint8_t,
                            b: uint8_t,
                            a: uint8_t)
                            -> c_int;
    pub fn filledPolygonColor(renderer: *const SDL_Renderer,
                                vx: *const int16_t,
                                vy: *const int16_t,
                                n: c_int,
                                color: uint32_t)
                                -> c_int;
    pub fn filledPolygonRGBA(renderer: *const SDL_Renderer,
                                vx: *const int16_t,
                                vy: *const int16_t,
                                n: c_int,
                                r: uint8_t,
                                g: uint8_t,
                                b: uint8_t,
                                a: uint8_t)
                                -> c_int;
    pub fn texturedPolygon(renderer: *const SDL_Renderer,
                            vx: *const int16_t,
                            vy: *const int16_t,
                            n: c_int,
                            texture: *mut SDL_Surface,
                            texture_dx: c_int,
                            texture_dy: c_int)
                            -> c_int;
    pub fn bezierColor(renderer: *const SDL_Renderer,
                        vx: *const int16_t,
                        vy: *const int16_t,
                        n: c_int,
                        s: c_int,
                        color: uint32_t)
                        -> c_int;
    pub fn bezierRGBA(renderer: *const SDL_Renderer,
                        vx: *const int16_t,
                        vy: *const int16_t,
                        n: c_int,
                        s: c_int,
                        r: uint8_t,
                        g: uint8_t,
                        b: uint8_t,
                        a: uint8_t)
                        -> c_int;
    pub fn gfxPrimitivesSetFont(fontdata: *const c_void, cw: uint32_t, ch: uint32_t);
    pub fn gfxPrimitivesSetFontRotation(rotation: uint32_t);
    pub fn characterColor(renderer: *const SDL_Renderer,
                            x: int16_t,
                            y: int16_t,
                            c: c_char,
                            color: uint32_t)
                            -> c_int;
    pub fn characterRGBA(renderer: *const SDL_Renderer,
                            x: int16_t,
                            y: int16_t,
                            c: c_char,
                            r: uint8_t,
                            g: uint8_t,
                            b: uint8_t,
                            a: uint8_t)
                            -> c_int;
    pub fn stringColor(renderer: *const SDL_Renderer,
                        x: int16_t,
                        y: int16_t,
                        s: *mut c_char,
                        color: uint32_t)
                        -> c_int;
    pub fn stringRGBA(renderer: *const SDL_Renderer,
                        x: int16_t,
                        y: int16_t,
                        s: *mut c_char,
                        r: uint8_t,
                        g: uint8_t,
                        b: uint8_t,
                        a: uint8_t)
                        -> c_int;
}

extern "C" {
    pub fn rotozoomSurface(src: *mut SDL_Surface, angle: c_double,
                            zoom: c_double, smooth: c_int) -> *mut SDL_Surface;
    pub fn rotozoomSurfaceXY(src: *mut SDL_Surface, angle: c_double,
                                zoomx: c_double, zoomy: c_double, smooth: c_int)
                                -> *mut SDL_Surface;
    pub fn rotozoomSurfaceSize(width: c_int, height: c_int, angle: c_double,
                                zoom: c_double, dstwidth: *mut c_int,
                                dstheight: *mut c_int);
    pub fn rotozoomSurfaceSizeXY(width: c_int, height: c_int, angle: c_double,
                                    zoomx: c_double, zoomy: c_double,
                                    dstwidth: *mut c_int, dstheight: *mut c_int);
    pub fn zoomSurface(src: *mut SDL_Surface, zoomx: c_double,
                        zoomy: c_double, smooth: c_int) -> *mut SDL_Surface;
    pub fn zoomSurfaceSize(width: c_int, height: c_int, zoomx: c_double,
                            zoomy: c_double, dstwidth: *mut c_int,
                            dstheight: *mut c_int);
    pub fn shrinkSurface(src: *mut SDL_Surface, factorx: c_int,
                            factory: c_int) -> *mut SDL_Surface;
    pub fn rotateSurface90Degrees(src: *mut SDL_Surface,
                                    numClockwiseTurns: c_int) ->
        *mut SDL_Surface;
}

