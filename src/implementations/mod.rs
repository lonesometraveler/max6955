//! HAL specific implementations
#[cfg_attr(feature = "eh-02", path = "eh-02.rs")]
#[cfg_attr(feature = "eh-1", path = "eh-1.rs")]
#[cfg_attr(feature = "eh-a", path = "eh-a.rs")]
mod eh;
pub use eh::*;
