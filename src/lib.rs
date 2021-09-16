mod union_find;

pub use union_find::{UnionFind, UnionFindTrait};

// Python/WASM bindings go here.
//
// Subset of the API can be selectively exposed.

#[cfg(test)]
mod tests {

    use crate::*;
    use std::collections::HashSet;

    fn create_union_find_usize(size: usize) -> UnionFind<usize> {
        UnionFind::<usize>::with_capacity(size)
    }

    #[test]
    fn create_union_find_size() {
        let uf = create_union_find_usize(10);
        assert_eq!(uf.size(), 10);
    }

    #[test]
    fn union() {
        let mut uf = create_union_find_usize(9);
        println!("Initial state: {}", &uf);
        println!("All elements form their own group (singletons).");

        assert_eq!(format!("{:?}", &uf.subsets()), "[]");
        println!("{:?}", &uf.subsets());
        
        uf.union(2, 1);
        println!("After combining the groups that contains 2 and 1: {}", &uf);
        uf.union(4, 3);
        println!("After combining the groups that contains 4 and 3: {}", &uf);
        uf.union(6, 5);
        println!("After combining the groups that contains 6 and 5: {}", &uf);

        let mut hs1 = HashSet::new();
        hs1.insert(1);
        hs1.insert(2);
        let mut hs2 = HashSet::new();
        hs2.insert(3);
        hs2.insert(4);
        let mut hs3 = HashSet::new();
        hs3.insert(5);
        hs3.insert(6);

        let mut subsets = uf.subsets();
        assert_eq!(subsets.len(), 3);

        assert!(&subsets.contains(&hs1));
        assert!(&subsets.contains(&hs2));
        assert!(&subsets.contains(&hs3));

        println!("Before union of (1, 5) {:?}", &uf.subsets());
        uf.union(1, 5);
        println!("After union of (1, 5) {:?}", &uf.subsets());

        println!("After combining the groups that contains 1 and 5: {}", &uf);

        subsets = uf.subsets();
        assert_eq!(subsets.len(), 2);

        hs3.extend(&hs1);

        assert!(&subsets.contains(&hs3));
        assert!(&subsets.contains(&hs2));


        let mut uf_clone = uf.clone();
        println!("Before find (2) {:?}", &uf_clone.subsets());
        uf_clone.find(2);
        println!("After find (2) {:?}", &uf_clone.subsets());
        println!("The results must be same because a find operation does not modify the subsets formed.");
        assert_eq!(&uf, &uf_clone);

        assert_eq!(&uf, &uf_clone);
        println!("{}", &uf);
    }
}
