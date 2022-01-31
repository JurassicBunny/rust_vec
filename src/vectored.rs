use crate::vector::Vector3D;
use num::Float;

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

#[macro_export]
macro_rules! vectored {
    ($expression:ident) => {
        #[derive(Debug, Clone, Copy)]
        pub struct $expression<T: num::Float>(crate::vector::Vector3D<T>);

        impl<T: num::Float> $expression<T> {
            #[allow(dead_code)]
            pub fn new(x: T, y: T, z: T) -> Self {
                let vector = crate::vector::Vector3D::new(x, y, z);
                Self(vector)
            }
        }

        impl<T: num::Float> Vectored<T> for $expression<T> {
            fn as_vec(&self) -> crate::vector::Vector3D<T> {
                self.0
            }
            fn set_vec(&mut self, vec: crate::vector::Vector3D<T>) {
                self.0 = vec;
            }
        }

        impl<T, U> std::ops::Add<U> for $expression<T>
        where
            T: num::Float,
            U: crate::vectored::Vectored<T>,
        {
            type Output = $expression<T>;
            fn add(mut self, rhs: U) -> Self::Output {
                let result = self.as_vec() + rhs.as_vec();
                self.set_vec(result);
                self
            }
        }

        impl<T, U> std::ops::Sub<U> for $expression<T>
        where
            T: num::Float,
            U: crate::vectored::Vectored<T>,
        {
            type Output = $expression<T>;
            fn sub(mut self, rhs: U) -> Self::Output {
                let result = self.as_vec() - rhs.as_vec();
                self.set_vec(result);
                self
            }
        }
    };
}

vectored!(Acceleration);
vectored!(Force);
vectored!(Momentum);
vectored!(Position);
vectored!(Velocity);
