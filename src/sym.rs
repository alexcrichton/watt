use crate::data::Data;
use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};
use std::char;
use std::cmp::Ordering;
use std::iter::once;
use std::str::FromStr;

const SENTINEL: i32 = u32::max_value() as i32;
const TOKEN_GROUP: i32 = 0;
const TOKEN_IDENT: i32 = 1;
const TOKEN_PUNCT: i32 = 2;
const TOKEN_LITERAL: i32 = 3;
const DELIMITER_PARENTHESIS: i32 = 0;
const DELIMITER_BRACE: i32 = 1;
const DELIMITER_BRACKET: i32 = 2;
const DELIMITER_NONE: i32 = 3;
const SPACING_ALONE: i32 = 0;
const SPACING_JOINT: i32 = 1;
const ORDERING_LESS: i32 = 0;
const ORDERING_EQUAL: i32 = 1;
const ORDERING_GREATER: i32 = 2;

pub fn token_stream_new() -> i32 {
    Data::with(|d| {
        d.tokenstream.push(TokenStream::new())
    })
}

pub fn token_stream_is_empty(stream: i32) -> i32 {
    Data::with(|d| {
        let stream = &d.tokenstream[stream];
        let is_empty = stream.is_empty();
        is_empty as i32
    })
}

pub fn token_stream_from_str(string: i32) -> i32 {
    Data::with(|d| {
        let string = &d.string[string];
        let result = TokenStream::from_str(string);
        match result {
            Ok(stream) => d.tokenstream.push(stream),
            Err(_error) => SENTINEL,
        }
    })
}

pub fn token_stream_into_iter(stream: i32) -> i32 {
    Data::with(|d| {
        let stream = &d.tokenstream[stream];
        let iter = stream.clone().into_iter();
        d.intoiter.push(iter)
    })
}

pub fn token_stream_iter_next(iter: i32) -> i32 {
    Data::with(|d| {
        let iter = &mut d.intoiter[iter];
        match iter.next() {
            Some(token) => d.tokentree.push(token),
            None => SENTINEL,
        }
    })
}

pub fn token_stream_from_group(group: i32) -> i32 {
    Data::with(|d| {
        let group = &d.group[group];
        let tree = TokenTree::Group(group.clone());
        d.tokenstream.push(TokenStream::from(tree))
    })
}

pub fn token_stream_from_ident(ident: i32) -> i32 {
    Data::with(|d| {
        let ident = &d.ident[ident];
        let tree = TokenTree::Ident(ident.clone());
        d.tokenstream.push(TokenStream::from(tree))
    })
}

pub fn token_stream_from_punct(punct: i32) -> i32 {
    Data::with(|d| {
        let punct = &d.punct[punct];
        let tree = TokenTree::Punct(punct.clone());
        d.tokenstream.push(TokenStream::from(tree))
    })
}

pub fn token_stream_from_literal(literal: i32) -> i32 {
    Data::with(|d| {
        let literal = &d.literal[literal];
        let tree = TokenTree::Literal(literal.clone());
        d.tokenstream.push(TokenStream::from(tree))
    })
}

pub fn token_stream_push_group(stream: i32, group: i32) {
    Data::with(|d| {
        let group = &d.group[group];
        let stream = &mut d.tokenstream[stream];
        stream.extend(once(TokenTree::Group(group.clone())));
    })
}

pub fn token_stream_push_ident(stream: i32, ident: i32) {
    Data::with(|d| {
        let ident = &d.ident[ident];
        let stream = &mut d.tokenstream[stream];
        stream.extend(once(TokenTree::Ident(ident.clone())));
    })
}

pub fn token_stream_push_punct(stream: i32, punct: i32) {
    Data::with(|d| {
        let punct = &d.punct[punct];
        let stream = &mut d.tokenstream[stream];
        stream.extend(once(TokenTree::Punct(punct.clone())));
    })
}

pub fn token_stream_push_literal(stream: i32, literal: i32) {
    Data::with(|d| {
        let literal = &d.literal[literal];
        let stream = &mut d.tokenstream[stream];
        stream.extend(once(TokenTree::Literal(literal.clone())));
    })
}

