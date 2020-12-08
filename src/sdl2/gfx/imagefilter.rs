//! MMX image filters

use c_vec::CVec;
use get_error;
use libc::{self, c_int, c_uint, c_void, size_t};
use std::mem;
use sys::gfx::imagefilter;

/// MMX detection routine (with override flag).
pub fn mmx_detect() -> bool {
    unsafe { imagefilter::SDL_imageFilterMMXdetect() == 1 }
}

/// Disable MMX check for filter functions and and force to use non-MMX C based code.
pub fn mmx_off() {
    unsafe { imagefilter::SDL_imageFilterMMXoff() }
}

/// Enable MMX check for filter functions and use MMX code if available.
pub fn mmx_on() {
    unsafe { imagefilter::SDL_imageFilterMMXon() }
}

#[inline]
fn cvec_with_size(sz: usize) -> CVec<u8> {
    unsafe {
        let p = libc::malloc(sz as size_t) as *mut u8;
        CVec::new_with_dtor(p, sz, move |p| libc::free(p as *mut c_void))
    }
}

/// Filter using Add: D = saturation255(S1 + S2).
pub fn add(src1: CVec<u8>, src2: CVec<u8>) -> Result<CVec<u8>, String> {
    assert_eq!(src1.len(), src2.len());
    let size = src1.len();
    let dest = cvec_with_size(size);
    let ret = unsafe {
        imagefilter::SDL_imageFilterAdd(
            mem::transmute(src1.get(0)),
            mem::transmute(src2.get(0)),
            mem::transmute(dest.get(0)),
            size as c_uint,
        )
    };
    if ret == 0 {
        Ok(dest)
    } else {
        Err(get_error())
    }
}

/// Filter using Mean: D = S1/2 + S2/2.
pub fn mean(src1: CVec<u8>, src2: CVec<u8>) -> Result<CVec<u8>, String> {
    assert_eq!(src1.len(), src2.len());
    let size = src1.len();
    let dest = cvec_with_size(size);
    let ret = unsafe {
        imagefilter::SDL_imageFilterMean(
            mem::transmute(src1.get(0)),
            mem::transmute(src2.get(0)),
            mem::transmute(dest.get(0)),
            size as c_uint,
        )
    };
    if ret == 0 {
        Ok(dest)
    } else {
        Err(get_error())
    }
}

/// Filter using Sub: D = saturation0(S1 - S2).
pub fn sub(src1: CVec<u8>, src2: CVec<u8>) -> Result<CVec<u8>, String> {
    assert_eq!(src1.len(), src2.len());
    let size = src1.len();
    let dest = cvec_with_size(size);
    let ret = unsafe {
        imagefilter::SDL_imageFilterSub(
            mem::transmute(src1.get(0)),
            mem::transmute(src2.get(0)),
            mem::transmute(dest.get(0)),
            size as c_uint,
        )
    };
    if ret == 0 {
        Ok(dest)
    } else {
        Err(get_error())
    }
}

/// Filter using `AbsDiff`: D = | S1 - S2 |.
pub fn abs_diff(src1: CVec<u8>, src2: CVec<u8>) -> Result<CVec<u8>, String> {
    assert_eq!(src1.len(), src2.len());
    let size = src1.len();
    let dest = cvec_with_size(size);
    let ret = unsafe {
        imagefilter::SDL_imageFilterAbsDiff(
            mem::transmute(src1.get(0)),
            mem::transmute(src2.get(0)),
            mem::transmute(dest.get(0)),
            size as c_uint,
        )
    };
    if ret == 0 {
        Ok(dest)
    } else {
        Err(get_error())
    }
}

/// Filter using Mult: D = saturation255(S1 * S2).
pub fn mult(src1: CVec<u8>, src2: CVec<u8>) -> Result<CVec<u8>, String> {
    assert_eq!(src1.len(), src2.len());
    let size = src1.len();
    let dest = cvec_with_size(size);
    let ret = unsafe {
        imagefilter::SDL_imageFilterMult(
            mem::transmute(src1.get(0)),
            mem::transmute(src2.get(0)),
            mem::transmute(dest.get(0)),
            size as c_uint,
        )
    };
    if ret == 0 {
        Ok(dest)
    } else {
        Err(get_error())
    }
}

