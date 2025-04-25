pub mod constants;
pub mod libraries;
pub mod math;
pub mod state;
pub mod types;
pub mod utils;


#[allow(unused_imports)]
#[allow(unused_qualifications)]
#[rustfmt::skip]
pub mod generated;

pub use generated::programs::{AMM_V3_ID as ID, AMM_V3_ID};
#[cfg(feature = "fetch")]
pub use generated::shared::*;
#[cfg(feature = "fetch")]
pub(crate) use generated::*;

// solana_program::declare_id!("CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK");
