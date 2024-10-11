#[cfg(feature = "libm")]
mod libm {
    use libm::Libm;

    pub trait Math {
        fn abs(self) -> Self;
    }
    
    macro_rules! impl_math {
        ($($type:ty),+) => {
            $(impl Math for $type {
                fn abs(self) -> $type {
                    Libm::<$type>::fabs(self)
                }
            })+
        };
    }

    impl_math!(f64, f32);
}

#[cfg(feature = "vexide-core")]
mod vexide_core {
    pub use vexide_core::float::Float as Math;
}

#[cfg(feature = "libm")]
pub use libm::Math;
#[cfg(feature = "vexide-core")]
pub use vexide_core::Math;

#[cfg(all(feature = "libm", feature = "vexide-core"))]
compile_error!("Cannot enable both libm and vexide-core features");
