use std::ops;

#[derive(Clone, Copy, Debug)]
pub struct Vec3(f64, f64, f64);

impl Vec3 {
    fn x(&self) -> f64 {
        self.0
    }

    fn y(&self) -> f64 {
        self.1
    }

    fn z(&self) -> f64 {
        self.2
    }

    fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    fn length_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    #[inline]
    fn dot(u: Vec3, v: Vec3) -> f64 {
        u.0 * v.0 + u.1 * v.1 + u.2 * v.2
    }

    #[inline]
    fn cross(u: Vec3, v: Vec3) -> Vec3 {
        Vec3(
            u.1 * v.2 - u.2 * v.1,
            u.2 * v.0 - u.0 * v.2,
            u.0 * v.1 - u.1 * v.0,
        )
    }

    #[inline]
    fn unit_vector(v: Vec3) -> Vec3 {
        v / v.length()
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl ops::AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl ops::MulAssign<f64> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl ops::DivAssign<f64> for Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(self * rhs.0, self * rhs.1, self * rhs.2)
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

type Point3 = Vec3;
type Color = Vec3;