// mod feed_handling;

mod debug_buffer;
pub use self::debug_buffer::DebugBuffer;

use crate::prelude::Result;

use std::io::Write;
pub trait DebugInfo {
    fn append_debug_info<W: Write + std::fmt::Display>(
        &mut self,
        items: Vec<W>,
        sheet: Option<&str>,
        range: Option<&calamine::Range<calamine::Data>>,
    ) -> Result<()>;
}
