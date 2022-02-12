use crate::hiqueries::HiQuery;
use crate::lattices::{Lattice, LatticeRange};

#[derive(Debug, Clone)]
pub struct HiVec<T, const N: usize, const FANOUT: usize> {
    table: Vec<T>,
    layers: Vec<Vec<LatticeRange<T>>>,
}

impl<T: Copy + Lattice, const N: usize, const FANOUT: usize> HiVec<T, N, FANOUT> {
    pub fn new(table: Vec<T>) -> Self {
        let mut layers: Vec<Vec<LatticeRange<T>>> = Vec::with_capacity(N);
        let ranges = table
            .chunks(FANOUT)
            .map(|chunk| {
                let bot = chunk
                    .iter()
                    .cloned()
                    .reduce(|x, y| x.meet(y))
                    .expect("Impossible: Empty Chunk");
                let top = chunk
                    .iter()
                    .cloned()
                    .reduce(|x, y| x.meet(y))
                    .expect("Impossible: Empty Chunk");
                LatticeRange::new(top, bot)
            })
            .collect();
        layers.push(ranges);
        for l in 1..N {
            let nextlayer = layers[l - 1]
                .chunks(FANOUT)
                .map(|chunk| {
                    chunk
                        .iter()
                        .cloned()
                        .reduce(|x, y| x.unite(y))
                        .expect("Impossible: Empty Chunk")
                })
                .collect();
            layers.push(nextlayer)
        }
        HiVec { table, layers }
    }

    fn repair_invariant(&mut self, range: std::ops::RangeInclusive<usize>) {
        let range = (range.start() - range.start() % FANOUT)
            ..=(range.end() - range.end() % FANOUT + FANOUT - 1);
        let nriter = self.table[range.clone()].chunks(FANOUT).map(|chunk| {
            chunk
                .iter()
                .fold(None, |r: Option<LatticeRange<T>>, &e| {
                    Some(if let Some(r) = r {
                        r.expandby(e)
                    } else {
                        LatticeRange::singleton(e)
                    })
                })
                .expect("Impossible: empty chunk")
        });
        let s = range.start() / FANOUT;
        for (i, r) in nriter.enumerate() {
            self.layers[0][s + i] = r;
        }
        let mut range = s..=((range.end() + 1) / FANOUT);
        for n in 1..N {
            let (prevlayer, nextlayer) = self.layers.split_at_mut(n);
            range = (range.start() - range.start() % FANOUT)
                ..=(range.end() - range.end() % FANOUT + FANOUT - 1);
            let it = prevlayer.last().expect("Impossible: prevlayer empty")[range.clone()]
                .chunks(FANOUT)
                .map(|chunk| chunk.iter().cloned().reduce(|x, y| x.unite(y)).unwrap());
            let s = range.start() / FANOUT;
            for (i, r) in it.enumerate() {
                nextlayer[0][s + i] = r;
            }
            range = s..=((range.end() + 1) / FANOUT);
        }
    }

    pub fn len(&self) -> usize {
        self.table.len()
    }

    pub fn get(&self, i: usize) -> Option<&T> {
        self.table.get(i)
    }

    pub fn mutate(&mut self, i: usize, f: impl FnOnce(&mut T)) {
        self.table.get_mut(i).map(f);
        self.repair_invariant(i..=i);
    }

    pub fn query_equals(&self, item: T) -> EqualsQuery<T, N, FANOUT> {
        EqualsQuery { item, hiv: self }
    }

    pub fn query_range(&self, range: LatticeRange<T>) -> RangeQuery<T, N, FANOUT> {
        RangeQuery { range, hiv: self }
    }


}

pub struct EqualsQuery<'a, T, const N: usize, const FANOUT: usize> {
    item: T,
    hiv: &'a HiVec<T, N, FANOUT>,
}

impl<'a, T: Lattice + Copy, const N: usize, const FANOUT: usize> HiQuery<N, FANOUT>
    for EqualsQuery<'a, T, N, FANOUT>
{
    fn length(&self) -> usize {
        self.hiv.len()
    }
    fn query_at(&self, i: usize) -> bool {
        self.hiv
            .get(i)
            .map(|&x| x == self.item)
            .expect("Out of bounds")
    }
    fn hiquery(&self, layer: usize, i: usize) -> bool {
        if layer == 0 {
            self.query_at(i)
        } else {
            self.hiv.layers[layer - 1][i].contains(&self.item)
        }
    }
}

pub struct RangeQuery<'a, T, const N: usize, const FANOUT: usize> {
    range: LatticeRange<T>,
    hiv: &'a HiVec<T, N, FANOUT>,
}

impl<'a, T: Lattice + Copy, const N: usize, const FANOUT: usize> HiQuery<N, FANOUT>
    for RangeQuery<'a, T, N, FANOUT>
{
    fn length(&self) -> usize {
        self.hiv.len()
    }
    fn query_at(&self, i: usize) -> bool {
        self.hiv
            .get(i)
            .map(|x| self.range.contains(x))
            .expect("Out of bounds")
    }
    fn hiquery(&self, layer: usize, i: usize) -> bool {
        if layer == 0 {
            self.query_at(i)
        } else {
            !(self.hiv.layers[layer - 1][i]
                .intersect(self.range)
                .isempty())
        }
    }
}