/// Filter using `MultNor`: D = S1 * S2.
pub fn mult_nor(src1: CVec<u8>, src2: CVec<u8>) -> Result<CVec<u8>, String> {
    assert_eq!(src1.len(), src2.len());
    let size = src1.len();
    let dest = cvec_with_size(size);
    let ret = unsafe {
        imagefilter::SDL_imageFilterMultNor(
            mem::transmute(src1.get(0)),
            mem::transmute(src2.get(0)),
            mem::transmute(dest.get(0)),
            size as c_uint,
        )
    };
    if ret == 0 {
        Ok(dest)
    } else {
        Err(get_error())
    }
}

/// Filter using `MultDivby2`: D = saturation255(S1/2 * S2).
pub fn mult_div_by2(src1: CVec<u8>, src2: CVec<u8>) -> Result<CVec<u8>, String> {
    assert_eq!(src1.len(), src2.len());
    let size = src1.len();
    let dest = cvec_with_size(size);
    let ret = unsafe {
        imagefilter::SDL_imageFilterMultDivby2(
            mem::transmute(src1.get(0)),
            mem::transmute(src2.get(0)),
            mem::transmute(dest.get(0)),
            size as c_uint,
        )
    };
    if ret == 0 {
        Ok(dest)
    } else {
        Err(get_error())
    }
}

/// Filter using `MultDivby4`: D = saturation255(S1/2 * S2/2).
pub fn mult_div_by4(src1: CVec<u8>, src2: CVec<u8>) -> Result<CVec<u8>, String> {
    assert_eq!(src1.len(), src2.len());
    let size = src1.len();
    let dest = cvec_with_size(size);
    let ret = unsafe {
        imagefilter::SDL_imageFilterMultDivby4(
            mem::transmute(src1.get(0)),
            mem::transmute(src2.get(0)),
            mem::transmute(dest.get(0)),
            size as c_uint,
        )
    };
    if ret == 0 {
        Ok(dest)
    } else {
        Err(get_error())
    }
}

/// Filter using `BitAnd`: D = S1 & S2.
pub fn bit_and(src1: CVec<u8>, src2: CVec<u8>) -> Result<CVec<u8>, String> {
    assert_eq!(src1.len(), src2.len());
    let size = src1.len();
    let dest = cvec_with_size(size);
    let ret = unsafe {
        imagefilter::SDL_imageFilterBitAnd(
            mem::transmute(src1.get(0)),
            mem::transmute(src2.get(0)),
            mem::transmute(dest.get(0)),
            size as c_uint,
        )
    };
    if ret == 0 {
        Ok(dest)
    } else {
        Err(get_error())
    }
}

/// Filter using `BitOr`: D = S1 | S2.
pub fn bit_or(src1: CVec<u8>, src2: CVec<u8>) -> Result<CVec<u8>, String> {
    assert_eq!(src1.len(), src2.len());
    let size = src1.len();
    let dest = cvec_with_size(size);
    let ret = unsafe {
        imagefilter::SDL_imageFilterBitOr(
            mem::transmute(src1.get(0)),
            mem::transmute(src2.get(0)),
            mem::transmute(dest.get(0)),
            size as c_uint,
        )
    };
    if ret == 0 {
        Ok(dest)
    } else {
        Err(get_error())
    }
}

/// Filter using Div: D = S1 / S2.
pub fn div(src1: CVec<u8>, src2: CVec<u8>) -> Result<CVec<u8>, String> {
    assert_eq!(src1.len(), src2.len());
    let size = src1.len();
    let dest = cvec_with_size(size);
    let ret = unsafe {
        imagefilter::SDL_imageFilterDiv(
            mem::transmute(src1.get(0)),
            mem::transmute(src2.get(0)),
            mem::transmute(dest.get(0)),
            size as c_uint,
        )
    };
    if ret == 0 {
        Ok(dest)
    } else {
        Err(get_error())
    }
}

/// Filter using `BitNegation`: D = !S.
pub fn bit_negation(src1: CVec<u8>) -> Result<CVec<u8>, String> {
    let size = src1.len();
    let dest = cvec_with_size(size);
    let ret = unsafe {
        imagefilter::SDL_imageFilterBitNegation(
            mem::transmute(src1.get(0)),
            mem::transmute(dest.get(0)),
            size as c_uint,
        )
    };
    if ret == 0 {
        Ok(dest)
    } else {
        Err(get_error())
    }
}

