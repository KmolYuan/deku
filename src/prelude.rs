/*! Crate prelude

[What is a prelude?](std::prelude)
*/
pub use crate::error::{DekuError, NeedSize};
pub use crate::{
    deku_derive, reader::Reader, writer::Writer, DekuContainerRead, DekuContainerWrite,
    DekuEnumExt, DekuRead, DekuReadSizeHint, DekuReadSized, DekuReader, DekuUpdate, DekuWrite,
    DekuWriteSizeHint, DekuWriteSized, DekuWriter,
};
