use std::alloc::{alloc, Layout};
use std::cell::{Cell, RefCell};
use std::ptr;
use std::slice;
use std::str;

const PAGE_SIZE: usize = 4096;

#[derive(Debug)]
pub struct Arena {
    start: Cell<*mut u8>,
    end: Cell<*mut u8>,
    chunks: RefCell<Vec<*const u8>>,
}

impl Arena {
    pub fn new() -> Self {
        Self {
            start: Cell::new(ptr::null_mut()),
            end: Cell::new(ptr::null_mut()),
            chunks: RefCell::new(Vec::new()),
        }
    }

    fn new_chunk(&self, bytes: usize) {
        let mut chunk_size = PAGE_SIZE;

        while bytes > chunk_size {
            chunk_size = chunk_size
                .checked_shl(1)
                .unwrap_or_else(|| panic!("Cannot allocate more than {} bytes.", usize::MAX));
        }

        let layout = Layout::array::<u8>(chunk_size).unwrap();
        unsafe {
            let ptr = alloc(layout);
            self.chunks.borrow_mut().push(ptr);
            self.start.set(ptr);
            self.end.set(ptr.wrapping_add(chunk_size));
        }
    }

    fn allocate(&self, layout: Layout) -> *mut u8 {
        let start = self.start.get() as usize;
        let end = self.end.get() as usize;
        let align = layout.align();
        let bytes = layout.size();

        let aligned = start.checked_add(align - 1).unwrap() & !(align - 1);
        let new_start = aligned
            .checked_add(bytes)
            .unwrap_or_else(|| panic!("Cannot allocate more than {} bytes.", usize::MAX));

        if new_start <= end {
            self.start.set(new_start as *mut u8);
            aligned as *mut u8
        } else {
            self.new_chunk(bytes);
            let ptr = self.start.get();
            self.start.set(ptr.wrapping_add(bytes));
            ptr
        }
    }

    pub fn allocate_string<'a, 'b>(&'a self, s: &'b str) -> &'a str {
        assert!(!s.is_empty());
        let layout = Layout::for_value(s.as_bytes());
        let ptr = self.allocate(layout);

        unsafe {
            ptr.copy_from_nonoverlapping(s.as_ptr(), s.len());
            str::from_utf8_unchecked(slice::from_raw_parts(ptr, s.len()))
        }
    }
}
