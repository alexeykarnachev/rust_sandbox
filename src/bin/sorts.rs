use rand::Rng;
use std::time::Instant;

use algs::merge_sort::merge_sort;

fn main() {
    let n = 1e8 as usize;
    let mut rng = rand::thread_rng();
    let vec: Vec<i32> = (0..n).map(|_| rng.gen_range(0, 20)).collect();

    let x = vec.clone(); 
    let start = Instant::now();
    merge_sort(x);
    let duration = start.elapsed();
    println!("Custom merge_sort: {:.4}s", duration.as_secs_f32());

    let mut x = vec.clone();
    let start = Instant::now();
    x.sort();
    let duration = start.elapsed();
    println!("Rust default sort: {:.4}s", duration.as_secs_f32());
}
