#[allow(unused_imports)]
use crate::tour::between;
use crate::{metric::MetricKind, node::Container, Scalar};

use super::{Tour, TourOrder};

#[allow(dead_code)]
pub fn create_container(n_nodes: usize) -> Container {
    let mut container = Container::new(MetricKind::Euc2d);
    for ii in 0..n_nodes {
        container.add(ii as Scalar, ii as Scalar, ii as Scalar);
    }
    container
}

#[allow(dead_code)]
pub fn test_tour_order(tour: &impl Tour, expected: &TourOrder) {
    let expected = &expected.order;
    let len = expected.len();

    assert_eq!(tour.len(), len);

    for ii in 0..(expected.len() - 1) {
        assert_eq!(
            tour.get(expected[(len + ii - 1) % len]),
            tour.predecessor_at(expected[ii]),
            "Test predecessor at index = {}",
            ii
        );
        assert_eq!(
            tour.get(expected[(ii + 1) % len]),
            tour.successor_at(expected[ii]),
            "Test successor at index = {}",
            ii
        );
    }
}

#[test]
fn test_between() {
    // 1 -> 2 -> 3 -> 4 -> 5
    assert!(between(1, 3, 4)); // true
    assert!(!between(1, 5, 4)); // false
    assert!(between(5, 1, 3)); // true
    assert!(!between(5, 3, 1)); // false
}

#[allow(dead_code, unused_imports)]
mod tests_array {
    use crate::tour::Array;

    use super::*;

    #[test]
    fn test_apply() {
        let container = create_container(10);
        let mut tour = Array::new(&container);
        test_suite::apply(&mut tour);
    }

    #[test]
    fn test_total_dist() {
        let container = create_container(4);
        let mut tour = Array::new(&container);
        test_suite::total_dist(&mut tour);
    }

    #[test]
    #[ignore = "requires reimpl"]
    fn test_between() {
        let container = create_container(10);
        let mut tour = Array::new(&container);
        test_suite::between(&mut tour);
    }

    #[test]
    #[ignore = "requires reimpl"]
    fn test_flip_cases() {
        let container = create_container(100);
        let mut tour = Array::new(&container);
        test_suite::flip(&mut tour);
    }
}

#[allow(dead_code, unused_imports)]
mod tests_tlt {
    use crate::tour::TwoLevelTree;

    use super::*;

    #[test]
    fn test_apply() {
        let container = create_container(10);
        let mut tour = TwoLevelTree::new(&container, 4);
        test_suite::apply(&mut tour);
    }

    #[test]
    fn test_total_dist() {
        let container = create_container(4);
        let mut tour = TwoLevelTree::new(&container, 3);
        test_suite::total_dist(&mut tour);
    }

    #[test]
    fn test_between() {
        let container = create_container(10);
        let mut tour = TwoLevelTree::new(&container, 3);
        test_suite::between(&mut tour);
    }

    #[test]
    fn test_flip_cases() {
        let container = create_container(100);
        let mut tour = TwoLevelTree::new(&container, 10);
        test_suite::flip(&mut tour);
    }

    #[test]
    fn test_segment_reverse() {
        let n_nodes = 10;
        let container = create_container(n_nodes);
        let mut tree = TwoLevelTree::new(&container, 3);

        tree.apply(&TourOrder::new((0..n_nodes).collect()));

        // 0 -> 1 -> 2 -> 5 -> 4 -> 3 -> 6 -> 7 -> 8 -> 9
        tree.segment(1).borrow_mut().reverse();
        test_tour_order(&tree, &TourOrder::new(vec![0, 1, 2, 5, 4, 3, 6, 7, 8, 9]));

        // 0 -> 1 -> 2 -> 5 -> 4 -> 3 -> 8 -> 7 -> 6 -> 9
        tree.segment(2).borrow_mut().reverse();
        let order = TourOrder::new(vec![0, 1, 2, 5, 4, 3, 8, 7, 6, 9]);
        test_tour_order(&tree, &order);

        tree.segment(3).borrow_mut().reverse();
        test_tour_order(&tree, &order);

        // 0 -> 1 -> 2 -> 3 -> 4 -> 5 -> 6 -> 7 -> 8 -> 9
        tree.segment(1).borrow_mut().reverse();
        tree.segment(2).borrow_mut().reverse();
        test_tour_order(&tree, &TourOrder::new((0..10).collect()));
    }

