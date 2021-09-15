# A Disjoint-Set data structure (aka Union-Find w/ Rank)

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
