pub mod spot;
pub mod um;

mod prelude {
    pub use rust_decimal::prelude::Zero;
    pub use rust_decimal::Decimal;
    pub use serde::{Deserialize, Serialize};
    pub use serde_repr::{Deserialize_repr, Serialize_repr};

    pub use crate::error::*;
    pub use crate::proto::*;
    pub use crate::Atom;
    pub use crate::TimeWindow;
}
