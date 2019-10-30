extern crate proc_macro;

mod decode;
mod encode;

#[link(wasm_import_module = "watt-0.1.x")]
extern "C" {
    fn token_stream_serialize(stream: handle::TokenStream) -> handle::Bytes;
    fn token_stream_deserialize(ptr: *const u8, len: usize) -> handle::TokenStream;
    fn token_stream_parse(ptr: *const u8, len: usize) -> handle::TokenStream;

    fn watt_string_new(ptr: *const u8, len: usize) -> handle::String;
    fn watt_bytes_len(bytes: handle::Bytes) -> usize;
    fn watt_bytes_read(bytes: handle::Bytes, ptr: *mut u8);
    fn watt_print_panic(message: handle::String);
}

mod handle {
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    pub struct TokenStream(pub u32);

    #[repr(transparent)]
    #[derive(Copy, Clone)]
    pub struct String(pub u32);

    #[repr(transparent)]
    #[derive(Copy, Clone)]
    pub struct Bytes(pub u32);
}

use std::char;
use std::cmp::Ordering;
use std::fmt::{self, Debug, Display};
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;
use std::ops::RangeBounds;
use std::panic::{self, PanicInfo};
use std::str::FromStr;
use std::sync::Once;

pub fn set_wasm_panic_hook() {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        panic::set_hook(Box::new(panic_hook));
    });
}

fn panic_hook(panic: &PanicInfo) {
    let string = panic.to_string();
    unsafe {
        let s = watt_string_new(string.as_ptr(), string.len());
        watt_print_panic(s);
    }
}

#[repr(transparent)]
pub struct RawTokenStream(handle::TokenStream);

impl RawTokenStream {
    pub fn into_token_stream(self) -> TokenStream {
        set_wasm_panic_hook();
        let bytes = unsafe {
            let handle = token_stream_serialize(self.0);
            let len = watt_bytes_len(handle);
            let mut ret = Vec::with_capacity(len);
            ret.set_len(len);
            watt_bytes_read(handle, ret.as_mut_ptr());
            ret
        };
        decode::decode(&bytes)
    }
}

#[derive(Clone)]
pub struct TokenStream {
    inner: Vec<TokenTree>,
}

impl From<proc_macro::TokenStream> for TokenStream {
    fn from(_: proc_macro::TokenStream) -> Self {
        unimplemented!("From<proc_macro::TokenStream> does not exist in wasm");
    }
}

pub struct LexError {
    _private: (),
}

impl TokenStream {
    pub fn new() -> Self {
        TokenStream { inner: Vec::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn into_raw_token_stream(self) -> RawTokenStream {
        let bytes = encode::encode(self);
        unsafe { RawTokenStream(token_stream_deserialize(bytes.as_ptr(), bytes.len())) }
    }
}

impl Default for TokenStream {
    fn default() -> Self {
        TokenStream::new()
    }
}

impl FromStr for TokenStream {
    type Err = LexError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let ret = unsafe {
            token_stream_parse(input.as_ptr(), input.len())
        };
        if ret.0 == u32::max_value() {
            Err(LexError { _private: () })
        } else {
            Ok(RawTokenStream(ret).into_token_stream())
        }
    }
}

impl From<TokenTree> for TokenStream {
    fn from(token: TokenTree) -> Self {
        TokenStream { inner: vec![token] }
    }
}

impl Extend<TokenTree> for TokenStream {
    fn extend<I: IntoIterator<Item = TokenTree>>(&mut self, iter: I) {
        self.inner.extend(iter);
    }
}

impl Extend<TokenStream> for TokenStream {
    fn extend<I: IntoIterator<Item = TokenStream>>(&mut self, iter: I) {
        self.inner
            .extend(iter.into_iter().flat_map(|stream| stream));
    }
}

impl FromIterator<TokenTree> for TokenStream {
    fn from_iter<I: IntoIterator<Item = TokenTree>>(iter: I) -> Self {
        let mut stream = TokenStream::new();
        stream.extend(iter);
        stream
    }
}

impl FromIterator<TokenStream> for TokenStream {
    fn from_iter<I: IntoIterator<Item = TokenStream>>(iter: I) -> Self {
        let mut stream = TokenStream::new();
        stream.extend(iter);
        stream
    }
}

impl Display for TokenStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut joint = false;
        for (i, tt) in self.inner.iter().enumerate() {
            if i != 0 && !joint {
                write!(f, " ")?;
            }
            joint = false;
            match *tt {
                TokenTree::Group(ref tt) => {
                    let (start, end) = match tt.delimiter() {
                        Delimiter::Parenthesis => ("(", ")"),
                        Delimiter::Brace => ("{", "}"),
                        Delimiter::Bracket => ("[", "]"),
                        Delimiter::None => ("", ""),
                    };
                    if tt.stream().into_iter().next().is_none() {
                        write!(f, "{} {}", start, end)?
                    } else {
                        write!(f, "{} {} {}", start, tt.stream(), end)?
                    }
                }
                TokenTree::Ident(ref tt) => write!(f, "{}", tt)?,
                TokenTree::Punct(ref tt) => {
                    write!(f, "{}", tt.as_char())?;
                    match tt.spacing() {
                        Spacing::Alone => {}
                        Spacing::Joint => joint = true,
                    }
                }
                TokenTree::Literal(ref tt) => write!(f, "{}", tt)?,
            }
        }

