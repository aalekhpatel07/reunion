use criterion::{black_box, criterion_group, criterion_main, Criterion};
use reunion::{UnionFind, UnionFindTrait};
use rand::Rng;
use std::time::Instant;
use std::time::Duration;


fn test_big_merge(max_rank: usize, trials: u32) {
    let instant = Instant::now();

    let mut find_count: usize = 0;
    let mut union_count: usize = 0;

    let mut rng = rand::thread_rng();

    let num_elements = (1 << max_rank) as usize;
    let mut uf = UnionFind::<u32>::with_capacity(num_elements);

    for level in 0..max_rank {
        let merge_step: usize = (1 << level) as usize;
        let increment_step: usize = merge_step << 1_usize;
        for idx in (0..num_elements).step_by(increment_step) {

            let idx_rep: u32 = uf.find(idx as u32);
            let idx_plus_merge_step_rep: u32 = uf.find((idx + merge_step) as u32);
            assert!(idx_rep != idx_plus_merge_step_rep);
            
            uf.union(idx_rep, idx_plus_merge_step_rep);

            let idx_rep: u32 = uf.find(idx as u32);
            let idx_plus_merge_step_rep: u32 = uf.find((idx + merge_step) as u32);
            assert!(idx_rep == idx_plus_merge_step_rep);

            find_count += 2;
            union_count += 1;
        }

        let mask: usize = -(increment_step as isize) as usize;
        for _trial_idx in 0..(trials as usize) {
            let node_a: usize = rng.gen_range(0..(num_elements - 1));
            let node_b: usize = rng.gen_range(0..(num_elements - 1));
            
            let to_expect: bool = (node_a & mask) == (node_b & mask);

            let node_a_rep: u32 = uf.find(node_a as u32);
            let node_b_rep: u32 = uf.find(node_b as u32);
            assert_eq!(to_expect, node_a_rep == node_b_rep);
        }
        find_count += (trials as usize) << 1;
    }
    
    let duration = instant.elapsed();
    println!("#Find: {}, #Union: {}, #Total: {}, Time: {:?}, Time per operation: {:?}", find_count, union_count, find_count + union_count, duration, duration / ((find_count + union_count) as u32) );
}


fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Big Merge (10, 10000)", |b| b.iter(|| test_big_merge(black_box(10), black_box(10000))));
    c.bench_function("Big Merge (20, 10000)", |b| b.iter(|| test_big_merge(black_box(20), black_box(10000))));
    c.bench_function("Big Merge (20, 20000)", |b| b.iter(|| test_big_merge(black_box(20), black_box(20000))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