    #[test]
    fn test_split_segment() {
        let n_nodes = 15;
        let container = create_container(n_nodes);
        let mut tree = TwoLevelTree::with_default_order(&container, 5);
        //tree.apply(&TourOrder::new((0..n_nodes).collect()));

        // Original: [0 1 2 3 4] [5 6 7 >8< 9] [10 11 12 13 14]
        tree.split_seg(1, 3);
        test_tour_order(&tree, &TourOrder::new((0..n_nodes).collect()));
        assert_eq!(5, tree.segment(0).borrow().len());
        assert_eq!(3, tree.segment(1).borrow().len());
        assert_eq!(7, tree.segment(2).borrow().len());

        // Reverse a segment
        tree.segment(2).borrow_mut().reverse();
        let mut expected: Vec<usize> = (0..8).collect();
        expected.append(&mut vec![14, 13, 12, 11, 10, 9, 8]);
        let expected = TourOrder::new(expected);
        test_tour_order(&tree, &expected);

        // [8 9 10 11 >12< 13 14] (Reverse)
        tree.split_seg(2, 4);
        test_tour_order(&tree, &expected);
        assert_eq!(5, tree.segment(0).borrow().len());
        assert_eq!(6, tree.segment(1).borrow().len());
        assert_eq!(4, tree.segment(2).borrow().len());
    }
}

#[allow(dead_code, unused_imports)]
mod test_tll {
    use super::*;

    use crate::tour::{
        tests::{create_container, test_tour_order},
        tll::TwoLevelList,
        Tour, TourOrder,
    };

    #[test]
    fn test_apply() {
        let container = create_container(10);
        let mut tour = TwoLevelList::new(&container, 4);
        test_suite::apply(&mut tour);
    }

    #[test]
    fn test_total_dist() {
        let container = create_container(4);
        let mut tour = TwoLevelList::new(&container, 3);
        test_suite::total_dist(&mut tour);
    }

    #[test]
    fn test_between() {
        let container = create_container(10);
        let mut tour = TwoLevelList::new(&container, 3);
        test_suite::between(&mut tour);
    }

    #[test]
    fn test_flip_cases() {
        let container = create_container(100);
        let mut tour = TwoLevelList::new(&container, 10);
        test_suite::flip(&mut tour);
    }
}

#[allow(dead_code)]
mod test_suite {
    use crate::{
        tour::{tests::test_tour_order, Tour, TourOrder},
        Scalar,
    };

    pub fn apply(tour: &mut impl Tour) {
        let expected = TourOrder::new(vec![3, 0, 4, 1, 6, 8, 7, 9, 5, 2]);
        tour.apply(&expected);
        test_tour_order(tour, &expected);
    }

    pub fn total_dist(tour: &mut impl Tour) {
        assert_eq!(4, tour.len());
        tour.apply(&TourOrder::new(vec![0, 1, 2, 3]));
        assert_eq!(6. * (2. as Scalar).sqrt(), tour.total_distance());

        tour.apply(&TourOrder::new(vec![1, 3, 0, 2]));
        assert_eq!(8. * (2. as Scalar).sqrt(), tour.total_distance());
    }

    pub fn between(tour: &mut impl Tour) {
        assert_eq!(10, tour.len());
        tour.apply(&TourOrder::new((0..10).collect()));

        //  0 -> 1 -> 2 -> 3 -> 4 -> 5 -> 6 -> 7 -> 8 -> 9

        // All vertices reside under the same parent node.
        assert!(tour.between_at(0, 1, 2)); // true
        assert!(!tour.between_at(0, 2, 1)); // false
        assert!(!tour.between_at(2, 1, 0)); // false
        assert!(tour.between_at(2, 0, 1)); // true

        // All vertices reside under distinct parent node.
        assert!(tour.between_at(2, 3, 7)); // true
        assert!(!tour.between_at(2, 7, 3)); // true
        assert!(!tour.between_at(7, 3, 2)); // false
        assert!(tour.between_at(7, 2, 3)); // true

        // Two out of three vertices reside under the same parent node.
        assert!(tour.between_at(3, 5, 8)); // true
        assert!(!tour.between_at(3, 8, 5)); // false
        assert!(!tour.between_at(8, 5, 3)); // false
        assert!(tour.between_at(8, 3, 5)); // true

        // Reverse [3 4 5]
        assert!(tour.between_at(3, 4, 5)); // true
        assert!(!tour.between_at(5, 4, 3)); // false

        tour.flip_at(2, 3, 5, 6);

        assert!(!tour.between_at(3, 4, 5)); // false
        assert!(tour.between_at(5, 4, 3)); // true

        assert!(!tour.between_at(3, 5, 8)); // false
        assert!(tour.between_at(3, 8, 5)); // true
        assert!(tour.between_at(8, 5, 3)); // true
        assert!(!tour.between_at(8, 3, 5)); // false
    }

    pub fn flip(tour: &mut impl Tour) {
        flip_1(tour);
        flip_2(tour);
        flip_3(tour);
        flip_4(tour);
    }