pub fn token_stream_extend(stream: i32, next: i32) {
    Data::with(|d| {
        let next = d.tokenstream[next].clone();
        let stream = &mut d.tokenstream[stream];
        stream.extend(next);
    })
}

pub fn token_tree_kind(token: i32) -> i32 {
    Data::with(|d| {
        let token = &d.tokentree[token];
        match token {
            TokenTree::Group(_) => TOKEN_GROUP,
            TokenTree::Ident(_) => TOKEN_IDENT,
            TokenTree::Punct(_) => TOKEN_PUNCT,
            TokenTree::Literal(_) => TOKEN_LITERAL,
        }
    })
}

pub fn token_tree_unwrap_group(token: i32) -> i32 {
    Data::with(|d| {
        let token = &d.tokentree[token];
        let group = match token {
            TokenTree::Group(group) => group,
            _ => unreachable!(),
        };
        d.group.push(group.clone())
    })
}

pub fn token_tree_unwrap_ident(token: i32) -> i32 {
    Data::with(|d| {
        let token = &d.tokentree[token];
        let ident = match token {
            TokenTree::Ident(ident) => ident,
            _ => unreachable!(),
        };
        d.ident.push(ident.clone())
    })
}

pub fn token_tree_unwrap_punct(token: i32) -> i32 {
    Data::with(|d| {
        let token = &d.tokentree[token];
        let punct = match token {
            TokenTree::Punct(punct) => punct,
            _ => unreachable!(),
        };
        d.punct.push(punct.clone())
    })
}

pub fn token_tree_unwrap_literal(token: i32) -> i32 {
    Data::with(|d| {
        let token = &d.tokentree[token];
        let literal = match token {
            TokenTree::Literal(literal) => literal,
            _ => unreachable!(),
        };
        d.literal.push(literal.clone())
    })
}

pub fn span_call_site() -> i32 {
    Data::with(|d| {
        d.span.push(Span::call_site())
    })
}

pub fn group_new(delimiter: i32, stream: i32) -> i32 {
    Data::with(|d| {
        let stream = &d.tokenstream[stream];
        let delimiter = if delimiter == DELIMITER_PARENTHESIS {
            Delimiter::Parenthesis
        } else if delimiter == DELIMITER_BRACE {
            Delimiter::Brace
        } else if delimiter == DELIMITER_BRACKET {
            Delimiter::Bracket
        } else if delimiter == DELIMITER_NONE {
            Delimiter::None
        } else {
            unreachable!()
        };
        let group = Group::new(delimiter, stream.clone());
        d.group.push(group)
    })
}

pub fn group_delimiter(group: i32) -> i32 {
    Data::with(|d| {
        let group = &d.group[group];
        match group.delimiter() {
            Delimiter::Parenthesis => DELIMITER_PARENTHESIS,
            Delimiter::Brace => DELIMITER_BRACE,
            Delimiter::Bracket => DELIMITER_BRACKET,
            Delimiter::None => DELIMITER_NONE,
        }
    })
}

pub fn group_stream(group: i32) -> i32 {
    Data::with(|d| {
        let group = &d.group[group];
        d.tokenstream.push(group.stream())
    })
}

pub fn group_span(group: i32) -> i32 {
    Data::with(|d| {
        let group = &d.group[group];
        d.span.push(group.span())
    })
}

pub fn group_set_span(group: i32, span: i32) {
    Data::with(|d| {
        let span = d.span[span];
        let group = &mut d.group[group];
        group.set_span(span);
    })
}

pub fn punct_new(op: i32, spacing: i32) -> i32 {
    Data::with(|d| {
        let spacing = if spacing == SPACING_ALONE {
            Spacing::Alone
        } else if spacing == SPACING_JOINT {
            Spacing::Joint
        } else {
            unreachable!()
        };
        let op = char::from_u32(op as u32).unwrap();
        d.punct.push(Punct::new(op, spacing))
    })
}

pub fn punct_as_char(punct: i32) -> i32 {
    Data::with(|d| {
        let punct = &d.punct[punct];
        punct.as_char() as i32
    })
}

