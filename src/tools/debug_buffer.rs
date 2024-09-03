use crate::{prelude::Result, tools::DebugInfo};
use calamine::{Data, Range};
use std::{
    fmt::Display,
    io::{BufWriter, Stdout, Write},
};

pub struct DebugBuffer<S>
where
    S: Write,
{
    pub buffer: BufWriter<S>,
}

impl<S> DebugBuffer<S>
where
    S: Write,
{
    fn get_mut(&mut self) -> &mut BufWriter<S> {
        &mut self.buffer
    }
}

impl DebugBuffer<Stdout> {
    pub fn new(buffer: BufWriter<Stdout>) -> Self {
        Self { buffer }
    }

    // fn append_debug_info<W: Write + Display>(
    //     &mut self,
    //     items: Vec<W>,
    //     sheet: Option<&str>,
    //     // sheet: Option<&str>,
    //     range: Option<&Range<Data>>,
    //     // column_offset: usize,
    // ) -> Result<()> {
    //     #[rustfmt::skip]
    //     self.buffer.flush()?;
    //     if let Some(sh) = sheet {
    //         #[rustfmt::skip]
    //          self.buffer.write_all(format!("Sheet: {}\n", sh).as_bytes())?;
    //     }
    //     if let Some(range) = range {
    //         #[rustfmt::skip]
    //         self.buffer.write_all(format!("Height: {}\n", range.height()).as_bytes())?;
    //         #[rustfmt::skip]
    //         self.buffer.write_all(format!("Width: {}\n", range.width()).as_bytes())?;
    //     }
    //
    //     for item in items.iter() {
    //         #[rustfmt::skip]
    //         self.buffer.write_all(format!("Column Offset: {}\n", item).as_bytes())?;
    //     }
    //     Ok(())
    // }
}

impl DebugInfo for DebugBuffer<Stdout> {
    fn append_debug_info<W: Write + Display>(
        &mut self,
        items: Vec<W>,
        sheet: Option<&str>,
        range: Option<&Range<Data>>,
        // sheet: Option<&str>,
        // column_offset: usize,
    ) -> Result<()> {
        #[rustfmt::skip]
        self.buffer.flush()?;
        if let Some(sh) = sheet {
            #[rustfmt::skip]
             self.buffer.write_all(format!("Sheet: {}\n", sh).as_bytes())?;
        }
        if let Some(range) = range {
            #[rustfmt::skip]
            self.buffer.write_all(format!("Height: {}\n", range.height()).as_bytes())?;
            #[rustfmt::skip]
            self.buffer.write_all(format!("Width: {}\n", range.width()).as_bytes())?;
        }
        for item in items.iter() {
            #[rustfmt::skip]
            self.buffer.write_all(format!("Column Offset: {}\n", item).as_bytes())?;
        }
        Ok(())
    }
}
