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

macro_rules! dimension_to_coord_pairs {
    (1) => {
        x, 0
    };
    (2) => {
        x, 0, y, 1
    };
    (3) => {
        x, 0, y, 1, z, 2
    };
    (4) => {
        x, 0, y, 1, z, 2, w, 3
    };
}

macro_rules! coords_to_tuple_type {
    ($i: expr) => { (&T) };
    ($i: expr, $($r: expr),+) => { (&T, coords_to_tuple_type!($($r),+)) };
}


macro_rules! impl_point_accessors_recursive {
    ($coord_name: ident: $($coord_index: expr) +) => {
        pub fn $coord_name(&self) -> coords_to_tuple_type!($($coord_index),+) {
            ($(&self[$coord_index]),+)
        }
    };

    ($coord_name: ident: $($coord_index: expr) +, $($extra_coord_name: ident: $($extra_coord_index: expr) +),+) => {
        impl_point_accessors_recursive!($coord_name: $($coord_index) +);
        impl_point_accessors_recursive!($($extra_coord_name: $($extra_coord_index) +), +);
    }
}

macro_rules! impl_point_accessors {
    ($type_name: ident, $($coord_name: ident: $($coord_index: expr) +),+) => {
        impl<T> $type_name<T> {
            impl_point_accessors_recursive!($($coord_name: $($coord_index) +), +);
        }
    }
}


macro_rules! impl_point {
    ($type_name: ident, $dimension: expr) => {
        define_point!($type_name, $dimension);

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

impl_point_accessors!(Point2, x: 0, y: 1, xy: 0 1, yx: 1 0);
impl_point_accessors!(Point3, x: 0, y: 1, z: 2, xy: 0 1, xz: 0 2, yz: 1 2, yx: 1 0, zx: 2 0, zy: 2 1, xyz: 0 1 2, yzx: 1 2 0, zxy: 2 0 1, xzy: 1 2 0, zyx: 2 1 0, yxz: 1 0 2);
impl_point_accessors!(Point4, x: 0, y: 1, z: 2, w: 3);

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



