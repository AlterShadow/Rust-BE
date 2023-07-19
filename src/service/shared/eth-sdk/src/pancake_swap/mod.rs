pub mod execute;
pub mod pair_paths;
pub mod parse;

pub use execute::*;
pub use parse::*;

pub const SMART_ROUTER_ABI_JSON: &str = include_str!("smart_router_v3.json");
