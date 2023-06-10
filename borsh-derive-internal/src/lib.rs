#![recursion_limit = "128"]
// TODO: re-enable this lint when we bump msrv to 1.58
#![allow(clippy::uninlined_format_args)]

mod attribute_helpers;
mod enum_de;
mod enum_discriminant_map;
mod enum_ser;
mod struct_de;
mod struct_ser;
mod tokio;
mod union_de;
mod union_ser;

pub use enum_de::enum_de;
pub use enum_ser::enum_ser;
pub use struct_de::struct_de;
pub use struct_ser::struct_ser;
pub use union_de::union_de;
pub use union_ser::union_ser;

pub use tokio::enum_de as tokio_enum_de;
pub use tokio::enum_ser as tokio_enum_ser;
pub use tokio::struct_de as tokio_struct_de;
pub use tokio::struct_ser as tokio_struct_ser;
pub use tokio::union_de as tokio_union_de;
pub use tokio::union_ser as tokio_union_ser;
