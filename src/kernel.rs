use std::ops::FnOnce;

pub trait Kernel<T> {
    fn kernel_size(self, new_size: T) -> Self;
    fn evaluate(&self, x: T) -> T;
}
pub struct EpanechnikovKernel {
    kernel_size: f64,
    inv_sq_kernel_size: f64
}

impl EpanechnikovKernel {
    pub fn new() -> Self {
        EpanechnikovKernel {
            kernel_size: 1f64,
            inv_sq_kernel_size: 1f64,
        }
    }
}

impl Kernel<f64> for EpanechnikovKernel {
    fn kernel_size(mut self, new_size: f64) -> Self {
        self.kernel_size = new_size;
        self.inv_sq_kernel_size = 1f64 / (new_size * new_size);
        self
    }

    fn evaluate(&self, x: f64) -> f64 {
        if x.abs() > self.kernel_size {
            0f64
        }
        else {
            0.75f64 * (1f64 - x * x * self.inv_sq_kernel_size)
        }
    }
}


