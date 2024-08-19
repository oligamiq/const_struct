#[cfg(test)]
use const_struct::{primitive::OptionImpl, F32};

#[test]
pub fn test() {
    type A = const_struct::parse_value!((f32, u32), (0.6, 4));
    no_std_compat::println!("{:?}", <A as ::const_struct::PrimitiveTraits>::__DATA);

    // type B = const_struct::parse_value!(Option<f32>, Some(0.6));
    // type B = const_struct::parse_value!(Option<f32>, None);
    const S: Option<f32> = Some(0.6);
    type B = OptionImpl<
        F32!({
            match S {
                None => core::mem::zeroed(),
                Some(v) => v,
            }
        }),
        {
            match S {
                None => 0,
                Some(_) => 1,
            }
        },
    >;
    no_std_compat::println!("{:?}", <B as ::const_struct::PrimitiveTraits>::__DATA);
}
