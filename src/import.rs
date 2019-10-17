use crate::sym;
use watt_jit::*;

pub fn extern_vals(module: &Module, store: &Store) -> InstanceImports {
    let mut imports = InstanceImports::default();
    for import in module.imports().iter() {
        imports.func(resolve(import, store));
    }
    return imports;
}

fn resolve(import: &ImportType, store: &Store) -> Func {
    assert_eq!(import.module(), "env");
    // TODO: assert `import` is a function import
    // let (_module, name, sig) = import;

    match import.name() {
        "token_stream_new" => Func::new0(store, sym::token_stream_new),
        "token_stream_is_empty" => Func::new1(store, sym::token_stream_is_empty),
        "token_stream_from_str" => Func::new1(store, sym::token_stream_from_str),
        "token_stream_into_iter" => Func::new1(store, sym::token_stream_into_iter),
        "token_stream_iter_next" => Func::new1(store, sym::token_stream_iter_next),
        "token_stream_from_group" => Func::new1(store, sym::token_stream_from_group),
        "token_stream_from_ident" => Func::new1(store, sym::token_stream_from_ident),
        "token_stream_from_punct" => Func::new1(store, sym::token_stream_from_punct),
        "token_stream_from_literal" => Func::new1(store, sym::token_stream_from_literal),
        "token_stream_push_group" => Func::new2(store, sym::token_stream_push_group),
        "token_stream_push_ident" => Func::new2(store, sym::token_stream_push_ident),
        "token_stream_push_punct" => Func::new2(store, sym::token_stream_push_punct),
        "token_stream_push_literal" => Func::new2(store, sym::token_stream_push_literal),
        "token_stream_extend" => Func::new2(store, sym::token_stream_extend),

        "token_tree_kind" => Func::new1(store, sym::token_tree_kind),
        "token_tree_unwrap_group" => Func::new1(store, sym::token_tree_unwrap_group),
        "token_tree_unwrap_ident" => Func::new1(store, sym::token_tree_unwrap_ident),
        "token_tree_unwrap_punct" => Func::new1(store, sym::token_tree_unwrap_punct),
        "token_tree_unwrap_literal" => Func::new1(store, sym::token_tree_unwrap_literal),

        "span_call_site" => Func::new0(store, sym::span_call_site),

        "group_new" => Func::new2(store, sym::group_new),
        "group_delimiter" => Func::new1(store, sym::group_delimiter),
        "group_stream" => Func::new1(store, sym::group_stream),
        "group_span" => Func::new1(store, sym::group_span),
        "group_set_span" => Func::new2(store, sym::group_set_span),

        "punct_new" => Func::new2(store, sym::punct_new),
        "punct_as_char" => Func::new1(store, sym::punct_as_char),
        "punct_spacing" => Func::new1(store, sym::punct_spacing),
        "punct_span" => Func::new1(store, sym::punct_span),
        "punct_set_span" => Func::new2(store, sym::punct_set_span),

        "ident_new" => Func::new2(store, sym::ident_new),
        "ident_span" => Func::new1(store, sym::ident_span),
        "ident_set_span" => Func::new2(store, sym::ident_set_span),
        "ident_eq" => Func::new2(store, sym::ident_eq),
        "ident_eq_str" => Func::new2(store, sym::ident_eq_str),
        "ident_cmp" => Func::new2(store, sym::ident_cmp),

        "literal_u8_suffixed" => Func::new1(store, sym::literal_u8_suffixed),
        "literal_u16_suffixed" => Func::new1(store, sym::literal_u16_suffixed),
        "literal_u32_suffixed" => Func::new1(store, sym::literal_u32_suffixed),
        "literal_u64_suffixed" => Func::new1(store, sym::literal_u64_suffixed),
        "literal_u128_suffixed" => Func::new2(store, sym::literal_u128_suffixed),
        "literal_usize_suffixed" => Func::new1(store, sym::literal_usize_suffixed),
        "literal_i8_suffixed" => Func::new1(store, sym::literal_i8_suffixed),
        "literal_i16_suffixed" => Func::new1(store, sym::literal_i16_suffixed),
        "literal_i32_suffixed" => Func::new1(store, sym::literal_i32_suffixed),
        "literal_i64_suffixed" => Func::new1(store, sym::literal_i64_suffixed),
        "literal_i128_suffixed" => Func::new2(store, sym::literal_i128_suffixed),
        "literal_isize_suffixed" => Func::new1(store, sym::literal_isize_suffixed),
        "literal_u8_unsuffixed" => Func::new1(store, sym::literal_u8_unsuffixed),
        "literal_u16_unsuffixed" => Func::new1(store, sym::literal_u16_unsuffixed),
        "literal_u32_unsuffixed" => Func::new1(store, sym::literal_u32_unsuffixed),
        "literal_u64_unsuffixed" => Func::new1(store, sym::literal_u64_unsuffixed),
        "literal_u128_unsuffixed" => Func::new2(store, sym::literal_u128_unsuffixed),
        "literal_usize_unsuffixed" => Func::new1(store, sym::literal_usize_unsuffixed),
        "literal_i8_unsuffixed" => Func::new1(store, sym::literal_i8_unsuffixed),
        "literal_i16_unsuffixed" => Func::new1(store, sym::literal_i16_unsuffixed),
        "literal_i32_unsuffixed" => Func::new1(store, sym::literal_i32_unsuffixed),
        "literal_i64_unsuffixed" => Func::new1(store, sym::literal_i64_unsuffixed),
        "literal_i128_unsuffixed" => Func::new2(store, sym::literal_i128_unsuffixed),
        "literal_isize_unsuffixed" => Func::new1(store, sym::literal_isize_unsuffixed),
        "literal_f64_unsuffixed" => Func::new1(store, sym::literal_f64_unsuffixed),
        "literal_f64_suffixed" => Func::new1(store, sym::literal_f64_suffixed),
        "literal_f32_unsuffixed" => Func::new1(store, sym::literal_f32_unsuffixed),
        "literal_f32_suffixed" => Func::new1(store, sym::literal_f32_suffixed),
        "literal_string" => Func::new1(store, sym::literal_string),
        "literal_character" => Func::new1(store, sym::literal_character),
        "literal_byte_string" => Func::new1(store, sym::literal_byte_string),
        "literal_span" => Func::new1(store, sym::literal_span),
        "literal_set_span" => Func::new2(store, sym::literal_set_span),

        "token_stream_clone" => Func::new1(store, sym::token_stream_clone),
        "group_clone" => Func::new1(store, sym::group_clone),
        "ident_clone" => Func::new1(store, sym::ident_clone),
        "punct_clone" => Func::new1(store, sym::punct_clone),
        "literal_clone" => Func::new1(store, sym::literal_clone),
        "token_stream_iter_clone" => Func::new1(store, sym::token_stream_iter_clone),

        "token_stream_to_string" => Func::new1(store, sym::token_stream_to_string),
        "group_to_string" => Func::new1(store, sym::group_to_string),
        "ident_to_string" => Func::new1(store, sym::ident_to_string),
        "punct_to_string" => Func::new1(store, sym::punct_to_string),
        "literal_to_string" => Func::new1(store, sym::literal_to_string),
        "token_stream_debug" => Func::new1(store, sym::token_stream_debug),
        "group_debug" => Func::new1(store, sym::group_debug),
        "ident_debug" => Func::new1(store, sym::ident_debug),
        "punct_debug" => Func::new1(store, sym::punct_debug),
        "literal_debug" => Func::new1(store, sym::literal_debug),
        "span_debug" => Func::new1(store, sym::span_debug),

        "watt_string_with_capacity" => Func::new1(store, sym::watt_string_with_capacity),
        "watt_string_push_char" => Func::new2(store, sym::watt_string_push_char),
        "watt_string_len" => Func::new1(store, sym::watt_string_len),
        "watt_string_char_at" => Func::new2(store, sym::watt_string_char_at),
        "watt_bytes_with_capacity" => Func::new1(store, sym::watt_bytes_with_capacity),
        "watt_bytes_push" => Func::new2(store, sym::watt_bytes_push),
        "watt_print_panic" => Func::new1(store, sym::watt_print_panic),

        name => unreachable!("unresolved import: {:?}", name),
    }
}
