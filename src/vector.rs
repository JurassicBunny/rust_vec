use num::Float;
use std::fmt::Display;
use std::ops::{Add, Mul, Sub};

pub trait Vectored<T: Float> {
    fn as_vec(&self) -> Vector3D<T>;

    fn set_vec(&mut self, vec: Vector3D<T>);

    fn sqr_norm(&self) -> T {
        self.as_vec().x.powi(2) + self.as_vec().y.powi(2) + self.as_vec().z.powi(2)
    }

    fn norm(&self) -> T {
        self.sqr_norm().sqrt()
    }

    fn normalize(mut self) -> Self
    where
        Self: Sized,
    {
        let result = self.as_vec() * self.norm().recip();
        self.set_vec(result);
        self
    }
}

impl<T, U> Add<U> for Velocity<T>
where
    T: Float,
    U: Vectored<T>,
{
    type Output = Velocity<T>;
    fn add(mut self, rhs: U) -> Self::Output {
        let result = self.as_vec() + rhs.as_vec();
        self.set_vec(result);
        self
    }
}

impl<T, U> Add<U> for Acceleration<T>
where
    T: Float,
    U: Vectored<T>,
{
    type Output = Acceleration<T>;
    fn add(mut self, rhs: U) -> Self::Output {
        let result = self.as_vec() + rhs.as_vec();
        self.set_vec(result);
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Velocity<T: Float>(Vector3D<T>);

#[derive(Debug, Clone, Copy)]
pub struct Acceleration<T: Float>(Vector3D<T>);

impl<T: Float> Velocity<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        let vector = Vector3D::new(x, y, z);
        Self(vector)
    }
}

impl<T: Float> Acceleration<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        let vector = Vector3D::new(x, y, z);
        Self(vector)
    }
}

impl<T: Float> Vectored<T> for Velocity<T> {
    fn as_vec(&self) -> Vector3D<T> {
        self.0
    }
    fn set_vec(&mut self, vec: Vector3D<T>) {
        self.0 = vec;
    }
}

impl<T: Float> Vectored<T> for Acceleration<T> {
    fn as_vec(&self) -> Vector3D<T> {
        self.0
    }
    fn set_vec(&mut self, vec: Vector3D<T>) {
        self.0 = vec;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vector3D<T: Float> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Float> Vector3D<T> {
    pub fn new(x: T, y: T, z: T) -> Vector3D<T> {
        Vector3D { x, y, z }
    }
}

impl<T: Float> Mul<T> for Vector3D<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vector3D<f64>> for f64 {
    type Output = Vector3D<f64>;
    fn mul(self, rhs: Vector3D<f64>) -> Self::Output {
        Vector3D {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        }
    }
}

impl Mul<Vector3D<f32>> for f32 {
    type Output = Vector3D<f32>;
    fn mul(self, rhs: Vector3D<f32>) -> Self::Output {
        Vector3D {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        }
    }
}

impl<T: Float> Add for Vector3D<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: Float> Sub for Vector3D<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T> Display for Vector3D<T>
where
    T: Float + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {}\ny: {}\nz: {}", self.x, self.y, self.z)
    }
}
