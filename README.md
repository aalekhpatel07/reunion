# A Disjoint-Set data structure (aka Union-Find w/ Rank)

## What is Union-Find?

Suppose you have a collection `S` of elements `e1`, `e2`, `...`, `en`, and wish to group them into different collections using operations:

- "put `ei` and `ej` into the same group" (union),
- "give me a representative of the group `ei` belongs to" (find).

Then a Union-Find data structure helps to store the underlying groups very efficiently and implements this API.

## (Some) Applications

- **Detect Cycles in Graph**: Given a graph `G`, we can put the endpoints of edges into the same group (same connected component) unless there is a pair of endpoints `(ei, ej)` that share a group representative. If that happens, there was already a path existing between them, and adding this edge will add multiple paths, which cannot be the case for acyclic graphs.

- **Number of connected components in Graph**: Given a graph `G`, put the endpoints of edges into the same group (same connected component). Once all nodes are exhausted, the number of groups formed is the number of connected components in `G`.

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
