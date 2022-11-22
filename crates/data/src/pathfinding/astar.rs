use crate::prelude::*;

pub fn astar(
    start: impl Point2d,
    end: impl Point2d,
    movement_component: &Movement,
    path_map: &impl PathMap,
) -> Option<(Vec<IVec2>, OrderedFloat<f32>)> {
    pathfinding::prelude::astar(
        &start.as_ivec2(),
        |p| path_map.successors(*p, movement_component),
        |p| path_map.distance(*p, end),
        |p| *p == end.as_ivec2(),
    )
}

#[cfg(test)]
mod test {
    use crate::prelude::*;

    impl PathMap for BitGrid {
        type ExitIterator = arrayvec::IntoIter<(IVec2, OrderedFloat<f32>), 8>;

        /// The cost of moving to a given node
        fn cost(&self, _: impl Point2d, _: &Movement) -> OrderedFloat<f32> {
            1.0.into()
        }

        /// The distance between two node points.
        fn distance(&self, a_node: impl Point2d, b_node: impl Point2d) -> OrderedFloat<f32> {
            a_node.taxi_dist(b_node).into()
        }

        /// Returns an iterator of the valid list of successors for a given node
        fn successors(
            &self,
            node: impl Point2d,
            movement_component: &Movement,
        ) -> Self::ExitIterator {
            let mut points = arrayvec::ArrayVec::new();

            for adj in node.adj_8() {
                if !self.in_bounds(adj) {
                    continue;
                }

                if !self.get_unchecked(adj) {
                    points.push((adj, self.cost(adj, movement_component)));
                }
            }

            points.into_iter()
        }
    }

    #[test]
    fn right_test() {
        let path_map = BitGrid::new_default([10, 10]);
        let start = IVec2::new(0, 0);
        let end = IVec2::new(5, 0);
        let movement_component = Movement(MovementType::Any.as_u8());
        let path = PathFinder::Astar.compute(start, end, &movement_component, &path_map).unwrap();

        assert_eq!(6, path.0.len());
        assert_eq!([0, 0], path.0[0].to_array());
        assert_eq!([5, 0], path.0[5].to_array());
    }

    #[test]
    fn down_test() {
        let path_map = BitGrid::new_default([10, 10]);
        let start = IVec2::new(5, 5);
        let end = IVec2::new(5, 0);
        let movement_component = Movement(MovementType::Any.as_u8());
        let path = PathFinder::Astar.compute(start, end, &movement_component, &path_map).unwrap();

        assert_eq!(6, path.0.len());
        assert_eq!([5, 5], path.0[0].to_array());
        assert_eq!([5, 0], path.0[5].to_array());
    }

    #[test]
    fn up_test() {
        let map = BitGrid::new_default([10, 10]);

        let start = IVec2::new(5, 4);
        let end = IVec2::new(5, 9);
        let movement_component = Movement(MovementType::Any.as_u8());
        let path = PathFinder::Astar.compute(start, end, &movement_component, &map).unwrap();

        assert_eq!(6, path.0.len());
        assert_eq!([5, 4], path.0[0].to_array());
        assert_eq!([5, 9], path.0[5].to_array());
    }

    #[test]
    fn left_test() {
        let map = BitGrid::new_default([10, 10]);

        let start = IVec2::new(9, 5);
        let end = IVec2::new(4, 5);
        let movement_component = Movement(MovementType::Any.as_u8());
        let path = PathFinder::Astar.compute(start, end, &movement_component, &map).unwrap();

        assert_eq!(6, path.0.len());
        assert_eq!([9, 5], path.0[0].to_array());
        assert_eq!([4, 5], path.0[5].to_array());
    }

    #[test]
    fn fail_test() {
        let map = BitGrid::new_default([1, 1]);

        let start = IVec2::new(0, 0);
        let end = IVec2::new(11, 11);
        let movement_component = Movement(MovementType::Any.as_u8());
        let path = PathFinder::Astar.compute(start, end, &movement_component, &map);

        assert_eq!(path, None);
    }
}
