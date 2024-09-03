// Note: Calling this function will flush the current contents of the debug buffer
// fn append_debug_info(self) -> Result<()> {
//     let mut debug_buf = self.debug_buffer;
//     Ok(())
// }

// fn add_column_offset(&mut self, range: &Range<Data>) {
//     self.column_offset += range.width();
// }

////////////// working for the most part

// let mut row_leveling_offset = 0;
// let mut current_sheet_row_count = 0;
// let mut next_sheet_row_start = 0;

// To compute the current sheet's row count  ->>>>
//
// we need:
// current sheet's number of rows
// the next sheet's number of rows
//
// A placeholder value to hold the difference
//
// the difference after:::  place_holder = next_sheet_row_c - current_sheet_row_c
// EG:
// First sheet is 5 rows
// Second sheet is 20 rows;
// 20 - 5 = 15
//
// Now we have the computed value of 15 in our placceholder
//
//
// we can actually SET the offset for the next sheet to start 15 rows backwards from the
// current iterator
//
//
// EG ---> We have a third sheet coming up ->>
// Sheet 3 has say 100 rows
// now we can peek ->
// third_sheet_row_start = 100 - 15
// third_sheet_row_start = 85
//

// let all_sheets = xl.sheet_names().to_vec();
// let mut column_offset = 0;

// for (idx, sheet) in all_sheets.iter().enumerate().peekable() {
// let range = xl.worksheet_range(sheet)?;
// write_range(&mut buffer_file, &range, column_offset, row_leveling_offset)?;
// column_offset += range.width(); // Moved the 2nd sheet over by the number of cols in the
// first sheet
// }

// Ok(())

////////////// working for the most part

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
