pub mod Buffer;
use Buffer::Storage;
use std::alloc::{self, Layout};
use std::ptr:: NonNull;



 struct Config {
     dim: i32, // transformer dimension
     hidden_dim: i32, // for ffn layers
     n_layers: i32, // number of layers
     n_heads: i32, // number of query heads
     n_kv_heads: i32, // number of key/value heads (can be < query heads because of multiquery)
     vocab_size: i32, // vocabulary size, usually 256 (byte-level)
     seq_len: i32 // max sequence length
} 




fn accum(a : *mut f32, b: *const f32, size: usize) {
    for i in 0..size {
     unsafe{   *(a.add(i)) += *(b.add(i));}
    }
}




fn main(){

let mut v1 = vec![1.,2.,3.,4.,5.];


let mut st = Storage::from_raw_parts(v1.as_mut_ptr(), v1.capacity());

}