    // Test flip case: New paths lie within the same segment.
    fn flip_1(tour: &mut impl Tour) {
        let n_nodes = 100;
        assert_eq!(n_nodes, tour.len());
        tour.apply(&TourOrder::new((0..n_nodes).collect()));

        tour.flip_at(3, 4, 8, 9);
        let mut expected = vec![0, 1, 2, 3, 8, 7, 6, 5, 4, 9];
        expected.append(&mut (10..n_nodes).collect());
        test_tour_order(tour, &TourOrder::new(expected));

        tour.flip_at(3, 8, 4, 9);
        test_tour_order(tour, &TourOrder::new((0..n_nodes).collect()));

        tour.flip_at(8, 9, 3, 4);
        let mut expected = vec![0, 1, 2, 3, 8, 7, 6, 5, 4, 9];
        expected.append(&mut (10..n_nodes).collect());
        test_tour_order(tour, &TourOrder::new(expected));

        tour.flip_at(4, 9, 3, 8);
        let mut expected = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        expected.append(&mut (10..n_nodes).collect());
        test_tour_order(tour, &TourOrder::new(expected));

        // Reverses the entire segment.
        tour.flip_at(9, 10, 19, 20);
        let mut expected: Vec<usize> = (0..10).collect();
        expected.append(&mut (10..20).rev().collect());
        expected.append(&mut (20..n_nodes).collect());
        test_tour_order(tour, &TourOrder::new(expected));

        tour.flip_at(10, 20, 9, 19);
        test_tour_order(tour, &TourOrder::new((0..n_nodes).collect()));
    }

    // Test flip case: New paths consist of a sequence of consecutive segments.
    // This test focuses on inner reverse.
    fn flip_2(tour: &mut impl Tour) {
        let n_nodes = 100;
        assert_eq!(n_nodes, tour.len());
        tour.apply(&TourOrder::new((0..n_nodes).collect()));

        tour.flip_at(9, 10, 39, 40);
        let mut expected: Vec<usize> = (0..10).collect();
        expected.append(&mut (10..40).rev().collect());
        expected.append(&mut (40..n_nodes).collect());
        test_tour_order(tour, &TourOrder::new(expected));

        tour.flip_at(10, 40, 9, 39);
        test_tour_order(tour, &TourOrder::new((0..n_nodes).collect()));

        tour.flip_at(39, 40, 9, 10);
        let mut expected: Vec<usize> = (0..10).collect();
        expected.append(&mut (10..40).rev().collect());
        expected.append(&mut (40..n_nodes).collect());
        test_tour_order(tour, &TourOrder::new(expected));

        tour.flip_at(9, 39, 10, 40);
        test_tour_order(tour, &TourOrder::new((0..n_nodes).collect()));
    }

    // Test flip case: New paths consist of a sequence of consecutive segments.
    // This test focuses on outer reverse.
    fn flip_3(tour: &mut impl Tour) {
        let n_nodes = 100;
        assert_eq!(n_nodes, tour.len());
        tour.apply(&TourOrder::new((0..n_nodes).collect()));

        let mut expected: Vec<usize> = (90..n_nodes).rev().collect();
        expected.append(&mut (10..90).collect());
        expected.append(&mut (0..10).rev().collect());
        tour.flip_at(9, 10, 89, 90);
        test_tour_order(tour, &TourOrder::new(expected));

        tour.flip_at(90, 10, 89, 9);
        test_tour_order(tour, &TourOrder::new((0..n_nodes).collect()));

        let mut expected: Vec<usize> = (90..n_nodes).rev().collect();
        expected.append(&mut (10..90).collect());
        expected.append(&mut (0..10).rev().collect());
        tour.flip_at(89, 90, 9, 10);
        test_tour_order(tour, &TourOrder::new(expected));

        tour.flip_at(89, 9, 90, 10);
        test_tour_order(tour, &TourOrder::new((0..n_nodes).collect()));

        tour.flip_at(79, 80, 89, 90);
        tour.flip_at(9, 10, 79, 89);

        let mut expected: Vec<usize> = (80..90).collect();
        expected.append(&mut (10..80).collect());
        expected.append(&mut (0..10).rev().collect());
        expected.append(&mut (90..n_nodes).rev().collect());
        test_tour_order(tour, &TourOrder::new(expected));
    }

    // Test flip case: Vertices are positioned in the middle of segments.
    fn flip_4(tour: &mut impl Tour) {
        let n_nodes = 100;
        assert_eq!(n_nodes, tour.len());
        let order0 = TourOrder::new((0..n_nodes).collect());
        tour.apply(&order0);
        test_tour_order(tour, &order0);

        tour.flip_at(15, 16, 25, 26);
        let mut expected: Vec<usize> = (0..16).collect();
        expected.append(&mut (16..26).rev().collect());
        expected.append(&mut (26..n_nodes).collect());
        test_tour_order(tour, &TourOrder::new(expected));

        tour.flip_at(15, 25, 16, 26);
        test_tour_order(tour, &order0);

        tour.flip_at(24, 25, 55, 56);
        let mut expected: Vec<usize> = (0..25).collect();
        expected.append(&mut (25..56).rev().collect());
        expected.append(&mut (56..n_nodes).collect());
        test_tour_order(tour, &TourOrder::new(expected));

        tour.flip_at(24, 55, 25, 56);
        test_tour_order(tour, &order0);

        tour.flip_at(12, 13, 92, 93);
        let mut expected: Vec<usize> = (0..13).rev().collect();
        expected.append(&mut (93..n_nodes).rev().collect());
        expected.append(&mut (13..93).collect());
        test_tour_order(tour, &TourOrder::new(expected));
    }
}
