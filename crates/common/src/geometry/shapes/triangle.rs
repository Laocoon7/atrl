use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub enum TriangleAngleType {
    Acute,
    Right,
    Obtuse,
    Equiangular,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub enum TriangleSideType {
    Isosceles,
    Scalene,
    Equilateral,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Triangle {
    points: [IVec2; 3],
    angles: [f32; 3],
    angle_type: TriangleAngleType,
    side_type: TriangleSideType,
    center: IVec2,
}

impl Triangle {
    pub fn new(point1: impl Point2d, point2: impl Point2d, point3: impl Point2d) -> Self {
        let points = [point1.as_ivec2(), point2.as_ivec2(), point3.as_ivec2()];
        let angles = [
            points[0].angle_to(points[1]),
            points[1].angle_to(points[2]),
            points[2].angle_to(points[0]),
        ];

        let angle_type = if angles.iter().any(|a| a.abs() == 90.0) {
            TriangleAngleType::Right
        } else if angles[0].abs() == angles[1].abs() && angles[1].abs() == angles[2].abs() {
            TriangleAngleType::Equiangular
        } else if angles.iter().all(|a| a.abs() < 90.0) {
            TriangleAngleType::Acute
        } else {
            //else if angles.iter().any(|a| a.abs() > 90.0) {
            TriangleAngleType::Obtuse
        };

        let side_type = if DistanceAlg::PythagorasSquared.distance2d(points[0], points[1]) ==
            DistanceAlg::PythagorasSquared.distance2d(points[1], points[2]) &&
            DistanceAlg::PythagorasSquared.distance2d(points[2], points[0]) ==
                DistanceAlg::PythagorasSquared.distance2d(points[1], points[2])
        {
            TriangleSideType::Equilateral
        } else if DistanceAlg::PythagorasSquared.distance2d(points[0], points[1]) ==
            DistanceAlg::PythagorasSquared.distance2d(points[1], points[2]) ||
            DistanceAlg::PythagorasSquared.distance2d(points[2], points[0]) ==
                DistanceAlg::PythagorasSquared.distance2d(points[1], points[2]) ||
            DistanceAlg::PythagorasSquared.distance2d(points[2], points[0]) ==
                DistanceAlg::PythagorasSquared.distance2d(points[0], points[1])
        {
            TriangleSideType::Isosceles
        } else {
            TriangleSideType::Scalene
        };

        let mut triangle = Self { points, angles, angle_type, side_type, center: IVec2::new(0, 0) };

        triangle.center = IVec2::new(triangle.left(), triangle.bottom())
            .mid_point(IVec2::new(triangle.right(), triangle.top()));
        triangle
    }
}

impl Triangle {
    #[inline]
    pub const fn angles(&self) -> [f32; 3] { self.angles }
    #[inline]
    pub const fn angle_type(&self) -> &TriangleAngleType { &self.angle_type }
    #[inline]
    pub const fn side_type(&self) -> &TriangleSideType { &self.side_type }
}

impl Shape for Triangle {
    fn from_points(points: Vec<impl Point2d>) -> Self
    where Self: Sized {
        Self::new(points[0], points[1], points[2])
    }

    fn contains(&self, point: impl Point2d) -> bool {
        let p1 =
            IVec2::new(self.points[1].x - self.points[0].x, self.points[1].y - self.points[0].y);
        let p2 =
            IVec2::new(self.points[2].x - self.points[0].x, self.points[2].y - self.points[0].y);
        let q = IVec2::new(point.x() - self.points[0].x, point.y() - self.points[0].y);

        let s = q.cross_product(p2) as f32 / p1.cross_product(p2) as f32;
        let t = p1.cross_product(q) as f32 / p1.cross_product(p2) as f32;

        s >= 0.0 && t >= 0.0 && (s + t) <= 1.0
    }

    fn points(&self) -> Vec<IVec2> { self.points.to_vec() }

    #[inline]
    fn center(&self) -> IVec2 { self.center }
}

impl Triangle {
    pub fn as_rect(&self) -> Rectangle {
        Rectangle::new((self.left(), self.top()), (self.right(), self.bottom()))
    }
}
