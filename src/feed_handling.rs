#![allow(dead_code)]
use std::fmt::Display;

#[derive(Debug)]
enum LineFeedTypes {
    CR,
    LF,
    CRLF,
    ExcelDefault,
}

impl LineFeedTypes {
    fn empty(&self) -> String {
        String::new()
    }
}

#[derive(Debug)]
struct LineFeed {
    feed_type: Option<LineFeedTypes>,
    all_types: Vec<LineFeedTypes>,
}

impl Default for LineFeed {
    fn default() -> Self {
        LineFeed {
            feed_type: None,
            all_types: vec![
                LineFeedTypes::CR,
                LineFeedTypes::LF,
                LineFeedTypes::CRLF,
                LineFeedTypes::ExcelDefault,
            ],
        }
    }
}

impl Display for LineFeed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.feed_type {
            Some(LineFeedTypes::CR) => write!(f, "\r"),
            Some(LineFeedTypes::LF) => write!(f, "\n"),
            Some(LineFeedTypes::CRLF) => write!(f, "\r\n"),
            Some(LineFeedTypes::ExcelDefault) => {
                let excel_lf = std::char::from_u32(10).unwrap();
                write!(f, "{}", excel_lf)
            }
            _ => unimplemented!(),
        }
    }
}

impl LineFeed {
    fn new(feed_type: Option<LineFeedTypes>) -> Self {
        if let Some(feed_type) = feed_type {
            LineFeed {
                feed_type: Some(feed_type),
                all_types: vec![
                    LineFeedTypes::CR,
                    LineFeedTypes::LF,
                    LineFeedTypes::CRLF,
                    LineFeedTypes::ExcelDefault,
                ],
            }
        } else {
            LineFeed::default()
        }
    }

    fn remove_feed(&self, text: &str) -> String {
        let feed = self.to_string();
        text.replace(&feed, "")
    }
}

/////////////

// From in the crate::prelude module
// This seems to be best way of doing this, but HashMap is not the best choice

// Testing to make the call to convert::clean_text_cell(s)
// call something that can be optimized a bit more and easier to read
#[allow(dead_code)]
pub struct LfChunk {
    pub char_map: HashMap<&'static str, String>,
    pub chunk: String,
}

impl LfChunk {
    #[allow(dead_code)]
    pub fn new() -> Self {
        let mut map = HashMap::new();
        for c in get_chars() {
            dbg!(c);
            map.insert("", c.to_string());
        }

        for uc in get_utf8_codes().iter().map(|c| *c as char) {
            dbg!(uc);
            map.insert("", uc.to_string());
        }

        LfChunk {
            char_map: map,
            chunk: get_chunk().to_string(),
        }
    }

    // pub fn clean<'a>(&self, s: Box<&'a str>) -> Box<&'a str> {
    //     // let s = self.clear_chunk_from(s);
    //     // let s = self.clear_chars_from(s);
    //     // s
    // }
}