pub fn punct_spacing(punct: i32) -> i32 {
    Data::with(|d| {
        let punct = &d.punct[punct];
        match punct.spacing() {
            Spacing::Alone => SPACING_ALONE,
            Spacing::Joint => SPACING_JOINT,
        }
    })
}

pub fn punct_span(punct: i32) -> i32 {
    Data::with(|d| {
        let punct = &d.punct[punct];
        d.span.push(punct.span())
    })
}

pub fn punct_set_span(punct: i32, span: i32) {
    Data::with(|d| {
        let span = d.span[span];
        let punct = &mut d.punct[punct];
        punct.set_span(span);
    })
}

pub fn ident_new(string: i32, span: i32) -> i32 {
    Data::with(|d| {
        let span = d.span[span];
        let string = &d.string[string];
        d.ident.push(Ident::new(string, span))
    })
}

pub fn ident_span(ident: i32) -> i32 {
    Data::with(|d| {
        let ident = &d.ident[ident];
        d.span.push(ident.span())
    })
}

pub fn ident_set_span(ident: i32, span: i32) {
    Data::with(|d| {
        let span = d.span[span];
        let ident = &mut d.ident[ident];
        ident.set_span(span);
    })
}

pub fn ident_eq(ident: i32, other: i32) -> i32 {
    Data::with(|d| {
        let other = &d.ident[other];
        let ident = &d.ident[ident];
        let eq = ident.to_string() == other.to_string();
        eq as i32
    })
}

pub fn ident_eq_str(ident: i32, other: i32) -> i32 {
    Data::with(|d| {
        let other = &d.string[other];
        let ident = &d.ident[ident];
        let eq = ident.to_string() == *other;
        eq as i32
    })
}

pub fn ident_cmp(ident: i32, other: i32) -> i32 {
    Data::with(|d| {
        let other = &d.ident[other];
        let ident = &d.ident[ident];
        let cmp = match ident.to_string().cmp(&other.to_string()) {
            Ordering::Less => ORDERING_LESS,
            Ordering::Equal => ORDERING_EQUAL,
            Ordering::Greater => ORDERING_GREATER,
        };
        cmp
    })
}

pub fn literal_u8_suffixed(n: i32) -> i32 {
    Data::with(|d| {
        d.literal.push(Literal::u8_suffixed(n as u8))
    })
}

pub fn literal_u16_suffixed(n: i32) -> i32 {
    Data::with(|d| {
        d.literal.push(Literal::u16_suffixed(n as u16))
    })
}

pub fn literal_u32_suffixed(n: i32) -> i32 {
    Data::with(|d| d.literal.push(Literal::u32_suffixed(n as u32)))
}

pub fn literal_u64_suffixed(n: i64) -> i32 {
    Data::with(|d| d.literal.push(Literal::u64_suffixed(n as u64)))
}

pub fn literal_u128_suffixed(lo: i64, hi: i64) -> i32 {
    Data::with(|d| {
        let n = ((hi as u128) << 64) + lo as u128;
        d.literal.push(Literal::u128_suffixed(n))
    })
}

pub fn literal_usize_suffixed(n: i32) -> i32 {
    Data::with(|d| d.literal.push(Literal::usize_suffixed(n as usize)))
}

pub fn literal_i8_suffixed(n: i32) -> i32 {
    Data::with(|d| d.literal.push(Literal::i8_suffixed(n as i8)))
}

pub fn literal_i16_suffixed(n: i32) -> i32 {
    Data::with(|d| d.literal.push(Literal::i16_suffixed(n as i16)))
}

pub fn literal_i32_suffixed(n: i32) -> i32 {
    Data::with(|d| d.literal.push(Literal::i32_suffixed(n)))
}

pub fn literal_i64_suffixed(n: i64) -> i32 {
    Data::with(|d| d.literal.push(Literal::i64_suffixed(n)))
}

pub fn literal_i128_suffixed(lo: i64, hi: i64) -> i32 {
    Data::with(|d| {
        let n = (((hi as u128) << 64) + lo as u128) as i128;
        d.literal.push(Literal::i128_suffixed(n))
    })
}

