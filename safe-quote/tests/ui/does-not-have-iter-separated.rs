use safe_quote::quote;

fn main() {
    quote!(#(a b),*);
}
