/// A trait for implementing `DEFAULT`, which can be used within `const fn`.
/// Since instances of types that might implement `core::mem::Drop` cannot be declared as `const`,
/// this effectively requires `Copy`.
pub trait ConstDefault {
    const DEFAULT: Self;
}

macro_rules! impl_const_default {
    ($($t:ty = $v:expr;)*) => {
        $(
            impl ConstDefault for $t {
                const DEFAULT: Self = $v;
            }
        )*
    };
}

impl_const_default! {
    u8 = 0;
    u16 = 0;
    u32 = 0;
    u64 = 0;
    u128 = 0;
    usize = 0;
    i8 = 0;
    i16 = 0;
    i32 = 0;
    i64 = 0;
    i128 = 0;
    isize = 0;
    f32 = 0.0;
    f64 = 0.0;
    bool = false;
    char = '\0';
}

impl ConstDefault for &str {
    const DEFAULT: Self = "";
}

impl ConstDefault for () {
    const DEFAULT: Self = ();
}

impl<T> ConstDefault for core::option::Option<T> {
    const DEFAULT: Self = core::option::Option::None;
}

impl<T: ConstDefault + Copy, const N: usize> ConstDefault for [T; N] {
    const DEFAULT: Self = [T::DEFAULT; N];
}

impl ConstDefault for core::time::Duration {
    const DEFAULT: Self = core::time::Duration::from_nanos(0);
}

impl<T: ?Sized> ConstDefault for core::marker::PhantomData<T> {
    const DEFAULT: Self = core::marker::PhantomData;
}

macro_rules! impl_const_default_tuple {
    ($($t:ident),*) => {
        impl<$($t: ConstDefault),*> ConstDefault for ($($t,)*) {
            const DEFAULT: Self = ($($t::DEFAULT,)*);
        }
    };
}

impl_const_default_tuple!(A);
impl_const_default_tuple!(A, B);
impl_const_default_tuple!(A, B, C);
impl_const_default_tuple!(A, B, C, D);
impl_const_default_tuple!(A, B, C, D, E);
impl_const_default_tuple!(A, B, C, D, E, F);
impl_const_default_tuple!(A, B, C, D, E, F, G);
impl_const_default_tuple!(A, B, C, D, E, F, G, H);
