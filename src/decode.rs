use crate::data::Data;
use proc_macro::*;
use std::char;
use std::str::{self, FromStr};

pub fn decode(mut bytes: &[u8], data: &Data) -> TokenStream {
    let ret = TokenStream::decode(&mut bytes, data);
    assert!(bytes.is_empty());
    return ret;
}

trait Decode {
    fn decode(bytes: &mut &[u8], data: &Data) -> Self;
}

fn byte(data: &mut &[u8]) -> u8 {
    let ret = data[0];
    *data = &data[1..];
    ret
}

fn str<'a>(bytes: &mut &'a [u8], data: &Data) -> &'a str {
    let len = u32::decode(bytes, data) as usize;
    let ret = str::from_utf8(&bytes[..len]).unwrap();
    *bytes = &bytes[len..];
    return ret;
}

impl Decode for TokenStream {
    fn decode(bytes: &mut &[u8], data: &Data) -> Self {
        let mut ret = TokenStream::new();
        loop {
            match byte(bytes) {
                0 => break,
                1 => ret.extend(Some(TokenTree::Group(Group::decode(bytes, data)))),
                2 => ret.extend(Some(TokenTree::Ident(Ident::decode(bytes, data)))),
                3 => ret.extend(Some(TokenTree::Punct(Punct::decode(bytes, data)))),
                _ => ret.extend(Some(TokenTree::Literal(Literal::decode(bytes, data)))),
            }
        }
        return ret;
    }
}

impl Decode for Group {
    fn decode(bytes: &mut &[u8], data: &Data) -> Self {
        let delimiter = Delimiter::decode(bytes, data);
        let span = Span::decode(bytes, data);
        let stream = TokenStream::decode(bytes, data);
        let mut ret = Group::new(delimiter, stream);
        ret.set_span(span);
        return ret;
    }
}

impl Decode for Delimiter {
    fn decode(bytes: &mut &[u8], _data: &Data) -> Self {
        match byte(bytes) {
            0 => Delimiter::Parenthesis,
            1 => Delimiter::Brace,
            2 => Delimiter::Bracket,
            _ => Delimiter::None,
        }
    }
}

impl Decode for Span {
    fn decode(bytes: &mut &[u8], data: &Data) -> Self {
        data.span[u32::decode(bytes, data)]
    }
}

impl Decode for u32 {
    fn decode(bytes: &mut &[u8], _data: &Data) -> Self {
        let ret = ((bytes[0] as u32) << 0)
            | ((bytes[1] as u32) << 8)
            | ((bytes[2] as u32) << 16)
            | ((bytes[3] as u32) << 24);
        *bytes = &bytes[4..];
        return ret;
    }
}

impl Decode for Ident {
    fn decode(bytes: &mut &[u8], data: &Data) -> Self {
        let span = Span::decode(bytes, data);
        let name = str(bytes, data);
        Ident::new(name, span)
    }
}

impl Decode for Punct {
    fn decode(bytes: &mut &[u8], data: &Data) -> Self {
        let mut p = Punct::new(
            char::from_u32(u32::decode(bytes, data)).unwrap(),
            Spacing::decode(bytes, data),
        );
        p.set_span(Span::decode(bytes, data));
        return p;
    }
}

impl Decode for Spacing {
    fn decode(bytes: &mut &[u8], _data: &Data) -> Self {
        match byte(bytes) {
            0 => Spacing::Alone,
            _ => Spacing::Joint,
        }
    }
}

impl Decode for Literal {
    fn decode(bytes: &mut &[u8], data: &Data) -> Self {
        let span = Span::decode(bytes, data);
        let text = str(bytes, data);
        let token = TokenStream::from_str(text)
            .unwrap()
            .into_iter()
            .next()
            .unwrap();
        let mut literal = match token {
            TokenTree::Literal(l) => l,
            _ => unreachable!(),
        };
        literal.set_span(span);
        return literal;
    }
}
