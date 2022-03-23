use std::ops::Rem;

// /// A Matcher is a single rule of fizzbuzz: given a function on T, should
// /// a word be substituted in? If yes, which word?
// pub struct Matcher<F, T> {
//     f: F,
// }

// impl<F, T> Matcher<F, T>
// where
//     F: Fn(T) -> Option<String>,
// {
//     pub fn new(matcher: F) -> Matcher<F, T> {
//         Matcher { f: matcher }
//     }
// }

/// A Fizzy is a set of matchers, which may be applied to an iterator.
///
/// Strictly speaking, it's usually more idiomatic to use `iter.map()` than to
/// consume an iterator with an `apply` method. Given a Fizzy instance, it's
/// pretty straightforward to construct a closure which applies it to all
/// elements of the iterator. However, we're using the `apply` pattern
/// here because it's a simpler interface for students to implement.
///
/// Also, it's a good excuse to try out using impl trait.
pub struct Fizzy<'a, T> {
    matchers: Vec<Box<dyn Fn(T) -> Option<String> + 'a>>,
}

impl<'a, T> Fizzy<'a, T>
where
    T: ToString + Copy + 'a,
{
    // feel free to change the signature to `mut self` if you like
    #[must_use]
    pub fn add_matcher<F>(mut self, matcher: F) -> Self
    where
        F: Fn(T) -> Option<String> + 'a,
    {
        self.matchers.push(Box::new(matcher));
        self
    }

    /// map this fizzy onto every element of an iterator, returning a new iterator
    pub fn apply<I>(self, iter: I) -> impl Iterator<Item = String> + 'a
    where
        I: IntoIterator<Item = T> + 'a,
    {
        iter.into_iter().map(move |t| {
            let subs: String = self
                .matchers
                .iter()
                .flat_map(|matcher| matcher(t.clone()))
                .collect();

            if subs.is_empty() {
                t.to_string()
            } else {
                subs
            }
        })
    }
}

impl<'a, T> Default for Fizzy<'a, T> {
    fn default() -> Self {
        Fizzy { matchers: vec![] }
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<'a, T: 'a>() -> Fizzy<'a, T>
where
    T: Rem<T, Output = T> + PartialEq + ToString + Copy,
    u8: Into<T>,
{
    Fizzy::default()
        .add_matcher(|x| {
            if x % 5.into() == 0.into() {
                Some("Buzz".to_string())
            } else {
                None
            }
        })
        .add_matcher(|x| {
            if x % 3.into() == 0.into() {
                Some("Fizz".to_string())
            } else {
                None
            }
        })
        .add_matcher(|x| {
            if x % 7.into() == 0.into() {
                Some("Bam".to_string())
            } else {
                None
            }
        })
}

pub fn main() {
    let expect = vec![
        "1", "2", "Fizz", "4", "Buzz", "Fizz", "Bam", "8", "Fizz", "Buzz", "11", "Fizz", "13",
        "Bam", "BuzzFizz", "16",
    ];
    let fizzer: Fizzy<i32> = fizz_buzz();
    let got = fizzer.apply(1..=16).into_iter().collect::<Vec<_>>();
    let got = dbg!(got);
    assert_eq!(expect, got);
}
