use super::PrimitiveTraits;

pub trait OptionTy<T> {
    const __DATA: Option<T>;
    const VALUE: Option<T> = <Self as OptionTy<T>>::__DATA;
}

#[derive(Debug, Copy, Clone)]
pub struct OptionImpl<T: PrimitiveTraits> {
    __phantom: core::marker::PhantomData<T>,
}

impl<U: PrimitiveTraits<DATATYPE = Option<T>>, T> OptionTy<T> for U
{
    const __DATA: Option<T> = <U as PrimitiveTraits>::__DATA;
}

impl<T: PrimitiveTraits> PrimitiveTraits for OptionImpl<T> {
    type DATATYPE = Option<T::DATATYPE>;
    const __DATA: Self::DATATYPE = Some(<T as PrimitiveTraits>::__DATA);
}

pub struct NoneImpl;

impl<T> OptionTy<T> for NoneImpl {
    const __DATA: Option<T> = None;
}

#[macro_export]
macro_rules! Some {
    ($value:ty) => {
        $crate::primitive::OptionImpl<$value>
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
    use crate::primitive::{F32Ty, U32Ty, _F32 as F32, _U32 as U32};
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
