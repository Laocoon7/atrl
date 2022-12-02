use crate::prelude::*;
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Polygon {
    points: Vec<IVec2>,
    fpoints: Vec<Vec2>,
    is_regular: bool,
    center: IVec2,
    is_convex: bool,
}
impl Polygon {
    pub fn new(points: Vec<impl GridPoint>) -> Self {
        let points: Vec<IVec2> = points.into_iter().map(|p| p.as_ivec2()).collect();
        let fpoints = points.iter().map(|p| p.as_vec2()).collect();
        let is_convex = is_convex(&points);
        let mut poly = Self {
            points: points.clone(),
            fpoints,
            center: IVec2::default(),
            is_regular: false,
            is_convex,
        };
        poly.center = IVec2::new(poly.left(), poly.top()).mid_point(IVec2::new(poly.right(), poly.bottom()));
        let dists: Vec<f32> =
            points.iter().map(|p| DistanceAlg::PythagorasSquared.distance2d(*p, poly.center)).collect();
        poly.is_regular = dists.iter().all(|dist| dist == &dists[0]);
        poly
    }
}
impl Polygon {
    #[inline]
    pub const fn fpoints(&self) -> &Vec<Vec2> { &self.fpoints }

    #[inline]
    pub const fn is_regular(&self) -> bool { self.is_regular }

    pub fn point_closest_to_center(&self) -> IVec2 {
        let mut list = self.points.clone();
        list.sort_by_key(|p| DistanceAlg::PythagorasSquared.distance2d(*p, self.center) as i32);
        list[0]
    }

    pub fn point_farthest_from_center(&self) -> IVec2 {
        let mut list = self.points.clone();
        list.sort_by_key(|p| DistanceAlg::PythagorasSquared.distance2d(*p, self.center) as i32);
        *list.last().unwrap()
    }

    #[inline]
    pub const fn is_convex(&self) -> bool { self.is_convex }
}
impl Shape for Polygon {
    fn from_points(points: Vec<impl GridPoint>) -> Self
    where Self: Sized {
        Self::new(points)
    }

    fn contains(&self, point: impl GridPoint) -> bool {
        let fpoint = point.as_vec2();
        let mut j = self.fpoints.len() - 1;
        let mut odd_number_of_nodes = false;
        for i in 0..self.fpoints.len() {
            if (self.fpoints[i].y < fpoint.y && self.fpoints[j].y >= fpoint.y ||
                self.fpoints[j].y < fpoint.y && self.fpoints[i].y >= fpoint.y) &&
                (self.fpoints[i].x <= fpoint.x || self.fpoints[j].x <= fpoint.x)
            {
                odd_number_of_nodes ^= ((fpoint.y - self.fpoints[i].y) /
                    (self.fpoints[j].y - self.fpoints[i].y))
                    .mul_add(self.fpoints[j].x - self.fpoints[i].x, self.fpoints[i].x) <
                    fpoint.x;
            }
            j = i;
        }
        odd_number_of_nodes
    }

    fn points(&self) -> Vec<IVec2> { self.points.clone() }

    #[inline]
    fn center(&self) -> IVec2 { self.center }
}
impl Polygon {
    /// Creates a circle using the point closest to the center
    // pub fn as_inner_circle(&self) -> Circle {
    //     Circle::from_points(vec![self.center, self.point_closest_to_center()])
    // }

    /// Creates a circle using the point farthest to the center
    // pub fn as_outer_circle(&self) -> Circle {
    //     Circle::from_points(vec![self.center, self.point_farthest_from_center()])
    // }

    /// Creates a circle using the average point distance from the center
    pub fn as_avg_circle(&self) -> Circle {
        let total: f32 =
            self.points.iter().map(|p| DistanceAlg::Pythagoras.distance2d(*p, self.center)).sum();
        let radius = total / self.points.len() as f32;
        Circle::new(self.center, radius.floor() as u32)
    }

    /// If the polygon is regular then it returns a circle from center to the first point
    // pub fn as_circle(&self) -> Option<Circle> {
    //     if self.is_regular {
    //         Some(Circle::from_points(vec![self.center, self.points[0]]))
    //     } else {
    //         None
    //     }
    // }

    pub fn as_rect(&self) -> Rectangle {
        Rectangle::new((self.left(), self.top()), (self.right(), self.bottom()))
    }

    /// Cuts shape into triangles, triangles will be from the center to the edge
    /// This only works on convex polygons
    pub fn as_triangles(&self) -> Option<Vec<Triangle>> {
        if !self.is_convex {
            return None;
        }
        let mut output = vec![];
        for p in self.points.windows(2) {
            output.push(Triangle::new(p[0], p[1], self.center));
        }
        output.push(Triangle::new(
            *self.points.last().unwrap(),
            self.points[0],
            self.center,
        ));
        Some(output)
    }
}
fn is_convex(points: &Vec<IVec2>) -> bool {
    let mut prev = 0;
    for i in 0..points.len() {
        let product = (points[(i + 1) % points.len()] - points[i])
            .cross_product(points[(i + 2) % points.len()] - points[i]);
        if product != 0 {
            if product * prev < 0 {
                return false;
            } else {
                prev = product;
            }
        }
    }
    true
}
