use crate::prelude::{get_chars, get_chunk, get_utf8_codes, Result};
use calamine::{open_workbook_auto, Data, Range, Reader};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

pub fn convert(file: PathBuf) -> Result<()> {
    let dest = file.with_extension("csv");
    let mut dest = BufWriter::new(File::create(&dest)?);
    println!("\n\nConverting this File: {:?}", file);
    let mut xl = open_workbook_auto(file).inspect_err(|e| {
        eprintln!("Error opening workbook:: {}", e);
    })?;

    let all_sheets = xl.sheet_names().to_vec();

    for sheet in all_sheets {
        dbg!(&sheet.to_string());
        let range = xl.worksheet_range(&sheet)?;
        write_range(&mut dest, &range)?;
    }

    Ok(())
}

fn write_range<W: Write>(dest: &mut W, range: &Range<Data>) -> std::io::Result<()> {
    let height = range.get_size().0 - 1;

    // We write each row -> col | col | col ->> until None ->> next row
    for r in range.rows() {
        // println!("Row: {:?}", r);
        for (i, c) in r.iter().enumerate() {
            // inner_visibility(r, i, c); ///////// PERF:

            match *c {
                Data::Empty => Ok(()),
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
fn clean_text_cell(s: &str) -> String {
    let mut s = s.to_string();

    while let Some(pos) = s.find(get_chunk()) {
        s.replace_range(pos..pos + get_chunk().len(), "");
    }

    for c in get_utf8_codes().iter().map(|c| *c as char) {
        match c {
            '\r' | '\n' => {
                s = s.replace(c, "");
            }
            _ => (),
        }
        // Maybe no required
        if get_chars().contains(&c) {
            s = s.replace(c, "");
        }
        //END Maybe no required
        s = s.replace(c, "");
    }
    s.escape_default().to_string()
}

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

// let lfchunk = LfChunk::new();
// while let Some(pos) = s.find(lfchunk.chunk.as_str()) {
//     s.replace_range(pos..pos + lfchunk.chunk.len(), "");
// }
// let mut outer_s = s.clone();
// let mut hmap = std::collections::HashMap::new();
// for c in remove_chars {
//     hmap.insert("", c.to_string());
// }
// for uc in remove_utf8_codes.iter().map(|c| *c as char) {
//     hmap.insert("", uc.to_string());
// }
// hmap.insert("", remove_chunk.to_string());
//
// for (_k, v) in hmap.iter() {
//     s = s.replace("", v.as_str());
// }
//
//

// Data::DateTime(ref d) => {
//     let datetime: chrono::DateTime<chrono::FixedOffset> =
//         match W(d.to_string().as_str()).try_into() {
//             Ok(dt) => dt,
//             Err(e) => {
//                 eprintln!("Error converting to DateTime: {:?}", e);
//                 continue;
//             }
//         };
//     write!(dest, "{},", datetime)
// }
//
//
