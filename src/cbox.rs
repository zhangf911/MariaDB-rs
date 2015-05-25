// There's probably a library to do this already.
// Don't know of one of the top of my head, and
// it's easy enough to implement.

//! A wrapper for C pointers.

use ::libc::{free, c_void, size_t, c_char, malloc};
use ::std::ops::{Index, IndexMut};

/// Wrapper for a C-pointer.
/// The reason for this is Box<T> does not implement a .get() like function (the only way to get
/// the pointer is to consume the Box) and also it does not specify how allocations are done.  This
/// needs to call libc's malloc() and free() specifically.
pub struct CBox<T>(*mut T);

impl<T> CBox<T> {
    /// Make a new CBox by making a new pointer with a given size with malloc.
    pub fn new(size: usize) -> Self {
        CBox(unsafe { malloc(size as size_t) } as *mut T)
    }

    /// Make a new CBox out of a given *const T
    pub fn from_raw(var: *const T) -> Self {
        CBox(var as *mut T)
    }
    
    /// Make a new CBox out of a given *mut T
    pub fn from_raw_mut(var: *mut T) -> Self {
        CBox(var)
    }
    
    /// Get the pointer as *const T
    pub fn get_raw(&self) -> *const T {
        self.0
    }
    
    /// Get the pointer as *mut T
    pub fn get_raw_mut(&self) -> *mut T {
        self.0
    }
}

impl<T> Index<usize> for CBox<T> {
    type Output = T;

    fn index<'a>(&'a self, index: usize) -> &'a T {
        unsafe { &*self.0.offset(index as isize) }
    }
}

impl<T> IndexMut<usize> for CBox<T> {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut T {
        unsafe { &mut *self.0.offset(index as isize) }
    }
}

impl<T> Drop for CBox<T> {
    fn drop(&mut self) {
        unsafe { free(self.0 as *mut c_void) }
    }
}

pub fn from_cstr(str_in: &CBox<c_char>) -> String {
    let mut str_out = String::new();
    let mut pos = 0;
    while str_in[pos] != 0 {
        str_out.push(str_in[pos] as u8 as char);
        pos += 1;
    }
    str_out
}

pub fn to_cstr(str_in: &str) -> CBox<c_char> {
    let bytes = str_in.bytes();
    let mut var = CBox::from_raw(unsafe { malloc((bytes.len() + 1) as size_t) as *mut i8});
    let mut pos = 0;
    for i in bytes {
        var[pos] = i as i8;
        pos += 1;
    }

    //NUL terminate the end
    var[str_in.len()] = 0;
    
    var
}

#[test]
//Tests to make sure te mutable reference in CBox is working correctly.
fn test_on_indexmut() {
    let mut var = to_cstr("Blah");
    assert!(var[0] == 'B' as i8);
    var[0] = 'c' as i8;
    assert!(var[0] != 'B' as i8);
}

