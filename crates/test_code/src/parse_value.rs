#[cfg(test)]
mod test {
    #[test]
    pub fn test() {
        type A = const_struct::parse_value!((f32, u32), (0.6, 4));
        no_std_compat::println!("{:?}", <A as ::const_struct::PrimitiveTraits>::__DATA);

        // type B = const_struct::parse_value!(Option<Option<f32>>, Some(Some(0.6)));

        type B = const_struct::parse_value!(Option<Option<f32>>, None);

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
        no_std_compat::println!("{:?}", <B as ::const_struct::PrimitiveTraits>::__DATA);
    }
}
