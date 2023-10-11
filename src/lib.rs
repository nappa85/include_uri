use proc_macro::TokenStream;
use proc_macro2::{Literal, Span};
use quote::quote;
use syn::{parse_macro_input, LitStr};

/// Includes a UTF-8 encoded file from an URI as a string.
///
/// This macro will yield an expression of type `&'static str` which is the
/// contents of the file.
///
/// # Examples
///
/// Assume there is a remote files reachable via URI http://www.example.com/spanish.in,
/// and it's contents are:
///
/// ```text
/// adiós
/// ```
///
/// File 'main.rs':
///
/// ```ignore (cannot-doctest-external-file-dependency)
/// fn main() {
///     let my_str = include_url::include_str_from_url!("http://www.example.com/spanish.in");
///     assert_eq!(my_str, "adiós\n");
///     print!("{my_str}");
/// }
/// ```
///
/// Compiling 'main.rs' and running the resulting binary will print "adiós".
#[proc_macro]
pub fn include_str_from_url(tokens: TokenStream) -> TokenStream {
    include_url(tokens, IncludeType::String)
}

/// Includes a file from an URI as a reference to a byte array.
///
/// This macro will yield an expression of type `&'static [u8; N]` which is
/// the contents of the file.
///
/// # Examples
///
/// Assume there is a remote files reachable via URI http://www.example.com/spanish.in,
/// and it's contents are:
///
/// ```text
/// adiós
/// ```
///
/// File 'main.rs':
///
/// ```ignore (cannot-doctest-external-file-dependency)
/// fn main() {
///     let bytes = include_url::include_bytes_from_url!("http://www.example.com/spanish.in");
///     assert_eq!(bytes, b"adi\xc3\xb3s\n");
///     print!("{}", String::from_utf8_lossy(bytes));
/// }
/// ```
///
/// Compiling 'main.rs' and running the resulting binary will print "adiós".
#[proc_macro]
pub fn include_bytes_from_url(tokens: TokenStream) -> TokenStream {
    include_url(tokens, IncludeType::Bytes)
}

enum IncludeType {
    String,
    Bytes,
}
fn include_url(tokens: TokenStream, include_type: IncludeType) -> TokenStream {
    let input = parse_macro_input!(tokens as LitStr);

    match include_url_inner(&input.value(), include_type) {
        Ok(lit) => quote! {
            #lit
        }
        .into(),
        Err(e) => syn::Error::new(Span::call_site(), e)
            .into_compile_error()
            .into(),
    }
}

fn include_url_inner(url: &str, include_type: IncludeType) -> reqwest::Result<Literal> {
    let res = reqwest::blocking::get(url)?;
    match include_type {
        IncludeType::String => {
            let text = res.text()?;
            Ok(Literal::string(&text))
        }
        IncludeType::Bytes => {
            let bytes = res.bytes()?;
            Ok(Literal::byte_string(&bytes))
        }
    }
}
