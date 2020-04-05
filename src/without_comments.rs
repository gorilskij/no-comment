use std::collections::VecDeque;

#[derive(Deref, Debug)]
#[repr(transparent)]
struct Buf(VecDeque<char>);

impl Buf {
    fn new(max_len: usize) -> Self {
        Self(VecDeque::with_capacity(max_len))
    }

    fn is_full(&self) -> bool {
        if self.len() > self.capacity() {
            panic!("len > cap")
        }
        self.len() == self.capacity()
    }

    fn fill_up(&mut self, iter: &mut impl Iterator<Item = char>) {
        while !self.is_full() {
            match iter.next() {
                None => break,
                Some(x) => self.0.push_back(x),
            }
        }
    }

    fn matches(&self, pat: &str) -> bool {
        self.iter().take(pat.len()).copied().eq(pat.chars())
    }

    fn pop_front(&mut self) -> char {
        self.0.pop_front().unwrap()
    }

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

#[derive(Copy, Clone, Debug)]
pub struct Comment {
    pub(crate) open_pat: &'static str,
    pub(crate) close_pat: &'static str,
    pub(crate) nests: bool,
    pub(crate) keep_close_pat: bool, // whether to still return close_pat as part of the text
    pub(crate) allow_close_pat: bool, // whether to allow close_pat without matching open_pat
}

/// `char` iterator that removes rust-style line (`// ...`) and block (`/* ... */`) comments.
/// Nested block comments (`/* /* ... */ */`) are treated correctly.
/// Unclosed block comments (`/* ...`) are closed automatically when the inner iterator finishes.
/// Closing unopened block comments (`... */`) causes a panic.
pub struct WithoutComments<I: Iterator<Item = char>> {
    /// Inner `char` iterator
    iter: I,
    buf: Buf,
    comments: Box<[Comment]>,
    // (comments_idx, nest_depth)
    state: Option<(usize, Option<usize>)>,
}

/// Called after a `/*` has been read. Reads iterator until matching `*/` including nested block
/// comments and dumps the output. The next item returned by the iterator would be the character
/// straight after `*/` or `None` if `*/` occurs last or never occurs.
// TODO reimplement
// fn exhaust_block_comment(iter: &mut impl Iterator<Item = char>) {
//     let mut depth = 0_usize;
//     let mut last_read = None;
//     for c in iter {
//         match c {
//             '*' => match last_read {
//                 Some('/') => {
//                     depth += 1;
//                     last_read = None;
//                 }
//                 _ => last_read = Some('*'),
//             },
//
//             '/' => match last_read {
//                 Some('*') => {
//                     if depth == 0 {
//                         return;
//                     } else {
//                         depth -= 1;
//                         last_read = None;
//                     }
//                 }
//                 _ => last_read = Some('/'),
//             },
//
//             c => last_read = Some(c),
//         }
//     }
// }

impl<I: Iterator<Item = char>> WithoutComments<I> {
    fn new(mut iter: I, comments: Box<[Comment]>, buf_len: usize) -> Self {
        let mut buf = Buf::new(buf_len);
        buf.fill_up(&mut iter);

        Self {
            iter,
            buf,
            comments,
            state: None,
        }
    }

    /// Inner equivalent of `Iterator::next` returning a `Tription` instead of an `Option`.
    /// This is for the case where a block comment follows right after another
    /// (`/* ... *//* ... */`), after reading `*/` and then `/`, the iterator can't yet return
    /// an item because if the next character is `*`, another comment will be started, thus it
    /// returns `Tription::Wait` instead.
    fn next_(&mut self) -> Tription<char> {
        // at least one element missing from previous call
        self.buf.fill_up(&mut self.iter);

        if self.buf.is_empty() {
            return Tription::None;
        }

        // if in comment
        if let Some((idx, ref mut nest_depth)) = self.state {
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

                match nest_depth {
                    // non-nesting comment or top-level comment
                    None | Some(0) => self.state = None,
                    // nested comment
                    Some(d) => *d -= 1,
                }
            } else if let Some(depth) = nest_depth {
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
    /// # Example
    ///
    /// ```
    /// use no_comment::{IntoWithoutComments, languages};
    /// let with_comments = "S/*he */be/*lie*/ve//d";
    /// let without_comments = with_comments.chars().without_comments(languages::rust()).collect::<String>();
    /// assert_eq!(&without_comments, "Sbeve");
    /// ```
    fn without_comments(self, language: Box<[Comment]>) -> WithoutComments<Self> {
        let mut buf_len = 0;
        for Comment {
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