/// Filter using `AddByte`: D = saturation255(S + C).
pub fn add_byte(src1: CVec<u8>, c: u8) -> Result<CVec<u8>, String> {
    let size = src1.len();
    let dest = cvec_with_size(size);
    let ret = unsafe {
        imagefilter::SDL_imageFilterAddByte(
            mem::transmute(src1.get(0)),
            mem::transmute(dest.get(0)),
            size as c_uint,
            c,
        )
    };
    if ret == 0 {
        Ok(dest)
    } else {
        Err(get_error())
    }
}

/// Filter using `AddUint`: D = saturation255((S[i] + Cs[i % 4]), Cs=Swap32((uint)C).
pub fn add_uint(src1: CVec<u8>, c: u32) -> Result<CVec<u8>, String> {
    let size = src1.len();
    let dest = cvec_with_size(size);
    let ret = unsafe {
        imagefilter::SDL_imageFilterAddUint(
            mem::transmute(src1.get(0)),
            mem::transmute(dest.get(0)),
            size as c_uint,
            c,
        )
    };
    if ret == 0 {
        Ok(dest)
    } else {
        Err(get_error())
    }
}

/// Filter using `AddByteToHalf`: D = saturation255(S/2 + C).
pub fn add_byte_to_half(src1: CVec<u8>, c: u8) -> Result<CVec<u8>, String> {
    let size = src1.len();
    let dest = cvec_with_size(size);
    let ret = unsafe {
        imagefilter::SDL_imageFilterAddByteToHalf(
            mem::transmute(src1.get(0)),
            mem::transmute(dest.get(0)),
            size as c_uint,
            c,
        )
    };
    if ret == 0 {
        Ok(dest)
    } else {
        Err(get_error())
    }
}

/// Filter using `SubByte`: D = saturation0(S - C).
pub fn sub_byte(src1: CVec<u8>, c: u8) -> Result<CVec<u8>, String> {
    let size = src1.len();
    let dest = cvec_with_size(size);
    let ret = unsafe {
        imagefilter::SDL_imageFilterSubByte(
            mem::transmute(src1.get(0)),
            mem::transmute(dest.get(0)),
            size as c_uint,
            c,
        )
    };
    if ret == 0 {
        Ok(dest)
    } else {
        Err(get_error())
    }
}

/// Filter using `SubUint`: D = saturation0(S[i] - Cs[i % 4]), Cs=Swap32((uint)C).
pub fn sub_uint(src1: CVec<u8>, c: u32) -> Result<CVec<u8>, String> {
    let size = src1.len();
    let dest = cvec_with_size(size);
    let ret = unsafe {
        imagefilter::SDL_imageFilterSubUint(
            mem::transmute(src1.get(0)),
            mem::transmute(dest.get(0)),
            size as c_uint,
            c,
        )
    };
    if ret == 0 {
        Ok(dest)
    } else {
        Err(get_error())
    }
}

/// Filter using `ShiftRight`: D = saturation0(S >> N).
pub fn shift_right(src1: CVec<u8>, n: u8) -> Result<CVec<u8>, String> {
    let size = src1.len();
    let dest = cvec_with_size(size);
    let ret = unsafe {
        imagefilter::SDL_imageFilterShiftRight(
            mem::transmute(src1.get(0)),
            mem::transmute(dest.get(0)),
            size as c_uint,
            n,
        )
    };
    if ret == 0 {
        Ok(dest)
    } else {
        Err(get_error())
    }
}

/// Filter using `ShiftRightUint`: D = saturation0((uint)S[i] >> N).
pub fn shift_right_uint(src1: CVec<u8>, n: u8) -> Result<CVec<u8>, String> {
    let size = src1.len();
    let dest = cvec_with_size(size);
    let ret = unsafe {
        imagefilter::SDL_imageFilterShiftRightUint(
            mem::transmute(src1.get(0)),
            mem::transmute(dest.get(0)),
            size as c_uint,
            n,
        )
    };
    if ret == 0 {
        Ok(dest)
    } else {
        Err(get_error())
    }
}

/// Filter using `MultByByte`: D = saturation255(S * C).
pub fn mult_by_byte(src1: CVec<u8>, c: u8) -> Result<CVec<u8>, String> {
    let size = src1.len();
    let dest = cvec_with_size(size);
    let ret = unsafe {
        imagefilter::SDL_imageFilterMultByByte(
            mem::transmute(src1.get(0)),
            mem::transmute(dest.get(0)),
            size as c_uint,
            c,
        )
    };
    if ret == 0 {
        Ok(dest)
    } else {
        Err(get_error())
    }
}

