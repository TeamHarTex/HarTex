/// # Struct `SliceRead`
///
/// A DSL input source that reads from a slice of bytes.
pub struct SliceRead<'bytes> {
    slice: &'bytes [u8],
    index: usize
}