        Ok(())
    }
}

impl Debug for TokenStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("TokenStream ")?;
        f.debug_list().entries(self.clone()).finish()
    }
}

impl Debug for LexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("LexError")
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Span {
    handle: u32,
}

impl Span {
    pub fn call_site() -> Self {
        Span { handle: 0 }
    }

    pub fn join(&self, other: Span) -> Option<Span> {
        let _ = other;
        None
    }
}

pub enum TokenTree {
    Group(Group),
    Ident(Ident),
    Punct(Punct),
    Literal(Literal),
}

impl Clone for TokenTree {
    fn clone(&self) -> Self {
        match self {
            TokenTree::Group(group) => TokenTree::Group(group.clone()),
            TokenTree::Ident(ident) => TokenTree::Ident(ident.clone()),
            TokenTree::Punct(punct) => TokenTree::Punct(punct.clone()),
            TokenTree::Literal(literal) => TokenTree::Literal(literal.clone()),
        }
    }
}

impl TokenTree {
    pub fn span(&self) -> Span {
        match self {
            TokenTree::Group(t) => t.span(),
            TokenTree::Ident(t) => t.span(),
            TokenTree::Punct(t) => t.span(),
            TokenTree::Literal(t) => t.span(),
        }
    }

    pub fn set_span(&mut self, span: Span) {
        match self {
            TokenTree::Group(t) => t.set_span(span),
            TokenTree::Ident(t) => t.set_span(span),
            TokenTree::Punct(t) => t.set_span(span),
            TokenTree::Literal(t) => t.set_span(span),
        }
    }
}

impl From<Group> for TokenTree {
    fn from(g: Group) -> TokenTree {
        TokenTree::Group(g)
    }
}

impl From<Ident> for TokenTree {
    fn from(g: Ident) -> TokenTree {
        TokenTree::Ident(g)
    }
}

impl From<Punct> for TokenTree {
    fn from(g: Punct) -> TokenTree {
        TokenTree::Punct(g)
    }
}

impl From<Literal> for TokenTree {
    fn from(g: Literal) -> TokenTree {
        TokenTree::Literal(g)
    }
}

impl Display for TokenTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenTree::Group(group) => Display::fmt(group, f),
            TokenTree::Ident(ident) => Display::fmt(ident, f),
            TokenTree::Punct(punct) => Display::fmt(punct, f),
            TokenTree::Literal(literal) => Display::fmt(literal, f),
        }
    }
}

