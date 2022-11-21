use crate::prelude::*;

pub fn astar(
    start: impl Point2d,
    end: impl Point2d,
    movement_component: &Movement,
    path_map_provider: &impl PathMapProvider,
    path_map: &impl PathMap,
) -> Option<(Vec<IVec2>, OrderedFloat<f32>)> {
    pathfinding::prelude::astar(
        &start.as_ivec2(),
        |p| path_map.successors(*p).map(|p| (p, path_map_provider.cost(p, movement_component))),
        |p| path_map.distance(*p, end),
        |p| *p == end.as_ivec2(),
    )
}

#[cfg(test)]
mod test {
    use crate::prelude::*;

    struct PathProvider;
    impl PathMapProvider for PathProvider {
        fn cost(&self, node: impl Point2d, _: &Movement) -> OrderedFloat<f32> {
            OrderedFloat(1.0)
        }
    }

    #[test]
    fn right_test() {
        let map = PathMap2d::new_default([10, 10]);

        let start = IVec2::new(0, 0);
        let end = IVec2::new(5, 0);
        let movement_component = Movement(MovementType::Any.as_u8());
        let path = crate::pathfinding::pathfinder::astar(
            start,
            end,
            &movement_component,
            &PathProvider {},
            &map,
        )
        .unwrap();

        assert_eq!(6, path.0.len());
        assert_eq!([0, 0], path.0[0].to_array());
        assert_eq!([5, 0], path.0[5].to_array());
    }

    #[test]
    fn down_test() {
        let map = PathMap2d::new_default([10, 10]);

        let start = IVec2::new(5, 5);
        let end = IVec2::new(5, 0);
        let movement_component = Movement(MovementType::Any.as_u8());
        let path = crate::pathfinding::pathfinder::astar(
            start,
            end,
            &movement_component,
            &PathProvider {},
            &map,
        )
        .unwrap();

        assert_eq!(6, path.0.len());
        assert_eq!([5, 5], path.0[0].to_array());
        assert_eq!([5, 0], path.0[5].to_array());
    }

    #[test]
    fn up_test() {
        let map = PathMap2d::new_default([10, 10]);

        let start = IVec2::new(5, 4);
        let end = IVec2::new(5, 9);
        let movement_component = Movement(MovementType::Any.as_u8());
        let path = crate::pathfinding::pathfinder::astar(
            start,
            end,
            &movement_component,
            &PathProvider {},
            &map,
        )
        .unwrap();

        assert_eq!(6, path.0.len());
        assert_eq!([5, 4], path.0[0].to_array());
        assert_eq!([5, 9], path.0[5].to_array());
    }

    #[test]
    fn left_test() {
        let map = PathMap2d::new_default([10, 10]);

        let start = IVec2::new(9, 5);
        let end = IVec2::new(4, 5);
        let movement_component = Movement(MovementType::Any.as_u8());
        let path = crate::pathfinding::pathfinder::astar(
            start,
            end,
            &movement_component,
            &PathProvider {},
            &map,
        )
        .unwrap();

        assert_eq!(6, path.0.len());
        assert_eq!([9, 5], path.0[0].to_array());
        assert_eq!([4, 5], path.0[5].to_array());
    }

    #[test]
    fn fail_test() {
        let map = PathMap2d::new_default([10, 10]);

        let start = IVec2::new(0, 0);
        let end = IVec2::new(100000, 100000);
        let movement_component = Movement(MovementType::Any.as_u8());
        let path = crate::pathfinding::pathfinder::astar(
            start,
            end,
            &movement_component,
            &PathProvider {},
            &map,
        );

        assert_eq!(path, None);
    }
}
