mod Buffer;
use Buffer::Storage;
use std::alloc::{self, Layout};
use std::ptr:: NonNull;




fn  shift_array<T>(ptr : *mut T, size: usize, shift : usize){
    unsafe {std::ptr::copy(ptr, ptr.add(shift), size)};

}

pub fn allocate<T>(capacity: usize) -> *mut T {
      
    let layout = Layout::array::<T>(capacity).unwrap();
    let result = unsafe{alloc::alloc(layout) };    
    
    result as *mut T
        
    }



