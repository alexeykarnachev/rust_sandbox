use algs::sorts::heap_sort::heap_sort;
use algs::sorts::merge_sort::merge_sort;
use rand::Rng;
use std::time::Instant;

fn main() {
    let n = 1e6 as usize;
    let mut rng = rand::thread_rng();
    let vec: Vec<i32> = (0..n).map(|_| rng.gen_range(0, 20)).collect();

    let x = vec.clone();
    let start = Instant::now();
    merge_sort(x);
    let duration = start.elapsed();
    println!("Custom merge_sort: {:.4}s", duration.as_secs_f32());

    let mut x = vec.clone();
    let start = Instant::now();
    heap_sort(x.as_mut_slice());
    let duration = start.elapsed();
    println!("Custom heap_sort: {:.4}s", duration.as_secs_f32());

    let mut x = vec.clone();
    let start = Instant::now();
    x.sort();
    let duration = start.elapsed();
    println!("Rust default sort: {:.4}s", duration.as_secs_f32());
}
