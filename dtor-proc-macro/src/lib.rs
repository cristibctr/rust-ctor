use std::iter::FromIterator;

use proc_macro::{Delimiter, Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree};

#[proc_macro_attribute]
pub fn dtor(attribute: TokenStream, item: TokenStream) -> TokenStream {
    generate("dtor", attribute, item)
}

/// Generates the equivalent of this Rust code as a TokenStream:
///
/// ```nocompile
/// ::ctor::__support::ctor_parse!(#[ctor] fn foo() { ... });
/// ::dtor::__support::dtor_parse!(#[dtor] fn foo() { ... });
/// ```
fn generate(macro_type: &str, attribute: TokenStream, item: TokenStream) -> TokenStream {
    let mut inner = TokenStream::new();

    #[cfg(feature = "used_linker")]
    inner.extend([
        TokenTree::Punct(Punct::new('#', Spacing::Alone)),
        TokenTree::Group(Group::new(
            Delimiter::Bracket,
            TokenStream::from_iter([
                TokenTree::Ident(Ident::new("feature", Span::call_site())),
                TokenTree::Group(Group::new(
                    Delimiter::Parenthesis,
                    TokenStream::from_iter([TokenTree::Ident(Ident::new(
                        "used_linker",
                        Span::call_site(),
                    ))]),
                )),
            ]),
        )),
    ]);

    #[cfg(feature = "__warn_on_missing_unsafe")]
    inner.extend([
        TokenTree::Punct(Punct::new('#', Spacing::Alone)),
        TokenTree::Group(Group::new(
            Delimiter::Bracket,
            TokenStream::from_iter([
                TokenTree::Ident(Ident::new("feature", Span::call_site())),
                TokenTree::Group(Group::new(
                    Delimiter::Parenthesis,
                    TokenStream::from_iter([TokenTree::Ident(Ident::new(
                        "__warn_on_missing_unsafe",
                        Span::call_site(),
                    ))]),
                )),
            ]),
        )),
    ]);

    if attribute.is_empty() {
        // #[ctor]
        inner.extend([
            TokenTree::Punct(Punct::new('#', Spacing::Alone)),
            TokenTree::Group(Group::new(
                Delimiter::Bracket,
                TokenStream::from_iter([TokenTree::Ident(Ident::new(
                    macro_type,
                    Span::call_site(),
                ))]),
            )),
        ]);
    } else {
        inner.extend([
            TokenTree::Punct(Punct::new('#', Spacing::Alone)),
            TokenTree::Group(Group::new(
                Delimiter::Bracket,
                TokenStream::from_iter([
                    TokenTree::Ident(Ident::new(macro_type, Span::call_site())),
                    TokenTree::Group(Group::new(Delimiter::Parenthesis, attribute)),
                ]),
            )),
        ]);
    }

    inner.extend(item);

    TokenStream::from_iter([
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new(macro_type, Span::call_site())),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new("__support", Span::call_site())),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new(
            &format!("{}_parse", macro_type),
            Span::call_site(),
        )),
        TokenTree::Punct(Punct::new('!', Spacing::Alone)),
        TokenTree::Group(Group::new(Delimiter::Parenthesis, inner)),
        TokenTree::Punct(Punct::new(';', Spacing::Alone)),
    ])
}
