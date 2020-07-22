use std::{
    alloc,
    ops::{Deref, DerefMut},
    ptr,
};

use super::ibase;

pub struct XSqlDa {
    ptr: ptr::NonNull<ibase::XSQLDA>,
    len: i16,
}

impl XSqlDa {
    /// Allocates a new XSQLDA of length `len`
    pub fn new(len: i16) -> Self {
        #[allow(clippy::cast_ptr_alignment)]
        let ptr = unsafe { alloc::alloc_zeroed(xsqlda_layout(len)) } as *mut ibase::XSQLDA;

        let mut ptr = if let Some(ptr) = ptr::NonNull::new(ptr) {
            ptr
        } else {
            alloc::handle_alloc_error(xsqlda_layout(len))
        };

        unsafe {
            ptr.as_mut().version = ibase::SQLDA_VERSION1 as i16;
            ptr.as_mut().sqln = len;
        }

        Self { ptr, len }
    }

    /// Returns a mutable reference to a XSQLVAR
    pub fn get_xsqlvar_mut(&mut self, col: usize) -> Option<&mut ibase::XSQLVAR> {
        if col < self.sqld as usize {
            let xsqlvar = unsafe { self.ptr.as_mut().sqlvar.get_unchecked_mut(col as usize) };

            Some(xsqlvar)
        } else {
            None
        }
    }
}

impl Deref for XSqlDa {
    type Target = ibase::XSQLDA;

    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref() }
    }
}

impl DerefMut for XSqlDa {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.ptr.as_mut() }
    }
}

impl Drop for XSqlDa {
    fn drop(&mut self) {
        unsafe { alloc::dealloc(self.ptr.as_ptr() as *mut u8, xsqlda_layout(self.len)) }
    }
}

/// Calculates the memory layout (size and alignment) for a xsqlda
fn xsqlda_layout(len: i16) -> alloc::Layout {
    let (xsqlda_layout, _) = alloc::Layout::new::<ibase::XSQLDA>()
        .extend(alloc::Layout::array::<ibase::XSQLVAR>((len - 1).max(0) as usize).unwrap())
        .unwrap();

    xsqlda_layout
}