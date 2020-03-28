pub struct WithoutComments<I: Iterator<Item=char>> {
    iter: I,
    hold: Option<char>,
}

// called after a '/*' has been read, exhausts block comment including nested blocks
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

#[derive(Debug)]
enum Tription<T> {
    Some(T),
    None,
    Wait, // this means that the iteration isn't done but
          // one more call to next is required for the next element
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
                panic!("*/ without /* (closed block comment at top level)")
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

pub trait IntoWithoutComments where Self: Sized + Iterator<Item=char> {
    fn without_comments(self) -> WithoutComments<Self>;
}

impl<I: Iterator<Item=char>> IntoWithoutComments for I {
    fn without_comments(self) -> WithoutComments<Self> {
        WithoutComments {
            iter: self,
            hold: None,
        }
    }
}
