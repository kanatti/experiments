#[macro_export]
macro_rules! avec {
    () => {
        Vec::new()
    };
    ($($element:expr),+ $(,)?) => {{
        let mut vs = Vec::with_capacity($crate::avec![@COUNT; $($element),+]);
        $(vs.push($element);)+
        vs
    }};
    ($element:expr; $count:expr) => {{
        let count = $count;
        let mut vs = Vec::with_capacity(count);
        let x = $element;
        vs.extend(std::iter::repeat(x).take(count));
        vs
    }};

    (@COUNT; $($element:expr),+) => { 
        <[()]>::len(&[$($crate::avec![@SUBST; $element]),+])
    };

    (@SUBST; $element:expr) => { () }
}

pub trait MaxValue {
    fn max_value() -> Self;
}

macro_rules! max_impl {
    ($t:ty) => {
        impl MaxValue for $t {
            fn max_value() -> Self {
                <$t>::MAX
            }
        }
    };
}

max_impl!(i8);
max_impl!(u8);
max_impl!(i32);
max_impl!(u32);

#[test]
fn empty_vec() {
    let x: Vec<u32> = avec![];
    assert!(x.is_empty());
}

#[test]
fn single() {
    let x: Vec<u32> = avec![42];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 1);
    assert_eq!(x[0], 42);
}

#[test]
fn double() {
    let x: Vec<u32> = avec![42, 43];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 42);
    assert_eq!(x[1], 43);
}

#[test]
fn trailing() {
    let x: Vec<u32> = avec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28,
    ];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 28);
}

#[test]
fn count() {
    let x: Vec<u32> = avec![42; 2];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 42);
    assert_eq!(x[1], 42);
}
