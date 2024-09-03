#![allow(dead_code)]

use crate::{
    prelude::{get_chars, get_chunk, get_utf8_codes, Result},
    tools::{DebugBuffer, DebugInfo},
};
use calamine::{open_workbook_auto, Data, Range, Reader};
use std::{
    fmt::Display,
    fs::File,
    io::{BufWriter, Stdout, Write},
    ops::DerefMut,
    path::PathBuf,
};

struct SheetProcessor<'a, RS>
where
    RS: std::io::Read + std::io::Seek,
{
    xl: &'a mut calamine::Sheets<RS>,
    all_sheets: Vec<String>,
    column_offset: usize,
    row_offset: usize,
    debug_buffer: DebugBuffer<Stdout>,
}

impl<'a, RS> SheetProcessor<'a, RS>
where
    RS: std::io::Read + std::io::Seek,
{
    fn new(xl: &'a mut calamine::Sheets<RS>, debug_buffer: BufWriter<Stdout>) -> Self
    where
        RS: std::io::Read + std::io::Seek,
    {
        let all_sheets = xl.sheet_names().to_vec();
        Self {
            xl,
            all_sheets,
            column_offset: 0,
            row_offset: 0,
            debug_buffer: DebugBuffer::new(debug_buffer),
        }
    }

    fn process_sheets<W: Write>(&mut self, buffer: &mut W) -> Result<()> {
        let mut all_sheets_iter = self.all_sheets.iter().peekable();
        let mut sheet_counter = 0;
        while let Some(sheet) = all_sheets_iter.next() {
            // All sheets and start loop
            let range = self.xl.worksheet_range(sheet)?; //  Get the size/shape of current sheet
            let current_sheet_row_count = range.height(); // Get the height of the current sheet
            dbg!(&current_sheet_row_count);

            if let Some(next_sheet) = all_sheets_iter.peek() {
                // Get the next sheet as peek
                let next_range = self.xl.worksheet_range(next_sheet)?; // Get the stats of the next sheet
                let next_sheet_row_count = next_range.height(); // Get the height of the next sheet
                dbg!(&next_sheet_row_count);

                self.row_offset =
                    self.handle_row_offset(next_sheet_row_count, current_sheet_row_count);

                #[rustfmt::skip]
                println!("after we set it via handle_row_offset::: {}", self.row_offset);
            }

            self.write_sheet(buffer, &range)?;
            self.column_offset += range.width();
            dbg!(self.column_offset);
            dbg!(sheet_counter);
            sheet_counter += 1;
        }
        Ok(())
    }

    fn handle_row_offset(
        &self,
        next_sheet_row_count: usize,
        current_sheet_row_count: usize,
    ) -> usize {
        next_sheet_row_count.saturating_add(current_sheet_row_count)
    }

    fn write_sheet<W: Write>(&self, buffer: &mut W, range: &Range<Data>) -> Result<()> {
        let w = range.get_size().1 - 1;
        let h = range.get_size().0;

        dbg!(w);
        dbg!(h);

        // TODO: This will be what I'm after --> h as height of curr sheet size. --- NOT THE ENTIRE
        // BUFFER

        for rows in range.rows() {
            // Gets us the ending column of the previous sheet

            if self.column_offset != 1 || self.column_offset != 0 {
                write!(buffer, "{}", ",".repeat(self.column_offset))?;
            }

            for (mut i, col) in rows.iter().enumerate() {
                if i != 0 && i > 0 && i > self.column_offset {
                    i += self.column_offset;
                }

                match col {
                    Data::Empty => Ok(()), //write!(buffer, ","),
                    Data::String(ref s) | Data::DateTimeIso(ref s) | Data::DurationIso(ref s) => {
                        let s = clean_text_cell(s);
                        write!(buffer, "\"{}\",", s)
                    }
                    Data::Float(ref f) => write!(buffer, "{},", f),
                    Data::DateTime(ref d) => {
                        let dt = d.as_datetime().unwrap();
                        write!(buffer, "{},", dt)
                    }
                    Data::Int(ref xl_int) => write!(buffer, "{},", xl_int),
                    Data::Error(ref e) => write!(buffer, "{},", e),
                    Data::Bool(ref b) => write!(buffer, "{},", b),
                }?;
                if i != w {
                    write!(buffer, "")?;
                }
            }
            write!(buffer, "\r\n")?; // can we do the check at the end of the line
        }
        Ok(())
    }
}

impl<'a, W> DebugInfo for SheetProcessor<'a, W>
where
    W: Write + Display + std::io::Seek + std::io::Read,
{
    fn append_debug_info<K: Write + Display>(
        &mut self,
        items: Vec<K>,
        sheet: Option<&str>,
        range: Option<&Range<Data>>,
        // sheet: Option<&str>,
        // column_offset: usize,
    ) -> Result<()> {
        #[rustfmt::skip]
        self.debug_buffer.append_debug_info(items, sheet, range)?;
        Ok(())
    }
}

pub fn convert(file: PathBuf) -> Result<()> {
    let dest = file.with_extension("csv");
    let mut buffer_file = BufWriter::new(File::create(&dest)?);
    let mut xl = open_workbook_auto(file)
        .inspect_err(|e| {
            eprintln!("Error opening workbook:: {}", e);
        })
        .expect("Error opening workbook, perhaps it's open, or corrupted");

    let stdout = std::io::stdout();
    let buffer = BufWriter::new(stdout);

    let mut processor = SheetProcessor::new(&mut xl, buffer);

    processor.process_sheets(&mut buffer_file)?;
    Ok(())
}

fn write_range<W: Write>(
    dest: &mut W,
    range: &Range<Data>,
    column_offset: usize,
    // row_under_offset: usize,
    // starting_row: usize,
) -> std::io::Result<()> {
    let w = range.get_size().1 - 1;
    for rows in range.rows() {
        // Gets us the ending column of the previous sheet
        if column_offset != 1 || column_offset != 0 {
            write!(dest, "{}", ",".repeat(column_offset))?;
        }

        for (mut i, col) in rows.iter().enumerate() {
            if i != 0 && i > 0 && i > column_offset {
                i += column_offset;
            }

            match col {
                Data::Empty => write!(dest, ","),
                Data::String(ref s) | Data::DateTimeIso(ref s) | Data::DurationIso(ref s) => {
                    let s = clean_text_cell(s);
                    write!(dest, "\"{}\",", s)
                }
                Data::Float(ref f) => write!(dest, "{},", f),
                Data::DateTime(ref d) => {
                    let dt = d.as_datetime().unwrap();
                    write!(dest, "{},", dt)
                }
                Data::Int(ref xl_int) => write!(dest, "{},", xl_int),
                Data::Error(ref e) => write!(dest, "{},", e),
                Data::Bool(ref b) => write!(dest, "{},", b),
            }?;
            if i != w {
                write!(dest, "")?;
            }
        }
        write!(dest, "\r\n")?; // can we do the check at the end of the line
    }

    Ok(())
}

// This has terrible performance
fn clean_text_cell(s: &str) -> &str {
    while let Some(pos) = s.find(get_chunk()) {
        s.to_string()
            .replace_range(pos..pos + get_chunk().len(), "");
    }

    for c in get_utf8_codes().iter().map(|c| *c as char) {
        let new_cell_str = match c {
            '\r' | '\n' => s.replace(c, ""),
            _ => "".to_string(),
        };
        // Maybe no required
        if get_chars().contains(&c) {
            new_cell_str.replace(c, "").deref_mut();
        }
        //END Maybe no required
        new_cell_str.replace(c, "").deref_mut();
    }
    s
}

#[allow(dead_code)]
mod dev {
    use super::{Data, Write};

    fn inner_visibility(r: &[Data], i: usize, c: &Data) {
        let mut buffer = get_std_lock();
        let (current_cell, next_cell) = calc_cell(r, i);
        buffer.flush().unwrap();
        print_inner(buffer, i, c, current_cell, next_cell);
    }

    fn calc_cell(r: &[Data], i: usize) -> (Option<&Data>, Option<&Data>) {
        let current_cell = r.get(i);
        let next_cell = r.get(i + 1);
        (current_cell, next_cell)
    }

    fn get_std_lock() -> std::io::StdoutLock<'static> {
        let stdout = std::io::stdout();
        stdout.lock()
    }

    #[rustfmt::skip]
pub fn print_inner(
    mut buffer: std::io::StdoutLock,
    idx: usize,
    col: &Data,
    current_cell: Option<&Data>,
    next_cell: Option<&Data>,
) {
    buffer .write_all(format!("Current i: {}\n", idx).as_bytes()) .unwrap();
    buffer .write_all(format!("Current column: {}\n", col).as_bytes()) .unwrap();
    buffer .write_all(format!("Current cell [row]: {:?}\n", current_cell).as_bytes()) .unwrap();
    buffer .write_all(format!("Next cell [row]: {:?}\n", next_cell).as_bytes()) .unwrap();
    buffer.flush().unwrap();
    }
}
