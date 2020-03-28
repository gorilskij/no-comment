/// `char` iterator that removes rust-style line (`// ...`) and block (`/* ... */`) comments.
/// Nested block comments (`/* /* ... */ */`) are treated correctly.
/// Unclosed block comments (`/* ...`) are closed automatically when the inner iterator finishes.
/// Closing unopened block comments (`... */`) causes a panic.
pub struct WithoutComments<I: Iterator<Item=char>> {
    /// Inner `char` iterator
    iter: I,
    /// A one-`char` buffer used to match 2-long constructs `//`, `/*`, and `*/`
    hold: Option<char>,
}

/// Called after a `/*` has been read. Reads iterator until matching `*/` including nested block
/// comments and dumps the output. The next item returned by the iterator would be the character
/// straight after `*/` or `None` if `*/` occurs last or never occurs.
fn exhaust_block_comment(iter: &mut impl Iterator<Item=char>) {
    let mut depth = 0_usize;
    let mut last_read = None;
    for c in iter {
        match c {
            '*' => {
                match last_read {
                    Some('/') => {
                        depth += 1;
                        last_read = None;
                    }
                    _ => last_read = Some('*')
                }
            }

            '/' => {
                match last_read {
                    Some('*') => {
                        if depth == 0 {
                            return;
                        } else {
                            depth -= 1;
                            last_read = None;
                        }
                    }
                    _ => last_read = Some('/')
                }
            }

            c => last_read = Some(c)
        }
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

impl<I: Iterator<Item=char>> WithoutComments<I> {
    /// Inner equivalent of `Iterator::next` returning a `Tription` instead of an `Option`.
    /// This is for the case where a block comment follows right after another
    /// (`/* ... *//* ... */`), after reading `*/` and then `/`, the iterator can't yet return
    /// an item because if the next character is `*`, another comment will be started, thus it
    /// returns `Tription::Wait` instead.
    fn next_(&mut self) -> Tription<char> {
        if self.hold.is_none() {
            // return None if done
            self.hold = match self.iter.next() {
                None => return Tription::None,
                some => some,
            }
        }

        let next = match self.iter.next() {
            Some(c) => c,
            None => return self.hold.take().into(),
        };

        match (self.hold.unwrap(), next) {
            ('/', '/') => {
                self.hold = None;
                while let Some(c) = self.iter.next() {
                    if c == '\n' {
                        return Tription::Some('\n')
                    }
                }
                Tription::None
            }
            ('/', '*') => {
                self.hold = None;
                exhaust_block_comment(&mut self.iter);
                // there might be another block comment straight after
                Tription::Wait
            }
            ('*', '/') => {
                panic!("Closed block comment at top level ('*/' without matching '/*')")
            }
            (h, n) => {
                self.hold = Some(n);
                Tription::Some(h)
            }
        }
    }
}

impl<I: Iterator<Item=char>> Iterator for WithoutComments<I> {
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
pub trait IntoWithoutComments where Self: Sized + Iterator<Item=char> {
    /// Returns a `WithoutComments` iterator containing self
    ///
    /// # Example
    ///
    /// ```
    /// use no_comment::IntoWithoutComments;
    /// let with_comments = "S/*he */be/*lie*/ve//d";
    /// let without_comments = with_comments.chars().without_comments().collect::<String>();
    /// assert_eq!(&without_comments, "Sbeve");
    /// ```
    fn without_comments(self) -> WithoutComments<Self> {
        WithoutComments {
            iter: self,
            hold: None,
        }
    }
}

/// Blanket implementation
impl<I: Iterator<Item=char>> IntoWithoutComments for I {}