pub fn literal_isize_suffixed(n: i32) -> i32 {
    Data::with(|d| d.literal.push(Literal::isize_suffixed(n as isize)))
}

pub fn literal_u8_unsuffixed(n: i32) -> i32 {
    Data::with(|d| d.literal.push(Literal::u8_unsuffixed(n as u8)))
}

pub fn literal_u16_unsuffixed(n: i32) -> i32 {
    Data::with(|d| d.literal.push(Literal::u16_unsuffixed(n as u16)))
}

pub fn literal_u32_unsuffixed(n: i32) -> i32 {
    Data::with(|d| d.literal.push(Literal::u32_unsuffixed(n as u32)))
}

pub fn literal_u64_unsuffixed(n: i64) -> i32 {
    Data::with(|d| d.literal.push(Literal::u64_unsuffixed(n as u64)))
}

pub fn literal_u128_unsuffixed(lo: i64, hi: i64) -> i32 {
    Data::with(|d| {
        let n = ((hi as u128) << 64) + lo as u128;
        d.literal.push(Literal::u128_unsuffixed(n))
    })
}

pub fn literal_usize_unsuffixed(n: i32) -> i32 {
    Data::with(|d| d.literal.push(Literal::usize_unsuffixed(n as usize)))
}

pub fn literal_i8_unsuffixed(n: i32) -> i32 {
    Data::with(|d| d.literal.push(Literal::i8_unsuffixed(n as i8)))
}

pub fn literal_i16_unsuffixed(n: i32) -> i32 {
    Data::with(|d| d.literal.push(Literal::i16_unsuffixed(n as i16)))
}

pub fn literal_i32_unsuffixed(n: i32) -> i32 {
    Data::with(|d| d.literal.push(Literal::i32_unsuffixed(n)))
}

pub fn literal_i64_unsuffixed(n: i64) -> i32 {
    Data::with(|d| d.literal.push(Literal::i64_unsuffixed(n)))
}

pub fn literal_i128_unsuffixed(lo: i64, hi: i64) -> i32 {
    Data::with(|d| {
        let n = (((hi as u128) << 64) + lo as u128) as i128;
        d.literal.push(Literal::i128_unsuffixed(n))
    })
}

pub fn literal_isize_unsuffixed(n: i32) -> i32 {
    Data::with(|d| d.literal.push(Literal::isize_unsuffixed(n as isize)))
}

pub fn literal_f64_unsuffixed(f: f64) -> i32 {
    Data::with(|d| d.literal.push(Literal::f64_unsuffixed(f)))
}

pub fn literal_f64_suffixed(f: f64) -> i32 {
    Data::with(|d| d.literal.push(Literal::f64_suffixed(f)))
}

pub fn literal_f32_unsuffixed(f: f32) -> i32 {
    Data::with(|d| d.literal.push(Literal::f32_unsuffixed(f)))
}

pub fn literal_f32_suffixed(f: f32) -> i32 {
    Data::with(|d| d.literal.push(Literal::f32_suffixed(f)))
}

pub fn literal_string(s: i32) -> i32 {
    Data::with(|d| {
        let string = &d.string[s];
        d.literal.push(Literal::string(string))
    })
}

pub fn literal_character(ch: i32) -> i32 {
    Data::with(|d| {
        let ch = char::from_u32(ch as u32).unwrap();
        d.literal.push(Literal::character(ch))
    })
}

pub fn literal_byte_string(s: i32) -> i32 {
    Data::with(|d| {
        let bytes = &d.bytes[s];
        d.literal.push(Literal::byte_string(bytes))
    })
}

pub fn literal_span(lit: i32) -> i32 {
    Data::with(|d| {
        let literal = &d.literal[lit];
        d.span.push(literal.span())
    })
}

pub fn literal_set_span(lit: i32, span: i32) {
    Data::with(|d| {
        let span = d.span[span];
        let literal = &mut d.literal[lit];
        literal.set_span(span);
    })
}

