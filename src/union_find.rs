use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq)]
pub struct UnionFind<N: Copy + Hash + Eq + Clone> {
    _size: usize,
    pub parents: HashMap<N, N>,
    rank: HashMap<N, usize>,
}

pub trait UnionFindTrait<N: Copy + Eq + Hash + Clone> {
    fn find(&mut self, node: N) -> N;
    fn union(&mut self, x: N, y: N);
    fn subsets(&mut self) -> Vec<HashSet<N>>;
}

impl<T> Default for UnionFind<T>
where
    T: Copy + Eq + Hash + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> UnionFind<T>
where
    T: Copy + Eq + Hash + Clone,
{
    pub fn new() -> UnionFind<T> {
        let parents: HashMap<T, T> = HashMap::new();
        let rank: HashMap<T, usize> = HashMap::new();
        // for
        UnionFind {
            _size: 0,
            parents,
            rank,
        }
    }

    pub fn with_capacity(size: usize) -> UnionFind<T> {
        let parents: HashMap<T, T> = HashMap::with_capacity(size);
        let rank: HashMap<T, usize> = HashMap::with_capacity(size);
        // for
        UnionFind {
            _size: size,
            parents,
            rank,
        }
    }

    pub fn size(&self) -> usize {
        self._size
    }
}

impl<T> UnionFindTrait<T> for UnionFind<T>
where
    T: Copy + Eq + Hash + Clone,
{
    #[allow(clippy::map_entry)]
    fn find(&mut self, node: T) -> T {
        if !self.parents.contains_key(&node) {
            self.parents.insert(node, node);
            self._size += 1;
        }

        if !(node.eq(self.parents.get(&node).unwrap())) {
            let found = self.find(*self.parents.get(&node).unwrap());
            self.parents.insert(node, found);
        }
        *self.parents.get(&node).unwrap()
    }

    fn union(&mut self, x: T, y: T) {
        let x_root = self.find(x);
        let y_root = self.find(y);

        #[allow(clippy::map_entry)]
        if !self.rank.contains_key(&x_root) {
            self.rank.insert(x_root, 0);
        }

        #[allow(clippy::map_entry)]
        if !self.rank.contains_key(&y_root) {
            self.rank.insert(y_root, 0);
        }
        if x_root.eq(&y_root) {
            return;
        }
        let x_root_rank: usize = *self.rank.get(&x_root).unwrap();
        let y_root_rank: usize = *self.rank.get(&y_root).unwrap();

        if x_root_rank > y_root_rank {
            self.parents.insert(y_root, x_root);
        } else {
            self.parents.insert(x_root, y_root);
            if x_root_rank == y_root_rank {
                self.rank.insert(y_root, y_root_rank + 1);
            }
        }
    }

    fn subsets(&mut self) -> Vec<HashSet<T>> {
        let mut result: HashMap<T, HashSet<T>> = HashMap::with_capacity(self.size());

        let rank_cp = self.rank.clone();

        for (&node, _) in rank_cp.iter() {
            let root = self.find(node);

            #[allow(clippy::map_entry)]
            if !result.contains_key(&root) {
                let mut hset = HashSet::new();
                hset.insert(node);
                result.insert(root, hset);
            } else {
                let prev = &mut *result.get_mut(&root).unwrap();
                prev.insert(node);
            }
        }

        result.into_values().collect()
    }
}

impl<T> IntoIterator for UnionFind<T>
where
    T: Copy + Eq + Hash + Clone,
{
    type Item = HashSet<T>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(mut self) -> Self::IntoIter {
        self.subsets().into_iter()
    }
}

impl<T> fmt::Display for UnionFind<T>
where
    T: Clone + Eq + Hash + Copy + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = format!(
            "<UnionFind size={}, non_trivial_subsets={:?}>",
            self.size(),
            &self.clone().subsets()
        );

        write!(f, "{}", s)
    }
}
