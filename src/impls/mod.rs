mod bool;
mod ipaddr;
mod nonzero;
mod option;
mod primitive;
mod slice;
mod tuple;
mod unit;
mod vec;

#[cfg(feature = "alloc")]
mod arc;

#[cfg(feature = "alloc")]
mod cow;

#[cfg(feature = "alloc")]
mod cstring;

#[cfg(feature = "std")]
mod hashmap;

#[cfg(feature = "std")]
mod hashset;

#[cfg(feature = "alloc")]
mod boxed;

macro_rules! ImplDekuPointerSized {
    ($wrapper:ident<$($a:lifetime,)? $T:ident $(: $bound:tt)?>) => {
        impl<$T: crate::DekuReadSized $(+ $bound)?> crate::DekuReadSized for $wrapper<$($a,)? $T> {
            const BIT_SIZE: usize = <T as crate::DekuReadSized>::BIT_SIZE;
        }
        impl<T: crate::DekuReadSizeHint $(+ $bound)?> crate::DekuReadSizeHint for $wrapper<$($a,)? $T> {
            const LOWER_BIT_SIZE: usize = <T as crate::DekuReadSizeHint>::LOWER_BIT_SIZE;
            const UPPER_BIT_SIZE: usize = <T as crate::DekuReadSizeHint>::UPPER_BIT_SIZE;
        }
        impl<T: crate::DekuWriteSized $(+ $bound)?> crate::DekuWriteSized for $wrapper<$($a,)? $T> {
            const BIT_SIZE: usize = <T as crate::DekuWriteSized>::BIT_SIZE;
        }
        impl<T: crate::DekuWriteSizeHint $(+ $bound)?> crate::DekuWriteSizeHint for $wrapper<$($a,)? $T> {
            #[inline]
            fn bit_size(&self) -> usize {
                (**self).bit_size()
            }
        }
    };
}

macro_rules! ImplDekuSized {
    ($($typ:ty),+) => {$(
        impl crate::DekuReadSized for $typ {
            const BIT_SIZE: usize = core::mem::size_of::<Self>() * 8;
        }
        impl crate::DekuReadSizeHint for $typ {
            const LOWER_BIT_SIZE: usize = <Self as crate::DekuReadSized>::BIT_SIZE;
            const UPPER_BIT_SIZE: usize = <Self as crate::DekuReadSized>::BIT_SIZE;
        }
        impl crate::DekuWriteSized for $typ {
            const BIT_SIZE: usize = <Self as crate::DekuReadSized>::BIT_SIZE;
        }
        impl crate::DekuWriteSizeHint for $typ {
            #[inline]
            fn bit_size(&self) -> usize {
                <Self as crate::DekuWriteSized>::BIT_SIZE
            }
        }
    )+};
}

use ImplDekuPointerSized;
use ImplDekuSized;
