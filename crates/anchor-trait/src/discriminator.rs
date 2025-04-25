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

pub fn generate_account_discriminator(name: &str) -> [u8; 8] {
    generate_discriminator("account", name)
}

pub fn generate_event_discriminator(name: &str) -> [u8; 8] { generate_discriminator("event", name) }
