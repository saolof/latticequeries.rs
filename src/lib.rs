pub mod hiqueries;
pub mod hivecs;
pub mod lattices;

#[cfg(test)]
mod tests {
    use crate::hiqueries::HiQuery;
    use crate::hivecs::HiVec;
    use crate::lattices::{BoundedLattice, FreeL32, Lattice};

    #[test]
    fn test_constructors_accessors() {
        let v = vec![true, false, false, true, true, false, false, false, true];
        let l = v.len();
        let hv: HiVec<_, 3, 2> = HiVec::new(v);
        assert_eq!(l, hv.len());
        assert_eq!(hv.get(2).cloned(), Some(false));
        let q1 = hv.query_equals(true);
        assert_eq!(q1.length(), l);
    }

    #[test]
    fn test_locationqueries() {
        let v = vec![true, false, false, true, true, false, false, false, true];
        let hv: HiVec<_, 3, 2> = HiVec::new(v);
        let q1 = hv.query_equals(true);
        let q2 = hv.query_equals(false);
        println!("Test group 1a");
        assert_eq!(q1.findnext(0), Some(0));
        assert_eq!(q1.findnext(1), Some(3));
        assert_eq!(q1.findnext(7), Some(8));
        assert_eq!(q1.findnext(8), Some(8));
        println!("Test group 1b");
        assert_eq!(q1.findnext(9), None);
        println!("Test group 1c");
        assert_eq!(q1.count(), 4);
        println!("Test group 2a");
        assert_eq!(q2.findnext(0), Some(1));
        assert_eq!(q2.findnext(1), Some(1));
        assert_eq!(q2.findnext(2), Some(2));
        assert_eq!(q2.findnext(3), Some(5));
        assert_eq!(q2.findnext(5), Some(5));
        assert_eq!(q2.findnext(6), Some(6));
        assert_eq!(q2.findnext(7), Some(7));
        println!("Test group 2b");
        assert_eq!(q2.findnext(8), None);
        assert_eq!(q2.findnext(9), None);

        println!("IterTest");
        assert_eq!(q1.count(), 4);
        assert_eq!(q2.count(), 5);
        assert_eq!(q1.count() + q2.count(), hv.len());
    }

    #[test]
    fn test_lattice() {
        let l1 = FreeL32::new(0b000000010010111);
        let l2 = FreeL32::new(0b000001010010100);
        assert_eq!(l1.join(l2).val, 0b000001010010111);
        assert_eq!(l1.meet(l2).val, 0b000000010010100);
        assert_eq!(l1.meet(BoundedLattice::TOP), l1);
        assert_eq!(l1.join(BoundedLattice::BOT), l1);
        assert_eq!(l1.join(BoundedLattice::TOP), BoundedLattice::TOP);
        assert_eq!(l1.meet(BoundedLattice::BOT), BoundedLattice::BOT);
    }
}
