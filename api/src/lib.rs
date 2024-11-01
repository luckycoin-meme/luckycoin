pub mod consts;
pub mod error;
pub mod instruction;
pub mod sdk;
pub mod state;
pub mod event;
pub mod loaders;
pub mod cpi;

pub mod prelude {
    pub use crate::consts::*;
    pub use crate::error::*;
    pub use crate::instruction::*;
    pub use crate::sdk::*;
    pub use crate::state::*;
}

use steel::*;

// TODO Set program id
declare_id!("5A5bi3qQJ1c1821Brqnt8no6KjGtX8X811URhC1UjD9n");
