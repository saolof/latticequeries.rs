pub trait HiQuery<const N: usize, const FANOUT: usize> {
    fn length(&self) -> usize;

    fn query_at(&self, i: usize) -> bool;
    fn hiquery(&self, layer: usize, i: usize) -> bool; // Layers in range 0 ..= N

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

    fn not<'a>(&'a self) -> NotQuery<'a, N, FANOUT>
    where
        Self: Sized,
    {
        NotQuery { q: self }
    }
    fn and<'a, 'b>(
        &'a self,
        other: &'b (impl HiQuery<N, FANOUT> + Sized),
    ) -> AndQuery<'a, 'b, N, FANOUT>
    where
        Self: Sized,
    {
        assert_eq!(self.length(), other.length());
        AndQuery {
            q1: self,
            q2: other,
        }
    }

    fn or<'a, 'b>(
        &'a self,
        other: &'b (impl HiQuery<N, FANOUT> + Sized),
    ) -> OrQuery<'a, 'b, N, FANOUT>
    where
        Self: Sized,
    {
        assert_eq!(self.length(), other.length());
        OrQuery {
            q1: self,
            q2: other,
        }
    }
}

#[derive(Copy, Clone)]
pub struct NotQuery<'a, const N: usize, const FANOUT: usize> {
    q: &'a dyn HiQuery<N, FANOUT>,
}
#[derive(Copy, Clone)]
pub struct AndQuery<'a, 'b, const N: usize, const FANOUT: usize> {
    q1: &'a dyn HiQuery<N, FANOUT>,
    q2: &'b dyn HiQuery<N, FANOUT>,
}
#[derive(Copy, Clone)]
pub struct OrQuery<'a, 'b, const N: usize, const FANOUT: usize> {
    q1: &'a dyn HiQuery<N, FANOUT>,
    q2: &'b dyn HiQuery<N, FANOUT>,
}

impl<'a, const N: usize, const FANOUT: usize> HiQuery<N, FANOUT> for NotQuery<'a, N, FANOUT> {
    fn query_at(&self, i: usize) -> bool {
        self.q.query_at(i)
    }
    fn hiquery(&self, layer: usize, i: usize) -> bool {
        self.q.hiquery(layer, i)
    }
    fn length(&self) -> usize {
        self.q.length()
    }
}

impl<'a, 'b, const N: usize, const FANOUT: usize> HiQuery<N, FANOUT>
    for AndQuery<'a, 'b, N, FANOUT>
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

impl<'a, 'b, const N: usize, const FANOUT: usize> HiQuery<N, FANOUT>
    for OrQuery<'a, 'b, N, FANOUT>
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
