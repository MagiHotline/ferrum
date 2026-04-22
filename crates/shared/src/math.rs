
// ; Definition of data structures in the style of GLM
// ; Including vec2, vec3, vec4, ivec2, ivec3, ivec4 and quat
use std::ops::{Add, Mul, Div, Sub};

/// A 1x4 vector
#[derive(Debug, Clone, Copy)]
struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}

impl Vec4 {
    fn new() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0, w: 0.0 }
    }

    fn create(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    fn splat(v: f32) -> Self {
        Self {x: v, y: v, z: v, w: v}
    }
}

/// A 1x3 vector
#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec3 {
    pub fn new() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0}
    }

    pub fn create(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z }
    }

    pub fn splat(v: f32) -> Self {
        Self {x: v, y: v, z: v }
    }
}

/// A 1x2 vector
#[derive(Debug, Clone, Copy)]
struct Vec2 {
    pub x: f32,
    pub y: f32
}

impl Vec2 {
    fn new() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    fn create(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y }
    }

    fn splat(v: f32) -> Self {
        Self {x: v, y: v }
    }
}

/// a 2x2 Matrix
#[derive(Clone, Copy, Debug)]
struct Mat2 {
    pub a: Vec2,
    pub b: Vec2
}

impl Mat2 {
    fn new() -> Self {
        Self { a: Vec2::new(), b: Vec2::new() }
    }

    fn base(v: f32) -> Self {
        Self { a: Vec2 { x: v, y: 0.0 }, b: Vec2 { x: 0.0, y: v }}
    }

    fn transpose(self) -> Self {
        Self
        {
            a: Vec2 { x: self.a.x , y: self.b.x },
            b: Vec2 { x: self.a.y, y: self.b.y},
        }
    }
}

pub struct Mat3 {
    pub a: Vec3,
    pub b: Vec3,
    pub c: Vec3
}

impl Mat3 {
    /// Create an empty 3x3 matrix
    pub fn new() -> Self {
        Self { a: Vec3::new(), b: Vec3::new(), c: Vec3::new() }
    }

    /// Creates a 3x3 matrix with precise values
    pub fn create(
        a_1 : f32, a_2: f32, a_3: f32,
        b_1 : f32, b_2: f32, b_3: f32,
        c_1:  f32, c_2: f32, c_3: f32
    ) -> Self {
        Self
        {
            a: Vec3 { x: a_1, y: a_2, z: a_3 },
            b: Vec3 { x: b_1, y: b_2, z: b_3 },
            c: Vec3 { x: c_1, y: c_2, z: c_3 }
        }
    }

    /// Create a basis 3x3 matrix
    pub fn base(v: f32) -> Self {
        Self {
            a: Vec3 { x: v, y: 0.0, z: 0.0 },
            b: Vec3 { x: 0.0, y: v, z: 0.0 },
            c: Vec3 { x: 0.0, y: 0.0, z: v}
        }
    }

    pub fn transpose(self) -> Self {
        Self
        {
            a: Vec3 { x: self.a.x , y: self.b.x, z: self.c.x },
            b: Vec3 { x: self.a.y, y: self.b.y, z: self.c.y },
            c: Vec3 { x: self.a.z, y: self.b.z, z: self.c.z }
        }
    }
}

struct Mat4 {
    pub a: Vec4,
    pub b: Vec4,
    pub c: Vec4,
    pub d: Vec4
}

impl Mat4 {
    fn new() -> Self {
        Self { a: Vec4::new(), b: Vec4::new(), c: Vec4::new(), d: Vec4::new() }
    }

    fn base(v: f32) -> Self {
        Self {
            a: Vec4 { x: v, y: 0.0, z: 0.0, w: 0.0 },
            b: Vec4 { x: 0.0, y: v, z: 0.0, w: 0.0 },
            c: Vec4 { x: 0.0, y: 0.0, z: v, w: 0.0 },
            d: Vec4 { x: 0.0, y: 0.0, z: 0.0, w: v }
        }
    }

    fn transpose(self) -> Self {
        Self
        {
            a: Vec4 { x: self.a.x , y: self.b.x, z: self.c.x, w: self.d.z },
            b: Vec4 { x: self.a.y, y: self.b.y, z: self.c.y, w: self.d.y },
            c: Vec4 { x: self.a.z, y: self.b.z, z: self.c.y, w: self.d.z},
            d: Vec4 { x: self.a.w, y: self.b.w, z: self.c.w, w: self.d.w }
        }
    }
}

// OPERATIONS

trait MulS {
   type Output;
   fn muls(self, other: f32) -> Self;
}

trait DivS {
    type Output;
    fn divs(self, other: f32) -> Self;
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl MulS for Vec2 {
    type Output = Vec2;

    fn muls(self, other: f32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other
        }
    }
}

impl Div for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y
        }
    }
}

impl DivS for Vec2 {
    type Output = Vec2;

    fn divs(self, other: f32) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other
        }
    }
}


impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z
        }
    }
}

impl MulS for Vec3 {
    type Output = Vec3;

    fn muls(self, other: f32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other
        }
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z
        }
    }
}

impl DivS for Vec3 {
    type Output = Vec2;

    fn divs(self, other: f32) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other
        }
    }
}


impl Add for Vec4 {
    type Output = Vec4;

    fn add(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w
        }
    }
}

impl Sub for Vec4 {
    type Output = Vec4;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w
        }
    }
}

impl Mul for Vec4 {
    type Output = Vec4;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
            w: self.w * rhs.w
        }
    }
}

impl MulS for Vec4 {
    type Output = Vec4;

    fn muls(self, other: f32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other
        }
    }
}

impl Div for Vec4 {
    type Output = Vec4;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
            w: self.w / rhs.w
        }
    }
}

impl DivS for Vec4 {
    type Output = Vec2;

    fn divs(self, other: f32) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.w / other
        }
    }
}

impl Add for Mat2 {
    type Output = Mat2;

    fn add(self, rhs: Self) -> Self::Output {
        Self
        {
            a: self.a + rhs.a,
            b: self.b + rhs.b
        }
    }
}

impl Sub for Mat2 {
    type Output = Mat2;

    fn sub(self, rhs: Self) -> Self::Output {
        Self
        {
            a: self.a - rhs.a,
            b: self.b - rhs.b
        }
    }
}
