use ::std::os::raw::*;
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
