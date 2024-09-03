use crate::prelude::{get_chars, get_chunk, get_utf8_codes, Result};
use calamine::{open_workbook_auto, Data, Range, Reader};

pub fn convert(file: PathBuf) -> Result<()> {
    let dest = file.with_extension("csv");
    let mut buffer_file = BufWriter::new(File::create(&dest)?);

    let all_sheets = xl.sheet_names().to_vec();

    for sheet in all_sheets {
        let range = xl.worksheet_range(&sheet)?;
        write_range(&mut buffer_file, &range)?;
    }

    Ok(())
}

fn write_range<W: Write>(dest: &mut W, range: &Range<Data>) -> std::io::Result<()> {
    let height = range.get_size().0 - 1;

    // We write each row -> col | col | col ->> until None ->> next row
    for row in range.rows() {
        // println!("Row: {:?}", r);
        for (i, col) in row.iter().enumerate() {
            // inner_visibility(r, i, c); ///////// PERF:

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
                Data::Int(ref i) => write!(dest, "{},", i),
                Data::Error(ref e) => write!(dest, "{:?},", e),
                Data::Bool(ref b) => write!(dest, "{},", b),
            }?;
            if i != height {
                write!(dest, "")?;
            }
        }
        write!(dest, "\r\n")?;
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
