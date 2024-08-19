#[cfg(test)]
mod test {

    #[test]
    pub fn test() {
        type A = const_struct::parse_value!((f32, u32), (0.6, 4));
        no_std_compat::println!("{:?}", <A as ::const_struct::PrimitiveTraits>::__DATA);

        // type B = const_struct::parse_value!(Option<Option<f32>>, Some(Some(0.6)));

        type B = const_struct::parse_value!(
            (Option<Option<f32>>, f32, Option<bool>),
            (None, 0.6, Some(true))
        );

        assert_eq!(
            <B as ::const_struct::PrimitiveTraits>::__DATA,
            (None, 0.6, Some(true))
        );

        no_std_compat::println!("{:?}", <B as ::const_struct::PrimitiveTraits>::__DATA);
    }
}
