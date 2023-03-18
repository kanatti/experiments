pub struct IntegersTill {
    till: u32,
    current: u32,
}

impl IntegersTill {
    pub fn new(till: u32) -> Self {
        IntegersTill { till, current: 0 }
    }
}

impl Iterator for IntegersTill {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.till {
            None
        } else {
            self.current += 1;
            Some(self.current - 1)
        }
    }
}

// Flatten

pub fn flatten<I>(iter: I) -> Flatten<I::IntoIter>
where
    I: IntoIterator,
    I::Item: IntoIterator,
{
    Flatten::new(iter.into_iter())
}

pub struct Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    outer: O,
    left: Option<<O::Item as IntoIterator>::IntoIter>,
    right: Option<<O::Item as IntoIterator>::IntoIter>,
}

impl<O> Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    fn new(iter: O) -> Self {
        Flatten {
            outer: iter,
            left: None,
            right: None,
        }
    }
}

impl<O> Iterator for Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    type Item = <O::Item as IntoIterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut inner) = self.left {
            if let Some(i) = inner.next() {
                return Some(i);
            };
        };

        if let Some(new_inner) = self.outer.next() {
            self.left = Some(new_inner.into_iter());
            return self.next();
        } else {
            self.right.as_mut()?.next()
        }
    }
}

impl<O> DoubleEndedIterator for Flatten<O>
where
    O: Iterator + DoubleEndedIterator,
    O::Item: IntoIterator,
    <O::Item as IntoIterator>::IntoIter: DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if let Some(ref mut inner) = self.right {
            if let Some(i) = inner.next_back() {
                return Some(i);
            };
        };

        if let Some(new_inner) = self.outer.next_back() {
            self.right = Some(new_inner.into_iter());
            return self.next_back();
        } else {
            self.left.as_mut()?.next()
        }
    }
}

pub trait FlattenExt: Iterator {
    fn new_flatten(self) -> Flatten<Self>
    where
        Self: Sized,
        Self::Item: IntoIterator;
}

impl<T: Iterator> FlattenExt for T {
    fn new_flatten(self) -> Flatten<Self>
    where
        Self: Sized,
        Self::Item: IntoIterator,
    {
       crate::iterators::flatten(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integers_till_works() {
        let first_10: Vec<_> = IntegersTill::new(10).collect();

        assert_eq!(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], first_10);

        for i in IntegersTill::new(10) {
            println!("{}", i);
        }
    }

    #[test]
    fn empty() {
        assert_eq!(flatten(std::iter::empty::<Vec<()>>()).count(), 0)
    }

    #[test]
    fn one() {
        assert_eq!(flatten(std::iter::once(vec!["a"])).count(), 1)
    }

    #[test]
    fn two() {
        assert_eq!(flatten(vec![vec!["a", "b"]]).count(), 2)
    }

    #[test]
    fn two_wide() {
        assert_eq!(flatten(vec![vec!["a"], vec!["b"]]).count(), 2)
    }

    #[test]
    fn reverse() {
        assert_eq!(
            flatten(vec![vec!["a", "b"]]).rev().collect::<Vec<_>>(),
            vec!["b", "a"]
        );
    }

    #[test]
    fn reverse_wide() {
        assert_eq!(
            flatten(vec![vec!["a", "b"], vec!["c", "d"]])
                .rev()
                .collect::<Vec<_>>(),
            vec!["d", "c", "b", "a"]
        );
    }

    #[test]
    fn mixed_double_ended() {
        let mut flat = flatten(vec![vec!["a", "b"], vec!["c", "d"]]);

        assert_eq!(flat.next(), Some("a"));
        assert_eq!(flat.next_back(), Some("d"));
        assert_eq!(flat.next(), Some("b"));
        assert_eq!(flat.next_back(), Some("c"));
        assert_eq!(flat.next(), None);
        assert_eq!(flat.next_back(), None);
    }

    #[test]
    fn mixed_double_ended_longer() {
        let mut flat = flatten(vec![
            vec!["a", "b", "c", "d", "e"],
            vec!["f", "g"],
            vec!["h", "i", "j"],
        ]);

        assert_eq!(flat.next(), Some("a"));
        assert_eq!(flat.next(), Some("b"));
        assert_eq!(flat.next_back(), Some("j"));
        assert_eq!(flat.next(), Some("c"));
        assert_eq!(flat.next_back(), Some("i"));
        assert_eq!(flat.next_back(), Some("h"));
        assert_eq!(flat.next_back(), Some("g"));
        assert_eq!(flat.next(), Some("d"));
        assert_eq!(flat.next(), Some("e"));
        assert_eq!(flat.next(), Some("f"));
        assert_eq!(flat.next_back(), None);
        assert_eq!(flat.next(), None);
    }

    #[test]
    fn deep() {
        assert_eq!(
            flatten(flatten(vec![vec![vec![1, 2], vec![3, 4]]])).count(),
            4
        );
    }

    #[test]
    fn ext() {
        assert_eq!(
            vec![vec![1, 2], vec![3, 4]].iter().new_flatten().count(),
            4
        );
    }
}
