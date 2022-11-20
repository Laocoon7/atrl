use crate::prelude::*;

pub fn astar(
    map: &impl PathMap,
    start: impl Point2d,
    end: impl Point2d,
) -> Option<(Vec<IVec2>, OrderedFloat<f32>)> {
    pathfinding::prelude::astar(
        &start.as_ivec2(),
        |p| map.successors(p),
        |p| map.distance(*p, end),
        |p| *p == end.as_ivec2(),
    )
}

#[cfg(test)]
mod test {
    use crate::prelude::*;

    #[test]
    fn right_test() {
        let map = PathMap2d::new_default([10, 10]);

        let start = IVec2::new(0, 0);
        let end = IVec2::new(2, 0);
        let path = crate::pathfinding::pathfinder::astar(&map, start, end).unwrap();

        println!("{:?}", path);
        assert_eq!(6, path.0.len());
        assert_eq!([0, 0], path.0[0].to_array());
        assert_eq!([5, 0], path.0[5].to_array());
    }

    #[test]
    fn down_test() {
        let map = PathMap2d::new_default([10, 10]);

        let start = IVec2::new(5, 5);
        let end = IVec2::new(5, 0);
        let path = crate::pathfinding::pathfinder::astar(&map, start, end).unwrap();

        assert_eq!(6, path.0.len());
        assert_eq!([5, 5], path.0[0].to_array());
        assert_eq!([5, 0], path.0[5].to_array());
    }

    #[test]
    fn up_test() {
        let map = PathMap2d::new_default([10, 10]);

        let start = IVec2::new(5, 4);
        let end = IVec2::new(5, 9);
        let path = crate::pathfinding::pathfinder::astar(&map, start, end).unwrap();

        assert_eq!(6, path.0.len());
        assert_eq!([5, 4], path.0[0].to_array());
        assert_eq!([5, 9], path.0[5].to_array());
    }

    #[test]
    fn left_test() {
        let map = PathMap2d::new_default([10, 10]);

        let start = IVec2::new(9, 5);
        let end = IVec2::new(4, 5);
        let path = crate::pathfinding::pathfinder::astar(&map, start, end).unwrap();

        assert_eq!(6, path.0.len());
        assert_eq!([9, 5], path.0[0].to_array());
        assert_eq!([4, 5], path.0[5].to_array());
    }

    #[test]
    fn fail_test() {
        let map = PathMap2d::new_default([10, 10]);

        let start = IVec2::new(0, 0);
        let end = IVec2::new(100000, 100000);
        let path = crate::pathfinding::pathfinder::astar(&map, start, end);

        assert_eq!(path, None);
    }
}
