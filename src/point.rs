use std::default::Default;
use std::ops::{Add, Sub, Mul, Div, Index, IndexMut};

macro_rules! define_point {
($type_name: ident, $dimension: expr) => {
        pub struct $type_name<T> {
            coords: [T; $dimension]
        }

        impl<T: Default> Default for $type_name<T> {
            fn default() -> Self {
                $type_name {
                    coords: <[T; $dimension]>::default()
                }
            }
        }
    }
}

macro_rules! get_coord {
    ($coord_name: ident, $coord_index: expr) => {
        pub fn $coord_name(&self) -> &T {
            &self[$coord_index]
        }
    }
}

macro_rules! get_all_coords {
    (1) => {
        get_coord!(x, 0);
    };
    (2) => {
        get_coord!(y, 1);
        get_all_coords!(1);
    };
    (3) => {
        get_coord!(z, 2);
        get_all_coords!(2);
    };
    (4) => {
        get_coord!(w, 3);
        get_all_coords!(3);
    };
}

macro_rules! impl_point_accessors {
    ($type_name: ident, $dimension: tt) => {
        impl<T> $type_name<T> {
            get_all_coords!($dimension);
        }
    }
}

macro_rules! impl_new_point {
    ($type_name: ident, $($coord_name: tt),+) => {
        impl<T> $type_name<T> {
            pub fn new($($coord_name: T),+) -> Self {
                $type_name {
                    coords: [$($coord_name),+]
                }
            }
        }
    }
}

macro_rules! impl_point {
    ($type_name: ident, $dimension: tt) => {
        define_point!($type_name, $dimension);
        impl_point_accessors!($type_name, $dimension);

        impl<T> Index<usize> for $type_name<T> {
            type Output = T;
            fn index(&self, index: usize) -> &T {
                &self.coords[index]
            }
        }

        impl<T> IndexMut<usize> for $type_name<T> {
            fn index_mut(&mut self, index: usize) -> &mut T {
                &mut self.coords[index]
            }
        }


        impl<T: Add<Output = T> + Default + Copy + Clone> Add<$type_name<T>> for $type_name<T> {
            type Output = $type_name<T>;
            fn add(self, rhs: $type_name<T>) -> Self::Output {
                let mut return_point = $type_name::<T>::default();
                for i in 0..$dimension {
                    return_point[i] = self.coords[i] + rhs.coords[i];
                }
                return_point
            }
        }

        impl<T: Sub<Output = T> + Default + Copy + Clone> Sub<$type_name<T>> for $type_name<T> {
            type Output = $type_name<T>;
            fn sub(self, rhs: $type_name<T>) -> Self::Output {
                let mut return_point = $type_name::<T>::default();
                for i in 0..$dimension {
                    return_point[i] = self.coords[i] - rhs.coords[i];
                }
                return_point
            }
        }
    }
}

impl_point!(Point2, 2);
impl_point!(Point3, 3);
impl_point!(Point4, 4);

impl_new_point!(Point2, x, y);
impl_new_point!(Point3, x, y, z);
impl_new_point!(Point4, x, y, z, w);

pub type Point2i = Point2<i32>;
pub type Point2l = Point2<i64>;
pub type Point2u = Point2<usize>;
pub type Point2f = Point2<f32>;
pub type Point2d = Point2<f64>;

pub type Point3i = Point3<i32>;
pub type Point3l = Point3<i64>;
pub type Point3u = Point3<usize>;
pub type Point3f = Point3<f32>;
pub type Point3d = Point3<f64>;