pub fn token_stream_clone(obj: i32) -> i32 {
    Data::with(|d| {
        let clone = d.tokenstream[obj].clone();
        d.tokenstream.push(clone)
    })
}

pub fn group_clone(obj: i32) -> i32 {
    Data::with(|d| {
        let clone = d.group[obj].clone();
        d.group.push(clone)
    })
}

pub fn ident_clone(obj: i32) -> i32 {
    Data::with(|d| {
        let clone = d.ident[obj].clone();
        d.ident.push(clone)
    })
}

pub fn punct_clone(obj: i32) -> i32 {
    Data::with(|d| {
        let clone = d.punct[obj].clone();
        d.punct.push(clone)
    })
}

pub fn literal_clone(obj: i32) -> i32 {
    Data::with(|d| {
        let clone = d.literal[obj].clone();
        d.literal.push(clone)
    })
}

pub fn token_stream_iter_clone(obj: i32) -> i32 {
    Data::with(|d| {
        let clone = d.intoiter[obj].clone();
        d.intoiter.push(clone)
    })
}

pub fn token_stream_to_string(ts: i32) -> i32 {
    Data::with(|d| {
        let string = d.tokenstream[ts].to_string();
        d.string.push(string)
    })
}

pub fn group_to_string(group: i32) -> i32 {
    Data::with(|d| {
        let string = d.group[group].to_string();
        d.string.push(string)
    })
}

pub fn ident_to_string(ident: i32) -> i32 {
    Data::with(|d| {
        let string = d.ident[ident].to_string();
        d.string.push(string)
    })
}

pub fn punct_to_string(punct: i32) -> i32 {
    Data::with(|d| {
        let string = d.punct[punct].to_string();
        d.string.push(string)
    })
}

pub fn literal_to_string(lit: i32) -> i32 {
    Data::with(|d| {
        let string = d.literal[lit].to_string();
        d.string.push(string)
    })
}

pub fn token_stream_debug(tok: i32) -> i32 {
    Data::with(|d| {
        let debug = format!("{:?}", d.tokenstream[tok]);
        d.string.push(debug)
    })
}

pub fn group_debug(group: i32) -> i32 {
    Data::with(|d| {
        let debug = format!("{:?}", d.group[group]);
        d.string.push(debug)
    })
}

pub fn ident_debug(ident: i32) -> i32 {
    Data::with(|d| {
        let debug = format!("{:?}", d.ident[ident]);
        d.string.push(debug)
    })
}

pub fn punct_debug(punct: i32) -> i32 {
    Data::with(|d| {
        let debug = format!("{:?}", d.punct[punct]);
        d.string.push(debug)
    })
}

pub fn literal_debug(lit: i32) -> i32 {
    Data::with(|d| {
        let debug = format!("{:?}", d.literal[lit]);
        d.string.push(debug)
    })
}

pub fn span_debug(span: i32) -> i32 {
    Data::with(|d| {
        let debug = format!("{:?}", d.span[span]);
        d.string.push(debug)
    })
}

pub fn watt_string_with_capacity(cap: i32) -> i32 {
    Data::with(|d| d.string.push(String::with_capacity(cap as usize)))
}

pub fn watt_string_push_char(string: i32, ch: i32) {
    Data::with(|d| {
        d.string[string].push(char::from_u32(ch as u32).unwrap());
    })
}

pub fn watt_string_len(string: i32) -> i32 {
    Data::with(|d| d.string[string].len() as i32)
}

pub fn watt_string_char_at(string: i32, pos: i32) -> i32 {
    Data::with(|d| {
        let string = &d.string[string];
        string[pos as usize..].chars().next().unwrap() as i32
    })
}

pub fn watt_bytes_with_capacity(cap: i32) -> i32 {
    Data::with(|d| d.bytes.push(Vec::with_capacity(cap as usize)))
}

pub fn watt_bytes_push(bytes: i32, byte: i32) {
    Data::with(|d| {
        let bytes = &mut d.bytes[bytes];
        bytes.push(byte as u8);
    })
}

pub fn watt_print_panic(string: i32) {
    Data::with(|d| panic!("{}", d.string[string]))
}
