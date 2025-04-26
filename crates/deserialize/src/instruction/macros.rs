#[macro_export]
macro_rules! deserialize_instruction_args_with_discriminator {
    ($data:expr, $discriminator:ty) => {{
        let expected_discriminator = <$discriminator>::new();

        $crate::instruction::deserialize::deserialize_instruction_discriminator(
            $data,
            expected_discriminator,
        )
    }};
    ($data:expr, $discriminator:ty, $args_type:ty) => {{
        let expected_discriminator = <$discriminator>::new();

        $crate::instruction::deserialize::deserialize_instruction_args_with_discriminator::<
            $args_type,
        >($data, expected_discriminator)
    }};
}

#[cfg(test)]
mod tests {
    use borsh::{BorshDeserialize, BorshSerialize};

    #[derive(BorshSerialize, BorshDeserialize)]
    struct TestArgs {
        pub a: u8,
        pub b: u16,
    }

    struct TestDiscriminator {}

    impl TestDiscriminator {
        fn new() -> [u8; 8] { [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08] }
    }

    #[test]
    fn test_deserialize_instruction_args_with_discriminator() {
        let args = TestArgs { a: 1, b: 2 };
        let discriminator = TestDiscriminator::new();
        let mut args_data = vec![];
        args.serialize(&mut args_data).unwrap();
        let mut data = discriminator.to_vec();
        data.extend(args_data);
        let (discriminator, args) =
            deserialize_instruction_args_with_discriminator!(&data, TestDiscriminator, TestArgs)
                .unwrap();

        assert_eq!(discriminator, discriminator);
        assert_eq!(args.a, 1);
        assert_eq!(args.b, 2);
    }

    #[test]
    fn test_deserialize_instruction_args_with_discriminator_without_args() {
        let discriminator = TestDiscriminator::new();
        let data = discriminator.to_vec();
        let discriminator =
            deserialize_instruction_args_with_discriminator!(&data, TestDiscriminator).unwrap();

        assert_eq!(discriminator, discriminator);
    }
}
