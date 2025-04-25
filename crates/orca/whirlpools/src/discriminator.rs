use anchor_trait::Discriminator;

use crate::generated::accounts::{
    FeeTier, Position, PositionBundle, TickArray, Whirlpool, WhirlpoolsConfig,
};

impl Discriminator for Whirlpool {
    const DISCRIMINATOR: [u8; 8] = [63, 149, 209, 12, 225, 128, 99, 9];
}

impl Discriminator for WhirlpoolsConfig {
    const DISCRIMINATOR: [u8; 8] = [157, 20, 49, 224, 217, 87, 193, 254];
}

impl Discriminator for Position {
    const DISCRIMINATOR: [u8; 8] = [170, 188, 143, 228, 122, 64, 247, 208];
}

impl Discriminator for PositionBundle {
    const DISCRIMINATOR: [u8; 8] = [129, 169, 175, 65, 185, 95, 32, 100];
}

impl Discriminator for TickArray {
    const DISCRIMINATOR: [u8; 8] = [69, 97, 189, 190, 110, 7, 66, 187];
}

impl Discriminator for FeeTier {
    const DISCRIMINATOR: [u8; 8] = [56, 75, 159, 76, 142, 68, 190, 105];
}

#[cfg(test)]
mod tests {
    use anchor_trait::{generate_account_discriminator, Discriminator};

    use super::*;

    #[test]
    fn test_whirlpool_discriminator() {
        assert_eq!(Whirlpool::DISCRIMINATOR, generate_account_discriminator("Whirlpool"));
    }

    #[test]
    fn test_whirlpools_config_discriminator() {
        assert_eq!(
            WhirlpoolsConfig::DISCRIMINATOR,
            generate_account_discriminator("WhirlpoolsConfig")
        );
    }

    #[test]
    fn test_position_discriminator() {
        assert_eq!(Position::DISCRIMINATOR, generate_account_discriminator("Position"));
    }

    #[test]
    fn test_position_bundle_discriminator() {
        assert_eq!(PositionBundle::DISCRIMINATOR, generate_account_discriminator("PositionBundle"));
    }

    #[test]
    fn test_tick_array_discriminator() {
        assert_eq!(TickArray::DISCRIMINATOR, generate_account_discriminator("TickArray"));
    }

    #[test]
    fn test_fee_tier_discriminator() {
        assert_eq!(FeeTier::DISCRIMINATOR, generate_account_discriminator("FeeTier"));
    }
}
