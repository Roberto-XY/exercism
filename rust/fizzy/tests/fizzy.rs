use std::ops::Rem;

/// A Matcher is a single rule of fizzbuzz: given a function on T, should
/// a word be substituted in? If yes, which word?
pub struct Matcher<'a, T> {
    subs: String,
    predicate: Box<dyn Fn(T) -> bool + 'a>,
}

impl<'a, T> Matcher<'a, T> {
    pub fn new<F, S>(matcher: F, subs: S) -> Matcher<'a, T>
    where
        F: Fn(T) -> bool + 'a,
        S: ToString,
    {
        Matcher {
            subs: subs.to_string(),
            predicate: Box::new(matcher),
        }
    }

    fn substitute(&self, val: T) -> Option<&str> {
        if (self.predicate)(val) {
            Some(&self.subs)
        } else {
            None
        }
    }
}

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
    matchers: Vec<Matcher<'a, T>>,
}
impl<'a, T> Fizzy<'a, T>
where
    T: ToString + Copy + 'a,
{
    pub fn new() -> Self {
        Fizzy::default()
    }
    // feel free to change the signature to `mut self` if you like
    #[must_use]
    pub fn add_matcher(mut self, matcher: Matcher<'a, T>) -> Self {
        self.matchers.push(matcher);
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
                .flat_map(|matcher| matcher.substitute(t))
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
pub fn fizz_buzz<'a, T>() -> Fizzy<'a, T>
where
    T: Rem<T, Output = T> + PartialEq + Copy + ToString + 'a,
    u8: Into<T>,
{
    Fizzy::default()
        .add_matcher(Matcher::new(|x| x % 3.into() == 0.into(), "fizz"))
        .add_matcher(Matcher::new(|x| x % 5.into() == 0.into(), "buzz"))
}

pub struct Fizzy2<'a, T> {
    matchers: Vec<Box<dyn Fn(T) -> Option<String> + 'a>>,
}

impl<'a, T> Fizzy2<'a, T>
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
                .flat_map(|matcher| matcher(t))
                .collect();

            if subs.is_empty() {
                t.to_string()
            } else {
                subs
            }
        })
    }
}

impl<'a, T> Default for Fizzy2<'a, T> {
    fn default() -> Self {
        Fizzy2 { matchers: vec![] }
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz2<'a, T: 'a>() -> Fizzy2<'a, T>
where
    T: Rem<T, Output = T> + PartialEq + ToString + Copy,
    u8: Into<T>,
{
    Fizzy2::default()
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
}
