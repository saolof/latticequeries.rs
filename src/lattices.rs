use std::cmp::{max, min, Ordering};
use std::fmt::{Formatter, Write};

pub trait Lattice: PartialOrd {
    fn join(self, other: Self) -> Self;
    fn meet(self, other: Self) -> Self;
}

pub trait BoundedLattice: Lattice {
    const TOP: Self;
    const BOT: Self;
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

// impl<T : Lattice> PartialOrd<T> for LatticeRange<T> {
//     fn partial_cmp(&self, other : &Self) -> Option<Ordering> {

//     }
// }

// impl<T : Lattice> Lattice for LatticeRange<T> {
//     fn meet(self, other : Self) -> Self {self.intersect(other)}
//     fn join(self, other : Self) -> Self {self.unite(other)}
// }

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct FreeL32 {
    pub val: u32,
}

impl FreeL32 {
    pub fn new(i: u32) -> Self {
        Self { val: i }
    }

    pub fn generator(i: usize) -> Self {
        Self { val: 1 << i }
    }

    pub fn complement(&self) -> Self {
        Self { val: !self.val }
    }
}

impl PartialOrd for FreeL32 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let i1 = self.val & !other.val;
        let i2 = !self.val & other.val;
        match (i1, i2) {
            (0, 0) => Some(Ordering::Equal),
            (0, _) => Some(Ordering::Less),
            (_, 0) => Some(Ordering::Greater),
            _ => None,
        }
    }
}

impl Lattice for FreeL32 {
    fn meet(self, other: Self) -> Self {
        Self {
            val: self.val & other.val,
        }
    }
    fn join(self, other: Self) -> Self {
        Self {
            val: self.val | other.val,
        }
    }
}

impl BoundedLattice for FreeL32 {
    const TOP: Self = Self { val: !0 };
    const BOT: Self = Self { val: 0 };
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct FreeL64 {
    pub val: u64,
}

impl FreeL64 {
    pub fn new(i: u64) -> Self {
        Self { val: i }
    }

    pub fn generator(i: usize) -> Self {
        Self { val: 1 << i }
    }

    pub fn complement(&self) -> Self {
        Self { val: !self.val }
    }
}

impl PartialOrd for FreeL64 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let i1 = self.val & !other.val;
        let i2 = !self.val & other.val;
        match (i1, i2) {
            (0, 0) => Some(Ordering::Equal),
            (0, _) => Some(Ordering::Less),
            (_, 0) => Some(Ordering::Greater),
            _ => None,
        }
    }
}

impl Lattice for FreeL64 {
    fn meet(self, other: Self) -> Self {
        Self {
            val: self.val & other.val,
        }
    }
    fn join(self, other: Self) -> Self {
        Self {
            val: self.val | other.val,
        }
    }
}

impl BoundedLattice for FreeL64 {
    const TOP: Self = Self { val: !0 };
    const BOT: Self = Self { val: 0 };
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd)]
pub struct AlphaNumSet {
    pub val: FreeL64,
}

fn alphanum_to_u8offset(ch: char) -> u8 {
    match ch {
        '0'..='9' => ch as u8 - '0' as u8,
        'A'..='Z' => ch as u8 - 'A' as u8 + 10, // 10 to 35
        'a'..='z' => ch as u8 - 'a' as u8 + 36, // 36 to 61
        _ => {
            if ch.is_ascii() {
                62
            } else {
                63
            }
        }
    }
}

fn u8offset_to_alphanum(n: u8) -> char {
    match n {
        0..=9 => (n + '0' as u8) as char,
        10..=35 => (n - 10 + 'A' as u8) as char,
        36..=61 => (n - 36 + 'a' as u8) as char,
        62 => ':',
        _ => '?',
    }
}

impl AlphaNumSet {
    fn new(s: &str) -> Self {
        let mut bs: u64 = 0;
        for ch in s.chars() {
            bs |= 1 << alphanum_to_u8offset(ch)
        }
        Self {
            val: FreeL64::new(bs),
        }
    }

    fn singleton(ch: char) -> Self {
        Self {
            val: FreeL64::generator(alphanum_to_u8offset(ch) as usize),
        }
    }

    fn complement(&self) -> Self {
        Self {
            val: self.val.complement(),
        }
    }
}

impl Lattice for AlphaNumSet {
    fn join(self, other: Self) -> Self {
        Self {
            val: self.val.join(other.val),
        }
    }
    fn meet(self, other: Self) -> Self {
        Self {
            val: self.val.meet(other.val),
        }
    }
}

impl BoundedLattice for AlphaNumSet {
    const TOP: Self = AlphaNumSet { val: FreeL64::TOP };
    const BOT: Self = AlphaNumSet { val: FreeL64::BOT };
}

impl std::fmt::Display for AlphaNumSet {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        let v: u64 = self.val.val;
        for i in 0..=63 {
            if v & (1 << i) != 0 {
                f.write_char(u8offset_to_alphanum(i));
            }
        }
        Ok(())
    }
}