impl Debug for TokenTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenTree::Group(group) => Debug::fmt(group, f),
            TokenTree::Ident(ident) => Debug::fmt(ident, f),
            TokenTree::Punct(punct) => Debug::fmt(punct, f),
            TokenTree::Literal(literal) => Debug::fmt(literal, f),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Group {
    delimiter: Delimiter,
    stream: TokenStream,
    span: Span,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Delimiter {
    Parenthesis,
    Brace,
    Bracket,
    None,
}

impl Group {
    pub fn new(delimiter: Delimiter, stream: TokenStream) -> Self {
        Group {
            delimiter: delimiter,
            stream: stream,
            span: Span::call_site(),
        }
    }

    pub fn delimiter(&self) -> Delimiter {
        self.delimiter
    }

    pub fn stream(&self) -> TokenStream {
        self.stream.clone()
    }

    pub fn span(&self) -> Span {
        self.span
    }

    pub fn span_open(&self) -> Span {
        self.span
    }

    pub fn span_close(&self) -> Span {
        self.span
    }

    pub fn set_span(&mut self, span: Span) {
        self.span = span;
    }
}

impl Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (left, right) = match self.delimiter {
            Delimiter::Parenthesis => ("(", ")"),
            Delimiter::Brace => ("{", "}"),
            Delimiter::Bracket => ("[", "]"),
            Delimiter::None => ("", ""),
        };

        f.write_str(left)?;
        Display::fmt(&self.stream, f)?;
        f.write_str(right)?;

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct Punct {
    op: char,
    spacing: Spacing,
    span: Span,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Spacing {
    Alone,
    Joint,
}

impl Punct {
    pub fn new(op: char, spacing: Spacing) -> Self {
        Punct {
            op,
            spacing,
            span: Span::call_site(),
        }
    }

    pub fn as_char(&self) -> char {
        self.op
    }

    pub fn spacing(&self) -> Spacing {
        self.spacing
    }

    pub fn span(&self) -> Span {
        self.span
    }

    pub fn set_span(&mut self, span: Span) {
        self.span = span;
    }
}

impl Display for Punct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.op, f)
    }
}

#[derive(Clone)]
pub struct Ident {
    sym: String,
    span: Span,
    raw: bool,
}

impl Ident {
    fn _new(string: &str, raw: bool, span: Span) -> Ident {
        Ident::validate(string);

        Ident {
            sym: string.to_owned(),
            span: span,
            raw: raw,
        }
    }

    pub fn new(string: &str, span: Span) -> Ident {
        Ident::_new(string, false, span)
    }

    pub fn span(&self) -> Span {
        self.span
    }

    pub fn set_span(&mut self, span: Span) {
        self.span = span;
    }

    fn validate(string: &str) {
        let validate = string;
        if validate.is_empty() {
            panic!("Ident is not allowed to be empty; use Option<Ident>");
        }

        if validate.bytes().all(|digit| digit >= b'0' && digit <= b'9') {
            panic!("Ident cannot be a number; use Literal instead");
        }

        fn ident_ok(string: &str) -> bool {
            let mut chars = string.chars();
            let first = chars.next().unwrap();
            if !is_ident_start(first) {
                return false;
            }
            for ch in chars {
                if !is_ident_continue(ch) {
                    return false;
                }
            }
            true
        }

        if !ident_ok(validate) {
            panic!("{:?} is not a valid Ident", string);
        }

        #[inline]
        fn is_ident_start(c: char) -> bool {
            ('a' <= c && c <= 'z') || ('A' <= c && c <= 'Z') || c == '_'
            /*|| (c > '\x7f' && UnicodeXID::is_xid_start(c))*/ // TODO
        }

        #[inline]
        fn is_ident_continue(c: char) -> bool {
            ('a' <= c && c <= 'z') || ('A' <= c && c <= 'Z') || c == '_' || ('0' <= c && c <= '9')
            /*|| (c > '\x7f' && UnicodeXID::is_xid_continue(c))*/ // TODO
        }
    }
}

impl PartialEq for Ident {
    fn eq(&self, other: &Ident) -> bool {
        self.sym == other.sym && self.raw == other.raw
    }
}

impl<T> PartialEq<T> for Ident
where
    T: ?Sized + AsRef<str>,
{
    fn eq(&self, other: &T) -> bool {
        let other = other.as_ref();
        if self.raw {
            other.starts_with("r#") && self.sym == other[2..]
        } else {
            self.sym == other
        }
    }
}

impl Eq for Ident {}

impl PartialOrd for Ident {
    fn partial_cmp(&self, other: &Ident) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Ident {
    fn cmp(&self, other: &Ident) -> Ordering {
        self.to_string().cmp(&other.to_string())
    }
}

impl Hash for Ident {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.to_string().hash(hasher);
    }
}

impl Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.raw {
            f.write_str("r#")?;
        }
        Display::fmt(&self.sym, f)
    }
}

impl Debug for Ident {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut debug = f.debug_struct("Ident");
        debug.field("sym", &format_args!("{}", self));
        debug.field("span", &self.span);
        debug.finish()
    }
}

