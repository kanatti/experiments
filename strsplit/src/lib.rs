//! Splits a string
#![warn(missing_debug_implementations, rust_2018_idioms)]

#[derive(Debug)]
pub struct StrSplit<'h, 'd> {
    remainder: Option<&'h str>,
    delimiter: &'d str,
}

impl<'h, 'd> StrSplit<'h, 'd> {
    pub fn new(haystack: &'h str, delimiter: &'d str) -> Self {
        StrSplit {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

impl<'h> Iterator for StrSplit<'h, '_> {
    type Item = &'h str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(remainder) = self.remainder {
            if let Some(pos) = remainder.find(self.delimiter) {
                let item = &remainder[..pos];
                self.remainder = Some(&remainder[(pos + self.delimiter.len())..]);
                Some(item)
            } else {
                self.remainder.take()
            }
        } else {
            None
        }
    }
}

pub fn until_char(s: &str, c: char) -> &str {
    StrSplit::new(s, &format!("{}", c))
        .next()
        .expect("StrSplit always returns atleast one result")
}

#[test]
fn until_char_works() {
    assert_eq!(until_char("Hello World", 'o'), "Hell");
}

#[test]
#[allow(unused_assignments)]
fn it_works() {
    let haystack = "a b c d e";
    let mut letters = vec![];

    {
        let delimiter = " ".to_owned(); // Delimiter need not live as long as haystack or letters
        letters = StrSplit::new(haystack, &delimiter).collect();
    }

    assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
}

#[test]
fn it_works_with_ending_delim() {
    let haystack = "a b c d e ";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();

    assert_eq!(letters, vec!["a", "b", "c", "d", "e", ""]);
}
