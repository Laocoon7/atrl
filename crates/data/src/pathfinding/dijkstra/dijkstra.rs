use arrayvec::ArrayVec;
use pathfinding::prelude::{build_path, dijkstra, dijkstra_partial};

use super::super::shared::*;
use crate::prelude::*;

pub struct Dijkstras;

impl PathAlgorithm for Dijkstras {
    fn compute_path(
        origin: Position,
        destination: Position,
        movement_type: u8,
        partial_path_on_failure: bool,
        provider: &impl PathProvider,
    ) -> Option<Vec<Position>> {
        let dijkstra_path = dijkstra(
            &origin,
            |pt| Self::neighbors(provider, movement_type, pt),
            |pt| *pt == destination,
        )
        .map(|(path, _cost)| path);

        if partial_path_on_failure {
            dijkstra_path.or_else(|| {
                let (paths, _) = dijkstra_partial(
                    &origin,
                    |pt| Self::neighbors(provider, movement_type, pt),
                    |&pt| pt == destination,
                );

                let target = paths
                    .iter()
                    .min_by(|(a_pt, (_, a_cost)), (b_pt, (_, b_cost))| {
                        let dist1 = DistanceAlg::Pythagoras.distance2d(**a_pt, destination);
                        let dist2 = DistanceAlg::Pythagoras.distance2d(**b_pt, destination);
                        (dist1 + *a_cost as f32).total_cmp(&(dist2 + *b_cost as f32))
                    })
                    .map(|(pt, _)| pt)
                    .unwrap_or(&origin);

                Some(build_path(target, &paths))
            })
        } else {
            dijkstra_path
        }
    }
}

impl Dijkstras {
    pub fn neighbors(
        provider: &impl PathProvider,
        movement_type: u8,
        pt: &IVec2,
    ) -> arrayvec::IntoIter<(IVec2, u32), 8> {
        let mut neighbors = ArrayVec::<(IVec2, u32), 8>::new();

        pt.neighbors_all().for_each(|pt| {
            if provider.is_walkable(pt, movement_type) {
                neighbors.push((pt, provider.cost(pt, movement_type)));
            }
        });

        neighbors.into_iter()
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::*;

    pub struct TestPathProvider(pub Vec<IVec2>);

    impl PathProvider for TestPathProvider {
        fn is_walkable(&self, position: IVec2, _movement_type: u8) -> bool {
            if position.x > 0 && position.x < 10 && position.y > 0 && position.y < 10 {
                !self.0.contains(&position)
            } else {
                false
            }
        }

        fn cost(&self, _position: IVec2, _movement_type: u8) -> u32 { 1 }
    }

    #[test]
    pub fn happy_dijkstras() {
        let path = PathFinder::Dijkstras.compute(
            (0, 0),
            (9, 9),
            0,
            false,
            &TestPathProvider(vec![IVec2::new(5, 5), IVec2::new(5, 6)]),
        );
        assert!(path.is_some());

        let mut canvas = Canvas::new([10, 10]);
        canvas.put((5, 5), '#');
        canvas.put((5, 6), '#');

        path.unwrap().iter().for_each(|pt| {
            canvas.put(*pt, '*');
        });
        canvas.print();
    }

    #[test]
    pub fn happy_dijkstra_with_partial() {
        static BLOCKED_LINE: Lazy<Vec<IVec2>> = Lazy::new(|| {
            vec![
                IVec2::new(0, 5),
                IVec2::new(1, 5),
                IVec2::new(2, 5),
                IVec2::new(3, 5),
                IVec2::new(4, 5),
                IVec2::new(5, 5),
                IVec2::new(6, 5),
                IVec2::new(7, 5),
                IVec2::new(8, 5),
                IVec2::new(9, 5),
            ]
        });

        let path = PathFinder::Dijkstras.compute(
            (0, 0),
            (9, 9),
            0,
            true,
            &TestPathProvider(BLOCKED_LINE.clone()),
        );
        assert!(path.is_some());

        let mut canvas = Canvas::new([10, 10]);
        BLOCKED_LINE.iter().for_each(|pt| {
            canvas.put(*pt, '#');
        });

        path.unwrap().iter().for_each(|pt| {
            canvas.put(*pt, '*');
        });
        canvas.print();
    }
}
