#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum GLProfile {
    /// OpenGL core profile - deprecated functions are disabled
    Core,
    /// OpenGL compatibility profile - deprecated functions are allowed
    Compatibility,
    /// OpenGL ES profile - only a subset of the base OpenGL functionality is available
    GLES,
    /// Unknown profile - SDL will tend to return 0 if you ask when no particular profile
    /// has been defined or requested.
    Unknown(i32),
}
