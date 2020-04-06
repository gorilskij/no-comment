use std::collections::VecDeque;

/// Buffer type used in the `WithoutComments` iterator, `Deref`s to `VecDeque<char>`.
/// The capacity of the inner `VecDeque<char>` is constant, it represents the maximum length of
/// buffer needed to match any open or close pattern for the current language.
#[derive(Deref, Debug)]
#[repr(transparent)]
struct Buf(VecDeque<char>);

impl Buf {
    fn new(max_len: usize) -> Self {
        Self(VecDeque::with_capacity(max_len))
    }

    fn is_full(&self) -> bool {
        self.len() == self.capacity()
    }

    /// Fill up inner `VecDeque<char>` to capacity from provided iterator. This is the only way
    /// to add elements to the buffer.
    fn fill_up(&mut self, iter: &mut impl Iterator<Item = char>) {
        while !self.is_full() {
            match iter.next() {
                None => break,
                Some(x) => self.0.push_back(x),
            }
        }
    }

    /// Checks whether the beginning of the buffer matches the provided pattern, the buffer should
    /// be full when this method is called.
    fn matches(&self, pat: &str) -> bool {
        self.iter().take(pat.len()).copied().eq(pat.chars())
    }

    /// Assert that the buffer is not empty and pop the first element
    fn pop_front(&mut self) -> char {
        self.0.pop_front().unwrap()
    }

    /// Assert that the buffer has at least n elements and pop the first n elements
    fn pop_front_n(&mut self, n: usize) {
        let _ = self.0.drain(..n);
    }
}

/// Same as the `Option` type but with the additional `None`-like value `Wait` used to signify
/// that an item cannot be returned at this time but that another attempt should be made (as
/// opposed to `None` which means that the iteration has concluded).
#[derive(Debug)]
enum Tription<T> {
    /// An item of type `T` is returned
    Some(T),
    /// Iteration has completed
    None,
    /// No item can be returned but another attempt should be made
    Wait,
}

impl<T> From<Option<T>> for Tription<T> {
    fn from(o: Option<T>) -> Self {
        match o {
            Some(t) => Tription::Some(t),
            None => Tription::None,
        }
    }
}

/// Represents a set of rules for matching a specific comment in a language, for example 'block
/// comment in rust' or 'line comment in haskell'.
#[derive(Copy, Clone, Debug)]
pub struct Comment {
    /// Open comment pattern, such as `/*`
    pub(crate) open_pat: &'static str,
    /// Close comment pattern, such as `*/`
    pub(crate) close_pat: &'static str,
    /// Whether this type of comment can be nested. For example, rust block comments can be
    /// nested while C block comments can't.
    pub(crate) nests: bool,
    /// Whether to return the close comment pattern. For example, in rust block comments `*/`
    /// isn't returned while in rust line comments, `\n` is returned.
    pub(crate) keep_close_pat: bool, // whether to still return close_pat as part of the text
    /// Whether to allow the close comment pattern in regular text. For example, in rust `*/`
    /// will panic unless it closes a block comment while `\n` will be treated normally.
    pub(crate) allow_close_pat: bool, // whether to allow close_pat without matching open_pat
}

/// `char` iterator that removes comments based on a list of `Comment` specifications.
/// Unclosed comments (`//...` or `/*...` or equivalents) continue until the end of the iterator.
/// Closing unopened block comments (`... */` or equivalent) causes a panic.
pub struct WithoutComments<I: Iterator<Item = char>> {
    /// Inner `char` iterator
    iter: I,
    /// Buffer used to match against open and close patterns
    buf: Buf,
    /// List of types of comments and associated rules
    comments: Box<[Comment]>,
    /// The current state. None represents normal text, i.e. not currently in a comment,
    /// Some(idx, nesting) represents that the iterator is currently in a comment, idx
    /// is the index of the current comment in self.comments, nesting is None if the current
    /// comment doesn't nest and Some(d) otherwise, where d is the current nesting depth
    /// starting at 0.
    state: Option<(usize, Option<usize>)>,
}

impl<I: Iterator<Item = char>> WithoutComments<I> {
    fn new(iter: I, comments: Box<[Comment]>, buf_len: usize) -> Self {
        Self {
            iter,
            // buffer will be filled in first call to self.next_()
            buf: Buf::new(buf_len),
            comments,
            state: None,
        }
    }

