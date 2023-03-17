//! Splits a string
#![warn(missing_debug_implementations, rust_2018_idioms)]

pub trait Delimiter {
    /// Returns Option<(start, end)>
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

#[derive(Debug)]
pub struct StrSplit<'a, D> {
    remainder: Option<&'a str>,
    delimiter: D,
}

impl<'a, D> StrSplit<'a, D> {
    pub fn new(haystack: &'a str, delimiter: D) -> Self {
        StrSplit {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

impl<'a, D> Iterator for StrSplit<'a, D>
where
    D: Delimiter,
{
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(remainder) = self.remainder {
            if let Some((start, end)) = self.delimiter.find_next(remainder) {
                let item = &remainder[..start];
                self.remainder = Some(&remainder[end..]);
                Some(item)
            } else {
                self.remainder.take()
            }
        } else {
            None
        }
    }
}

impl Delimiter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self).map(|pos| (pos, pos + self.len()))
    }
}

impl Delimiter for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.char_indices()
            .find(|(_, c)| c == self)
            .map(|(pos, _)| (pos, pos + self.len_utf8()))
    }
}

pub fn until_char(s: &str, c: char) -> &str {
    StrSplit::new(s, c)
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
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();

    assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
}

#[test]
fn it_works_with_ending_delim() {
    let haystack = "a b c d e ";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();

    assert_eq!(letters, vec!["a", "b", "c", "d", "e", ""]);
}