#[derive(Clone, Debug)]
pub struct Literal {
    text: String,
    span: Span,
}

macro_rules! suffixed_numbers {
    ($($name:ident => $kind:ident,)*) => ($(
        pub fn $name(n: $kind) -> Literal {
            Literal::_new(format!(concat!("{}", stringify!($kind)), n))
        }
    )*)
}

macro_rules! unsuffixed_numbers {
    ($($name:ident => $kind:ident,)*) => ($(
        pub fn $name(n: $kind) -> Literal {
            Literal::_new(n.to_string())
        }
    )*)
}

impl Literal {
    fn _new(text: String) -> Literal {
        Literal {
            text: text,
            span: Span::call_site(),
        }
    }

    suffixed_numbers! {
        u8_suffixed => u8,
        u16_suffixed => u16,
        u32_suffixed => u32,
        u64_suffixed => u64,
        usize_suffixed => usize,
        i8_suffixed => i8,
        i16_suffixed => i16,
        i32_suffixed => i32,
        i64_suffixed => i64,
        isize_suffixed => isize,

        f32_suffixed => f32,
        f64_suffixed => f64,
        u128_suffixed => u128,
        i128_suffixed => i128,
    }

    unsuffixed_numbers! {
        u8_unsuffixed => u8,
        u16_unsuffixed => u16,
        u32_unsuffixed => u32,
        u64_unsuffixed => u64,
        usize_unsuffixed => usize,
        i8_unsuffixed => i8,
        i16_unsuffixed => i16,
        i32_unsuffixed => i32,
        i64_unsuffixed => i64,
        isize_unsuffixed => isize,
        u128_unsuffixed => u128,
        i128_unsuffixed => i128,
    }

    pub fn f32_unsuffixed(f: f32) -> Literal {
        let mut s = f.to_string();
        if !s.contains(".") {
            s.push_str(".0");
        }
        Literal::_new(s)
    }

    pub fn f64_unsuffixed(f: f64) -> Literal {
        let mut s = f.to_string();
        if !s.contains(".") {
            s.push_str(".0");
        }
        Literal::_new(s)
    }

    pub fn string(t: &str) -> Literal {
        let mut text = String::with_capacity(t.len() + 2);
        text.push('"');
        for c in t.chars() {
            if c == '\'' {
                // escape_default turns this into "\'" which is unnecessary.
                text.push(c);
            } else {
                text.extend(c.escape_default());
            }
        }
        text.push('"');
        Literal::_new(text)
    }

    pub fn character(t: char) -> Literal {
        let mut text = String::new();
        text.push('\'');
        if t == '"' {
            // escape_default turns this into '\"' which is unnecessary.
            text.push(t);
        } else {
            text.extend(t.escape_default());
        }
        text.push('\'');
        Literal::_new(text)
    }

    pub fn byte_string(bytes: &[u8]) -> Literal {
        let mut escaped = "b\"".to_string();
        for b in bytes {
            match *b {
                b'\0' => escaped.push_str(r"\0"),
                b'\t' => escaped.push_str(r"\t"),
                b'\n' => escaped.push_str(r"\n"),
                b'\r' => escaped.push_str(r"\r"),
                b'"' => escaped.push_str("\\\""),
                b'\\' => escaped.push_str("\\\\"),
                b'\x20'..=b'\x7E' => escaped.push(*b as char),
                _ => escaped.push_str(&format!("\\x{:02X}", b)),
            }
        }
        escaped.push('"');
        Literal::_new(escaped)
    }

    pub fn span(&self) -> Span {
        self.span
    }

    pub fn set_span(&mut self, span: Span) {
        self.span = span;
    }

    pub fn subspan<R: RangeBounds<usize>>(&self, range: R) -> Option<Span> {
        let _ = range;
        None
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.text, f)
    }
}

pub mod token_stream {
    use super::*;

    pub use crate::TokenStream;

    #[derive(Clone)]
    pub struct IntoIter(std::vec::IntoIter<TokenTree>);

    impl Iterator for IntoIter {
        type Item = TokenTree;

        fn next(&mut self) -> Option<TokenTree> {
            self.0.next()
        }
    }

    impl IntoIterator for TokenStream {
        type Item = TokenTree;
        type IntoIter = IntoIter;

        fn into_iter(self) -> Self::IntoIter {
            IntoIter(self.inner.into_iter())
        }
    }
}
