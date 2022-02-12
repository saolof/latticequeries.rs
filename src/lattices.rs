use std::cmp::{max, min, Ordering};

pub trait Lattice: PartialOrd {
    fn join(self, other: Self) -> Self;
    fn meet(self, other: Self) -> Self;
}

pub trait BoundedLattice: Lattice {
    const TOP : Self;
    const BOT : Self;
}

impl<T: Ord> Lattice for T {
    fn join(self, other: T) -> T {
        max(self, other)
    }
    fn meet(self, other: T) -> T {
        min(self, other)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LatticeRange<T> {
    top: T,
    bottom: T,
}

impl<T: Lattice> LatticeRange<T> {
    pub fn new(top: T, bottom: T) -> LatticeRange<T> {
        LatticeRange { top, bottom }
    }
    pub fn singleton(x: T) -> LatticeRange<T>
    where
        T: Clone,
    {
        LatticeRange {
            top: x.clone(),
            bottom: x,
        }
    }
    pub fn isempty(&self) -> bool {
        self.top >= self.bottom
    }
    pub fn contains(&self, x: &T) -> bool {
        self.top >= *x && self.bottom >= *x
    }
    pub fn expandby(&self, x: T) -> Self
    where
        T: Clone,
    {
        LatticeRange {
            top: self.top.clone().join(x.clone()),
            bottom: self.bottom.clone().meet(x),
        }
    }
    pub fn unite(self, other: Self) -> Self {
        LatticeRange {
            top: self.top.join(other.top),
            bottom: self.bottom.meet(other.bottom),
        }
    }
    pub fn intersect(self, other: Self) -> Self {
        LatticeRange {
            top: self.top.meet(other.top),
            bottom: self.bottom.join(other.bottom),
        }
    }
}

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub struct FreeL32 {
    pub val : u32
}

impl FreeL32 {
    pub fn new(i : u32) -> Self {
        FreeL32{val: i}
    }

    pub fn generator(i : usize) -> Self {
        FreeL32{ val : 1 << i}
    }
}



impl PartialOrd for FreeL32 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let i1 = self.val & !other.val;
        let i2 = !self.val & other.val;
        match (i1,i2) {
            (0,0) => Some(Ordering::Equal),
            (0,_) => Some(Ordering::Less),
            (_,0) => Some(Ordering::Greater),
            _ => None
        }
    }
}

impl Lattice for FreeL32 {
    fn meet(self,other: Self) -> Self {
        FreeL32 {val: self.val & other.val}
    }
    fn join(self,other: Self) -> Self {
        FreeL32 {val: self.val | other.val}
    }
}

impl BoundedLattice for FreeL32 {
    const TOP : Self = FreeL32 {val : !0};
    const BOT : Self = FreeL32 {val : 0};
}


// impl<T : Lattice> PartialOrd<T> for LatticeRange<T> {
//     fn partial_cmp(&self, other : &Self) -> Option<Ordering> {

//     }
// }

// impl<T : Lattice> Lattice for LatticeRange<T> {
//     fn meet(self, other : Self) -> Self {self.intersect(other)}
//     fn join(self, other : Self) -> Self {self.unite(other)}
// }
