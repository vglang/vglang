use proc_macro::TokenStream;
use syn::{parse_macro_input, Item};

mod item_enum;
mod item_struct;

pub fn derive_api(item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as Item);

    match item {
        Item::Enum(item) => item_enum::derive_api(item).into(),
        Item::Struct(item) => item_struct::drive_struct(item).into(),
        _ => unimplemented!("unsupport item."),
    }
}