/// Filter using `ShiftRightAndMultByByte`: D = saturation255((S >> N) * C).
pub fn shift_right_and_mult_by_byte(src1: CVec<u8>, n: u8, c: u8) -> Result<CVec<u8>, String> {
    let size = src1.len();
    let dest = cvec_with_size(size);
    let ret = unsafe {
        imagefilter::SDL_imageFilterShiftRightAndMultByByte(
            mem::transmute(src1.get(0)),
            mem::transmute(dest.get(0)),
            size as c_uint,
            n,
            c,
        )
    };
    if ret == 0 {
        Ok(dest)
    } else {
        Err(get_error())
    }
}

/// Filter using `ShiftLeftByte`: D = (S << N).
pub fn shift_left_byte(src1: CVec<u8>, n: u8) -> Result<CVec<u8>, String> {
    let size = src1.len();
    let dest = cvec_with_size(size);
    let ret = unsafe {
        imagefilter::SDL_imageFilterShiftLeftByte(
            mem::transmute(src1.get(0)),
            mem::transmute(dest.get(0)),
            size as c_uint,
            n,
        )
    };
    if ret == 0 {
        Ok(dest)
    } else {
        Err(get_error())
    }
}

/// Filter using `ShiftLeftUint`: D = ((uint)S << N).
pub fn shift_left_uint(src1: CVec<u8>, n: u8) -> Result<CVec<u8>, String> {
    let size = src1.len();
    let dest = cvec_with_size(size);
    let ret = unsafe {
        imagefilter::SDL_imageFilterShiftLeftUint(
            mem::transmute(src1.get(0)),
            mem::transmute(dest.get(0)),
            size as c_uint,
            n,
        )
    };
    if ret == 0 {
        Ok(dest)
    } else {
        Err(get_error())
    }
}

/// Filter `ShiftLeft`: D = saturation255(S << N).
pub fn shift_left(src1: CVec<u8>, n: u8) -> Result<CVec<u8>, String> {
    let size = src1.len();
    let dest = cvec_with_size(size);
    let ret = unsafe {
        imagefilter::SDL_imageFilterShiftLeft(
            mem::transmute(src1.get(0)),
            mem::transmute(dest.get(0)),
            size as c_uint,
            n,
        )
    };
    if ret == 0 {
        Ok(dest)
    } else {
        Err(get_error())
    }
}

/// Filter using `BinarizeUsingThreshold`: D = (S >= T) ? 255:0.
pub fn binarize_using_threshold(src1: CVec<u8>, t: u8) -> Result<CVec<u8>, String> {
    let size = src1.len();
    let dest = cvec_with_size(size);
    let ret = unsafe {
        imagefilter::SDL_imageFilterBinarizeUsingThreshold(
            mem::transmute(src1.get(0)),
            mem::transmute(dest.get(0)),
            size as c_uint,
            t,
        )
    };
    if ret == 0 {
        Ok(dest)
    } else {
        Err(get_error())
    }
}

/// Filter using `ClipToRange`: D = (S >= Tmin) & (S <= Tmax) S:Tmin | Tmax.
pub fn clip_to_range(src1: CVec<u8>, tmin: u8, tmax: u8) -> Result<CVec<u8>, String> {
    let size = src1.len();
    let dest = cvec_with_size(size);
    let ret = unsafe {
        imagefilter::SDL_imageFilterClipToRange(
            mem::transmute(src1.get(0)),
            mem::transmute(dest.get(0)),
            size as c_uint,
            tmin,
            tmax,
        )
    };
    if ret == 0 {
        Ok(dest)
    } else {
        Err(get_error())
    }
}

/// Filter using `NormalizeLinear`: D = saturation255((Nmax - Nmin)/(Cmax - Cmin)*(S - Cmin) + Nmin).
pub fn normalize_linear(
    src1: CVec<u8>,
    cmin: i32,
    cmax: i32,
    nmin: i32,
    nmax: i32,
) -> Result<CVec<u8>, String> {
    let size = src1.len();
    let dest = cvec_with_size(size);
    let ret = unsafe {
        imagefilter::SDL_imageFilterNormalizeLinear(
            mem::transmute(src1.get(0)),
            mem::transmute(dest.get(0)),
            size as c_uint,
            cmin as c_int,
            cmax as c_int,
            nmin as c_int,
            nmax as c_int,
        )
    };
    if ret == 0 {
        Ok(dest)
    } else {
        Err(get_error())
    }
}
