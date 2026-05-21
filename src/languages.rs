use crate::without_comments::Comment;

#[allow(dead_code)]
pub const RUST: [Comment; 2] = [
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
pub const C: [Comment; 2] = [
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
pub const PYTHON: [Comment; 3] = [
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
pub const HASKELL: [Comment; 2] = [
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
