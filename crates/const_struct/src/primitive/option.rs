// use super::{EnumQueuePlaneDataType, EnumQueuePlaneEnd, EnumQueuePlaneHead, PrimitiveTraits};
use super::PrimitiveTraits;

pub trait OptionTy<T> {
    const __DATA: Option<T>;
    const VALUE: Option<T> = <Self as OptionTy<T>>::__DATA;
}

impl<U: PrimitiveTraits<DATATYPE = Option<T>>, T> OptionTy<T> for U {
    const __DATA: Option<T> = <U as PrimitiveTraits>::__DATA;
}

#[derive(Debug, Copy, Clone)]
pub struct SomeImpl<T: PrimitiveTraits> {
    __phantom: core::marker::PhantomData<T>,
}

impl<T: PrimitiveTraits> PrimitiveTraits for SomeImpl<T> {
    type DATATYPE = Option<T::DATATYPE>;
    const __DATA: Self::DATATYPE = Some(<T as PrimitiveTraits>::__DATA);
}

pub struct NoneImpl;

impl<T> OptionTy<T> for NoneImpl {
    const __DATA: Option<T> = None;
}

// impl<T, U: PrimitiveTraits<DATATYPE = T>> PrimitiveTraits
//     for EnumQueuePlaneHead<Option<T>, EnumQueuePlaneDataType<U, EnumQueuePlaneEnd>, 0>
// {
//     type DATATYPE = Option<T>;
//     const __DATA: Self::DATATYPE = None;
// }

// impl<T, U: PrimitiveTraits<DATATYPE = T>> PrimitiveTraits
//     for EnumQueuePlaneHead<Option<T>, EnumQueuePlaneDataType<U, EnumQueuePlaneEnd>, 1>
// {
//     type DATATYPE = Option<T>;
//     const __DATA: Self::DATATYPE = Some(<U as PrimitiveTraits>::__DATA);
// }

#[macro_export]
macro_rules! Some {
    ($value:ty) => {
        $crate::primitive::SomeImpl<$value>
    };
}

#[macro_export]
macro_rules! None {
    () => {
        $crate::primitive::NoneImpl
    };
}

#[cfg(test)]
mod tests {
    use crate::primitive::{F32Ty, U32Ty, F32, U32};
    use core::mem;

    use super::OptionTy;

    pub const fn tester_inner<T: F32Ty>() -> f32 {
        T::VALUE
    }

    pub const fn tester_inner_u32<T: U32Ty>() -> u32 {
        T::VALUE
    }

    pub const fn tester_inner_option<T: OptionTy<f32>>() -> Option<f32> {
        T::VALUE
    }

    #[test]
    pub fn call_tester() {
        let s: Some!(F32!(-25.333)) = unsafe { mem::zeroed() };
        assert_eq!(core::mem::size_of_val(&s), 0);
        assert_eq!(tester_inner::<F32!(-0.5)>(), -0.5);
        assert_eq!(
            tester_inner_option::<Some!(F32!(-25.333))>(),
            Some(-25.333f32)
        );
        assert_eq!(tester_inner_option::<None!()>(), None);
        assert_eq!(tester_inner_u32::<U32!(0)>(), 0);
    }
}

// type B = const_struct::parse_value!(Option<f32>, None);
// EnumQueuePlaneHead<Option<T>, EnumQueuePlaneDataType<U, EnumQueuePlaneEnd>, 0>
// type B = EnumQueuePlaneHead<
//     Option<f32>,
//     EnumQueuePlaneDataType<
//         F32!({
//             match S {
//                 None => core::mem::zeroed(),
//                 Some(v0) => v0,
//             }
//         }),
//         EnumQueuePlaneEnd,
//     >,
//     {
//         match S {
//             None => 0,
//             Some(_) => 1,
//         }
//     },
// >;
