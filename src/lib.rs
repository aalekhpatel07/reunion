pub mod union_find;

// Python/WASM bindings go here.
// 
// Subset of the API can be selectively exposed.

#[cfg(test)]
mod tests {

    use crate::union_find::*;
    use std::collections::HashSet;

    fn create_union_find_usize(size: usize) -> UnionFind<usize> {
        UnionFind::<usize>::new(size)
    }
    fn create_union_find_u8(size: usize) -> UnionFind<u8> {
        UnionFind::<u8>::new(size)
    }
    fn create_union_find_u16(size: usize) -> UnionFind<u16> {
        UnionFind::<u16>::new(size)
    }

    #[test]
    fn create_union_find_size() {
        let uf = create_union_find_usize(10);
        assert_eq!(uf.size(), 10);
    }

    #[test]
    fn union() {
        let mut uf = create_union_find_usize(9);
        uf.union(2, 1);
        uf.union(4, 3);
        uf.union(6, 5);


        let mut hs1 = HashSet::new();
        hs1.insert(1);
        hs1.insert(2);
        let mut hs2 = HashSet::new();
        hs2.insert(3);
        hs2.insert(4);
        let mut hs3 = HashSet::new();
        hs3.insert(5);
        hs3.insert(6);

        let mut subsets = uf.into_subsets();
        assert_eq!(subsets.len(), 3);

        assert!(&subsets.contains(&hs1));
        assert!(&subsets.contains(&hs2));
        assert!(&subsets.contains(&hs3));

        uf.union(1, 5);

        subsets = uf.into_subsets();
        assert_eq!(subsets.len(), 2);
        
        hs3.extend(&hs1);

        assert!(&subsets.contains(&hs3));
        assert!(&subsets.contains(&hs2));
        
        let mut uf_clone = uf.clone();
        uf_clone.find(2);

        assert_eq!(&uf, &uf_clone);
        println!("{}", &uf);
    }

}