    /// Inner equivalent of `Iterator::next` returning a `Tription` instead of an `Option`.
    /// This is for the case where a block comment follows right after another
    /// (`/* ... *//* ... */` or equivalent), after reading `*/`, the buffer needs to be filled
    /// to make sure that any eventual `/*` will be matched, this is done in the next call to
    /// `next_`, thus, the calling loop in `Iterator::next` is told to wait one more iteration.
    fn next_(&mut self) -> Tription<char> {
        // at least one element missing from previous call
        self.buf.fill_up(&mut self.iter);

        if self.buf.is_empty() {
            return Tription::None;
        }

        // if in comment
        if let Some((idx, ref mut nesting)) = self.state {
            let comment = &self.comments[idx];
            let &Comment {
                open_pat,
                close_pat,
                keep_close_pat,
                ..
            } = comment;

            // check close before open to make thinks like python's '''...''' work
            if self.buf.matches(close_pat) {
                // matched close pattern

                if !keep_close_pat {
                    self.buf.pop_front_n(close_pat.len());
                }

                match nesting {
                    // non-nesting comment or top-level comment
                    None | Some(0) => self.state = None,
                    // nested comment
                    Some(d) => *d -= 1,
                }
            } else if let Some(depth) = nesting {
                if self.buf.matches(open_pat) {
                    // matched nesting open pattern
                    self.buf.pop_front_n(open_pat.len());
                    *depth += 1;
                } else {
                    self.buf.pop_front();
                }
            } else {
                self.buf.pop_front();
            }

            Tription::Wait
        } else {
            // if in text
            // #![feature(bindings_after_at)]
            // for comment @ Comment(open_pat, .., nests) in self.comments.as_ref() {
            // for each rule...
            for (idx, comment) in self.comments.iter().enumerate() {
                let Comment {
                    open_pat,
                    close_pat,
                    nests,
                    allow_close_pat,
                    ..
                } = comment;

                // if it matches open pattern, open
                if self.buf.matches(open_pat) {
                    self.buf.pop_front_n(open_pat.len());

                    let nesting = match nests {
                        true => Some(0),
                        false => None,
                    };
                    self.state = Some((idx, nesting));
                    return Tription::Wait;
                } else if !allow_close_pat && self.buf.matches(close_pat) {
                    // if close pattern forbidden, panic
                    panic!("Got \"{}\" without matching \"{}\"", close_pat, open_pat)
                }
            }

            Tription::Some(self.buf.pop_front())
        }
    }
}

impl<I: Iterator<Item = char>> Iterator for WithoutComments<I> {
    type Item = char;

    /// Simply calls `WithoutComments::next_`, a return value of `Tription::Wait` signifies
    /// that another attempt should be made, `Tription::Some` and `Tription::None` are
    /// equivalent to the same variants of the `Option` type.
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.next_() {
                Tription::None => return None,
                Tription::Some(c) => return Some(c),
                Tription::Wait => (),
            }
        }
    }
}

/// A trait to implement the `without_comments` method on all `Iterator<Item=char>`
pub trait IntoWithoutComments
where
    Self: Sized + Iterator<Item = char>,
{
    /// Returns a `WithoutComments` iterator containing self
    ///
    /// # Arguments
    ///
    /// * `language` - A boxed slice containing all the comments that the returned iterator
    /// will be removing
    ///
    /// # Example
    ///
    /// ```
    /// use no_comment::{IntoWithoutComments, languages};
    /// let with_comments = "S/*he */be/*lie*/ve//d";
    /// let without_comments = with_comments
    ///     .chars()
    ///     .without_comments(languages::rust())
    ///     .collect::<String>();
    /// assert_eq!(&without_comments, "Sbeve");
    /// ```
    fn without_comments(self, language: Box<[Comment]>) -> WithoutComments<Self> {
        let mut buf_len = 0;
        for &Comment {
            open_pat,
            close_pat,
            ..
        } in language.iter()
        {
            if open_pat.len() > buf_len {
                buf_len = open_pat.len()
            }
            if close_pat.len() > buf_len {
                buf_len = close_pat.len()
            }
        }
        assert_ne!(buf_len, 0);
        WithoutComments::new(self, language, buf_len)
    }
}

/// Blanket implementation
impl<I: Iterator<Item = char>> IntoWithoutComments for I {}
