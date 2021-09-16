# A Disjoint-Set data structure (aka Union-Find w/ Rank)

The variant implemented uses Path Compression to further improve the performance.

Some [interesting lecture notes](https://www.cs.cmu.edu/~avrim/451f13/lectures/lect0912.pdf) regarding Union-Find.

## Usage

### Setup

In `Cargo.toml`, add this crate as a dependency.

```toml
[dependencies]
reunion = { version = "0.1" }
```
### API

#### Example 1

*Task*: Create a UnionFind data structure of arbitrary size that contains `usize` at its elements.
Then, union a few elements and capture the state of the data structure after that.

*Solution*: 

```rust

use reunion::{UnionFind, UnionFindTrait};
use std::collections::HashSet;

fn main() {
    // Create a UnionFind data structure of arbitrary size that contains subsets of usizes.
    let mut uf1 = UnionFind::<usize>::new();

    // Note: Trivial subsets (i.e. singletons) are ignored in the data structure because they can always be calculated based on the state and the context.

    println!("Freshly created structure: {}", uf1);

    uf1.union(2, 1);
    uf1.union(4, 3);
    uf1.union(6, 5);
    uf1.union(1, 5);

    println!("After a few unions: {}", uf1);

    let mut hs1 = HashSet::new();
    hs1.insert(1);
    hs1.insert(2);
    hs1.insert(6);
    hs1.insert(5);

    let mut hs2 = HashSet::new();
    hs2.insert(3);
    hs2.insert(4);

    let subsets = uf1.subsets();

    assert_eq!(subsets.len(), 2);

    assert!(&subsets.contains(&hs1));
    assert!(&subsets.contains(&hs2));

    // Iterate over the subsets.

    for partition in uf1 {
        println!("{:?}", partition);
    }
}

```

#### Example 2

*Task*: Create a UnionFind data structure of size at least `10`, that contains `u16` at its elements.

**Note**: The size business only helps for reducing the number of memory reallocations required. Therefore, it is not too special and is totally optional.

*Solution*: 

```rust

// Create a UnionFind data structure of a fixed size that contains subsets of u16.
let mut uf2 = UnionFind::<u16>::with_capacity(10);

println!("{}", uf2);

```

## Performance

### Benchmark

### DIY
To benchmark on your machine:

1. Clone this repository.
2. Run `cargo bench`

You should see some output like this:

```
#Find: 2497150, #Union: 1048575, #Total: 3545725, Time: 1.027244379s, Time per operation: 289.712µs
#Find: 2497150, #Union: 1048575, #Total: 3545725, Time: 1.009215098s, Time per operation: 284.628µs
...

Big Merge (20, 10000)   time:   [1.0175 s 1.0190 s 1.0205 s]                                     
                        change: [-0.4773% -0.2721% -0.0647%] (p = 0.01 < 0.05)
                        Change within noise threshold.
Found 13 outliers among 100 measurements (13.00%)
  10 (10.00%) high mild
  3 (3.00%) high severe

...
```

### Summary

On a [AMD Ryzen 9 3900X 12-Core Processor](https://www.amd.com/en/products/cpu/amd-ryzen-9-3900x) (with lots of other processes running),
working with a UnionFind of size `2 ** 20`, a total of `3,545,725` operations take roughly `1` second, which is expected because the time complexity
for these operations is effectively `O(1)` (in truth it is `O(alpha(n))` where `alpha(n)` is the inverse Ackermann function but it grows so slow that we can hand wave it asa constant).
