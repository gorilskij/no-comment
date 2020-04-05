use crate::without_comments::Comment;

macro_rules! make_getter {
    (const $c:ident: [Comment; $_:expr], pub fn $f:ident) => {
        #[allow(dead_code)]
        pub fn $f() -> Box<[Comment]> {
            $c.iter().copied().collect::<Vec<_>>().into_boxed_slice()
        }
    };
}

make_getter!(const RUST: [Comment; 2], pub fn rust);
make_getter!(const C: [Comment; 2], pub fn c);
make_getter!(const PYTHON: [Comment; 3], pub fn python);
make_getter!(const HASKELL: [Comment; 2], pub fn haskell);

#[allow(dead_code)]
const RUST: [Comment; 2] = [
    Comment {
        open_pat: "//",
        close_pat: "\n",
        nests: false,
        keep_close_pat: true,
        allow_close_pat: true,
    },
    Comment {
        open_pat: "/*",
        close_pat: "*/",
        nests: true,
        keep_close_pat: false,
        allow_close_pat: false,
    },
];

#[allow(dead_code)]
const C: [Comment; 2] = [
    Comment {
        open_pat: "//",
        close_pat: "\n",
        nests: false,
        keep_close_pat: true,
        allow_close_pat: true,
    },
    Comment {
        open_pat: "/*",
        close_pat: "*/",
        nests: false,
        keep_close_pat: false,
        allow_close_pat: false,
    },
];

#[allow(dead_code)]
const PYTHON: [Comment; 3] = [
    Comment {
        open_pat: "#",
        close_pat: "\n",
        nests: false,
        keep_close_pat: true,
        allow_close_pat: true,
    },
    // allow_close_pat won't be checked because open_pat will match first
    Comment {
        open_pat: "'''",
        close_pat: "'''",
        nests: false,
        keep_close_pat: false,
        allow_close_pat: false,
    },
    Comment {
        open_pat: "\"\"\"",
        close_pat: "\"\"\"",
        nests: false,
        keep_close_pat: false,
        allow_close_pat: false,
    },
];

#[allow(dead_code)]
const HASKELL: [Comment; 2] = [
    Comment {
        open_pat: "--",
        close_pat: "\n",
        nests: false,
        keep_close_pat: true,
        allow_close_pat: true,
    },
    Comment {
        open_pat: "{-",
        close_pat: "-}",
        nests: true,
        keep_close_pat: false,
        allow_close_pat: false,
    },
];
