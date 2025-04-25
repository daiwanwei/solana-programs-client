use sha2::{Digest, Sha256};

pub trait Discriminator {
    const DISCRIMINATOR: [u8; 8];
    fn discriminator() -> [u8; 8] { Self::DISCRIMINATOR }
}

pub fn generate_discriminator(namespace: &str, name: &str) -> [u8; 8] {
    let mut hasher = Sha256::new();
    hasher.update(format!("{namespace}:{name}").as_bytes());
    let discriminator = hasher.finalize();
    discriminator[..8].try_into().unwrap()
}
