use crate::window_handler::GlContext;
use glow::HasContext;

pub trait AsRawBytes {
    fn as_raw_bytes(&self) -> &[u8];
}

impl<T: Copy> AsRawBytes for &[T] {
    fn as_raw_bytes(&self) -> &[u8] {
        generic_slice_as_u8_slice(*self)
    }
}

impl<T: Copy> AsRawBytes for Vec<T> {
    fn as_raw_bytes(&self) -> &[u8] {
        generic_slice_as_u8_slice(self.as_slice())
    }
}

impl<T: Copy, const LEN: usize> AsRawBytes for [T; LEN] {
    fn as_raw_bytes(&self) -> &[u8] {
        generic_slice_as_u8_slice(self.as_slice())
    }
}

fn generic_slice_as_u8_slice<T: Copy>(slice: &[T]) -> &[u8] {
    unsafe {
        let len = slice.len() * std::mem::size_of::<T>();
        let ptr = std::mem::transmute::<_, *mut u8>(slice.as_ptr());
        std::slice::from_raw_parts_mut(ptr, len)
    }
}

#[allow(unused)]
pub fn gl_get_error(gl: &GlContext) {
    unsafe {
        loop {
            match gl.get_error() {
                glow::NO_ERROR => break,
                error => println!("GL errno: 0x{:x?}", error),
            }
        }
    }
}
