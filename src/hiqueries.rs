use std::sync::Arc;

/*
Trait for lazy heirarchical query objects. Building a tree of them lets you
build boolean queries, and consider which power-of-FANOUT blocks in a vec contain a
solution to said query, so that they can be found with binary search.

The iter() method gives you an iterator over the indices of the query results.
*/
pub trait HiQuery<const N: usize, const FANOUT: usize> {
    fn length(&self) -> usize;
    fn query_at(&self, i: usize) -> bool;

    /*
    Should return whether query_at(i) is true for a chunk of length FANOUT^layer elements.
    */
    fn hiquery(&self, layer: usize, i: usize) -> bool; // Layers in range 0 ..= N

    /*
    Finds the next index after i (including i itself) that for which queryat(i) is true.
    */
    fn findnext(&self, mut i: usize) -> Option<usize> {
        while i < self.length() {
            if self.query_at(i) {
                return Some(i);
            }
            let mut step = 1;
            let mut l = 0;
            let mut j = i;
            while l < N && j % FANOUT == 0 && self.hiquery(l, j) {
                l += 1;
                j /= FANOUT;
                step *= FANOUT;
            }
            i += step;
        }
        None
    }

    fn count(&self) -> usize {
        let mut n = 0;
        let mut i = 0;
        while let Some(j) = self.findnext(i) {
            i = j + 1;
            n += 1;
        }
        return n;
    }

    fn and<Q2: HiQuery<N, FANOUT> + Sized>(
        self: Arc<Self>,
        other: Arc<Q2>,
    ) -> AndQuery<Self, Q2, N, FANOUT>
    where
        Self: Sized,
    {
        assert_eq!(self.length(), other.length());
        AndQuery {
            q1: self,
            q2: other,
        }
    }

    fn or<Q2: HiQuery<N, FANOUT> + Sized>(
        self: Arc<Self>,
        other: Arc<Q2>,
    ) -> OrQuery<Self, Q2, N, FANOUT>
    where
        Self: Sized,
    {
        assert_eq!(self.length(), other.length());
        OrQuery {
            q1: self,
            q2: other,
        }
    }

    fn iter(&self) -> HiQIter<Self, N, FANOUT>
    where
        Self: Sized,
    {
        HiQIter { hq: self, i: 0 }
    }

    fn rc(self) -> Arc<Self>
    where
        Self: Sized,
    {
        Arc::new(self)
    }
}

pub trait NegatableQuery<const N: usize, const FANOUT: usize>: HiQuery<N, FANOUT> {
    type NegType: NegatableQuery<N, FANOUT>;
    fn negation(self: &Arc<Self>) -> Self::NegType;
}

#[derive(Clone)]
pub struct AndQuery<Q1, Q2, const N: usize, const FANOUT: usize> {
    q1: Arc<Q1>,
    q2: Arc<Q2>,
}
#[derive(Clone)]
pub struct OrQuery<Q1, Q2, const N: usize, const FANOUT: usize> {
    q1: Arc<Q1>,
    q2: Arc<Q2>,
}

impl<Q1, Q2, const N: usize, const FANOUT: usize> HiQuery<N, FANOUT> for AndQuery<Q1, Q2, N, FANOUT>
where
    Q1: HiQuery<N, FANOUT>,
    Q2: HiQuery<N, FANOUT>,
{
    fn query_at(&self, i: usize) -> bool {
        self.q1.query_at(i) && self.q2.query_at(i)
    }
    fn hiquery(&self, layer: usize, i: usize) -> bool {
        self.q1.hiquery(layer, i) && self.q2.hiquery(layer, i)
    }
    fn length(&self) -> usize {
        self.q1.length()
    }
}

impl<Q1, Q2, const N: usize, const FANOUT: usize> HiQuery<N, FANOUT> for OrQuery<Q1, Q2, N, FANOUT>
where
    Q1: HiQuery<N, FANOUT>,
    Q2: HiQuery<N, FANOUT>,
{
    fn query_at(&self, i: usize) -> bool {
        self.q1.query_at(i) || self.q2.query_at(i)
    }
    fn hiquery(&self, layer: usize, i: usize) -> bool {
        self.q1.hiquery(layer, i) || self.q2.hiquery(layer, i)
    }
    fn length(&self) -> usize {
        self.q1.length()
    }
}

impl<Q1, Q2, const N: usize, const FANOUT: usize> NegatableQuery<N, FANOUT>
    for AndQuery<Q1, Q2, N, FANOUT>
where
    Q1: NegatableQuery<N, FANOUT>,
    Q2: NegatableQuery<N, FANOUT>,
{
    type NegType = OrQuery<Q1::NegType, Q2::NegType, N, FANOUT>;

    fn negation(self: &Arc<Self>) -> Self::NegType {
        Self::NegType {
            q1: self.q1.negation().rc(),
            q2: self.q2.negation().rc(),
        }
    }
}

impl<Q1, Q2, const N: usize, const FANOUT: usize> NegatableQuery<N, FANOUT>
    for OrQuery<Q1, Q2, N, FANOUT>
where
    Q1: NegatableQuery<N, FANOUT>,
    Q2: NegatableQuery<N, FANOUT>,
{
    type NegType = AndQuery<Q1::NegType, Q2::NegType, N, FANOUT>;

    fn negation(self: &Arc<Self>) -> Self::NegType {
        Self::NegType {
            q1: self.q1.negation().rc(),
            q2: self.q2.negation().rc(),
        }
    }
}

pub struct HiQIter<'a, T: HiQuery<N, FANOUT>, const N: usize, const FANOUT: usize> {
    hq: &'a T,
    i: usize,
}

impl<'a, T: HiQuery<N, FANOUT>, const N: usize, const FANOUT: usize> Iterator
    for HiQIter<'a, T, N, FANOUT>
{
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        self.hq.findnext(self.i).map(|i| {
            self.i = i + 1;
            i
        })
    }
}
