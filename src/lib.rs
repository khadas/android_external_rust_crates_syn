#![doc(html_root_url = "https://dtolnay.github.io/syn")]
#![cfg_attr(feature = "cargo-clippy",
            allow(const_static_lifetime, doc_markdown, large_enum_variant, match_bool,
                  redundant_closure))]

extern crate proc_macro2;
extern crate proc_macro;
extern crate unicode_xid;

#[cfg(feature = "printing")]
extern crate quote;

#[cfg(feature = "parsing")]
#[macro_use]
#[doc(hidden)]
pub mod parsers;

#[macro_use]
mod macros;

#[macro_use]
pub mod token;

#[cfg(any(feature = "full", feature = "derive"))]
mod attr;
#[cfg(any(feature = "full", feature = "derive"))]
pub use attr::{AttrStyle, Attribute, MetaItem, MetaItemList, MetaNameValue, NestedMetaItem};

#[cfg(any(feature = "full", feature = "derive"))]
mod data;
#[cfg(any(feature = "full", feature = "derive"))]
pub use data::{Field, Fields, FieldsNamed, FieldsUnnamed, Variant, VisCrate, VisPublic, VisRestricted, Visibility};

#[cfg(any(feature = "full", feature = "derive"))]
mod expr;
#[cfg(any(feature = "full", feature = "derive"))]
pub use expr::{Expr, ExprAddrOf, ExprArray, ExprAssign, ExprAssignOp, ExprBinary, ExprBlock,
               ExprBox, ExprBreak, ExprCall, ExprCast, ExprCatch, ExprClosure, ExprContinue,
               ExprField, ExprForLoop, ExprGroup, ExprIf, ExprIfLet, ExprInPlace, ExprIndex,
               ExprLit, ExprLoop, ExprMacro, ExprMatch, ExprMethodCall, ExprParen, ExprPath, ExprRange,
               ExprRepeat, ExprReturn, ExprStruct, ExprTry, ExprTuple, ExprType,
               ExprUnary, ExprUnsafe, ExprVerbatim, ExprWhile, ExprWhileLet, ExprYield, Index, Member};

#[cfg(feature = "full")]
pub use expr::{Arm, Block, FieldPat, FieldValue, GenericMethodArgument, Label, Local,
               MethodTurbofish, Pat, PatBox, PatIdent, PatLit, PatMacro, PatPath, PatRange, PatRef, PatSlice,
               PatStruct, PatTuple, PatTupleStruct, PatVerbatim, PatWild, RangeLimits, Stmt};

#[cfg(any(feature = "full", feature = "derive"))]
mod generics;
#[cfg(any(feature = "full", feature = "derive"))]
pub use generics::{BoundLifetimes, ConstParam, GenericParam, Generics, LifetimeDef,
                   PredicateEq, PredicateLifetime, PredicateType, TraitBound, TraitBoundModifier,
                   TypeParam, TypeParamBound, WhereClause, WherePredicate};
#[cfg(all(any(feature = "full", feature = "derive"), feature = "printing"))]
pub use generics::{ImplGenerics, Turbofish, TypeGenerics};

mod ident;
pub use ident::Ident;

#[cfg(feature = "full")]
mod item;
#[cfg(feature = "full")]
pub use item::{ArgCaptured, ArgSelf, ArgSelfRef, FnArg, FnDecl,
               ForeignItem, ForeignItemFn, ForeignItemStatic, ForeignItemType, ForeignItemVerbatim, ImplItem,
               ImplItemConst, ImplItemMacro, ImplItemMethod, ImplItemType, ImplItemVerbatim, Item,
               ItemConst, ItemEnum, ItemExternCrate, ItemFn, ItemForeignMod,
               ItemImpl, ItemMacro, ItemMacro2, ItemMod, ItemStatic, ItemStruct, ItemTrait,
               ItemType, ItemUnion, ItemUse, ItemVerbatim, MethodSig, TraitItem, TraitItemConst, TraitItemMacro,
               TraitItemMethod, TraitItemType, TraitItemVerbatim, UseGlob, UseList, UsePath, UseTree};

#[cfg(feature = "full")]
mod file;
#[cfg(feature = "full")]
pub use file::File;

#[cfg(any(feature = "full", feature = "derive"))]
mod lifetime;
#[cfg(any(feature = "full", feature = "derive"))]
pub use lifetime::Lifetime;

#[cfg(any(feature = "full", feature = "derive"))]
mod lit;
#[cfg(any(feature = "full", feature = "derive"))]
pub use lit::{Lit, LitStr, LitByteStr, LitByte, LitChar, LitInt, LitFloat, LitBool, LitVerbatim,
              StrStyle, IntSuffix, FloatSuffix};

#[cfg(any(feature = "full", feature = "derive"))]
mod mac;
#[cfg(any(feature = "full", feature = "derive"))]
pub use mac::{Macro, MacroDelimiter};

#[cfg(any(feature = "full", feature = "derive"))]
mod derive;
#[cfg(feature = "derive")]
pub use derive::{Data, DataEnum, DataStruct, DataUnion, DeriveInput};

#[cfg(any(feature = "full", feature = "derive"))]
mod op;
#[cfg(any(feature = "full", feature = "derive"))]
pub use op::{BinOp, UnOp};

#[cfg(any(feature = "full", feature = "derive"))]
mod ty;
#[cfg(any(feature = "full", feature = "derive"))]
pub use ty::{Abi, BareFnArg, BareFnArgName, ReturnType, Type, TypeArray,
             TypeBareFn, TypeGroup, TypeImplTrait, TypeInfer, TypeMacro, TypeNever, TypeParen,
             TypePath, TypePtr, TypeReference, TypeSlice, TypeTraitObject, TypeTuple, TypeVerbatim};

#[cfg(any(feature = "full", feature = "derive"))]
mod path;
#[cfg(any(feature = "full", feature = "derive"))]
pub use path::{Path, PathSegment, PathArguments, GenericArgument, AngleBracketedGenericArguments,
               Binding, ParenthesizedGenericArguments, QSelf};
#[cfg(all(any(feature = "full", feature = "derive"), feature = "printing"))]
pub use path::PathTokens;

#[cfg(feature = "parsing")]
mod cursor;
#[cfg(feature = "parsing")]
pub mod synom;
pub mod punctuated;
#[cfg(all(any(feature = "full", feature = "derive"), feature = "parsing"))]
mod tt;

#[cfg(all(feature = "parsing", feature = "printing"))]
pub mod spanned;

mod gen {
    #[cfg(feature = "visit")]
    pub mod visit;

    #[cfg(feature = "visit_mut")]
    pub mod visit_mut;

    #[cfg(feature = "fold")]
    pub mod fold;

    #[cfg(any(feature = "full", feature = "derive"))]
    #[path = "../gen_helper.rs"]
    mod helper;
}
pub use gen::*;

////////////////////////////////////////////////////////////////////////////////

#[cfg(feature = "parsing")]
use synom::Synom;
#[cfg(feature = "parsing")]
use cursor::SynomBuffer;

#[cfg(feature = "parsing")]
mod error;
#[cfg(feature = "parsing")]
use error::ParseError;

// Not public API.
#[cfg(feature = "parsing")]
#[doc(hidden)]
pub use error::parse_error;

/// Parse tokens of source code into the chosen syn data type.
///
/// This is preferred over parsing a string because tokens are able to preserve
/// information about where in the user's code they were originally written (the
/// "span" of the token), possibly allowing the compiler to produce better error
/// messages.
///
/// # Examples
///
/// ```rust
/// extern crate proc_macro;
/// use proc_macro::TokenStream;
///
/// extern crate syn;
///
/// #[macro_use]
/// extern crate quote;
///
/// use syn::DeriveInput;
///
/// # const IGNORE_TOKENS: &str = stringify! {
/// #[proc_macro_derive(MyMacro)]
/// # };
/// pub fn my_macro(input: TokenStream) -> TokenStream {
///     // Parse the tokens into a syntax tree
///     let ast: DeriveInput = syn::parse(input).unwrap();
///
///     // Build the output, possibly using quasi-quotation
///     let expanded = quote! {
///         /* ... */
///     };
///
///     // Convert into a token stream and return it
///     expanded.into()
/// }
/// #
/// # fn main() {}
/// ```
#[cfg(feature = "parsing")]
pub fn parse<T>(tokens: proc_macro::TokenStream) -> Result<T, ParseError>
where
    T: Synom,
{
    _parse(tokens.into())
}

#[cfg(feature = "parsing")]
fn _parse<T>(tokens: proc_macro2::TokenStream) -> Result<T, ParseError>
where
    T: Synom,
{
    let buf = SynomBuffer::new(tokens);
    let result = T::parse(buf.begin());
    let err = match result {
        Ok((t, rest)) => {
            if rest.eof() {
                return Ok(t);
            } else if rest == buf.begin() {
                // parsed nothing
                ParseError::new("failed to parse anything")
            } else {
                ParseError::new("failed to parse all tokens")
            }
        }
        Err(err) => err,
    };
    match T::description() {
        Some(s) => Err(ParseError::new(format!("failed to parse {}: {}", s, err))),
        None => Err(err),
    }
}

/// Parse a string of Rust code into the chosen syn data type.
///
/// # Examples
///
/// ```rust
/// extern crate syn;
/// #
/// #
/// # type Result<T> = std::result::Result<T, Box<std::error::Error>>;
///
/// use syn::Expr;
///
/// fn run() -> Result<()> {
///     let code = "assert_eq!(u8::max_value(), 255)";
///     let expr = syn::parse_str::<Expr>(code)?;
///     println!("{:#?}", expr);
///     Ok(())
/// }
/// #
/// # fn main() { run().unwrap() }
/// ```
#[cfg(feature = "parsing")]
pub fn parse_str<T: Synom>(s: &str) -> Result<T, ParseError> {
    match s.parse() {
        Ok(tts) => _parse(tts),
        Err(_) => Err(ParseError::new("error while lexing input string")),
    }
}

// FIXME the name parse_file makes it sound like you might pass in a path to a
// file, rather than the content.
/// Parse the content of a file of Rust code.
///
/// This is different from `syn::parse_str::<File>(content)` in two ways:
///
/// - It discards a leading byte order mark `\u{FEFF}` if the file has one.
/// - It preserves the shebang line of the file, such as `#!/usr/bin/env rustx`.
///
/// If present, either of these would be an error using `from_str`.
///
/// # Examples
///
/// ```rust,no_run
/// extern crate syn;
/// #
/// #
/// # type Result<T> = std::result::Result<T, Box<std::error::Error>>;
///
/// use std::fs::File;
/// use std::io::Read;
///
/// fn run() -> Result<()> {
///     let mut file = File::open("path/to/code.rs")?;
///     let mut content = String::new();
///     file.read_to_string(&mut content)?;
///
///     let ast = syn::parse_file(&content)?;
///     if let Some(shebang) = ast.shebang {
///         println!("{}", shebang);
///     }
///     println!("{} items", ast.items.len());
///
///     Ok(())
/// }
/// #
/// # fn main() { run().unwrap() }
/// ```
#[cfg(all(feature = "parsing", feature = "full"))]
pub fn parse_file(mut content: &str) -> Result<File, ParseError> {
    // Strip the BOM if it is present
    const BOM: &'static str = "\u{feff}";
    if content.starts_with(BOM) {
        content = &content[BOM.len()..];
    }

    let mut shebang = None;
    if content.starts_with("#!") && !content.starts_with("#![") {
        if let Some(idx) = content.find('\n') {
            shebang = Some(content[..idx].to_string());
            content = &content[idx..];
        } else {
            shebang = Some(content.to_string());
            content = "";
        }
    }

    let mut file: File = parse_str(content)?;
    file.shebang = shebang;
    Ok(file)
}

#[cfg(all(feature = "parsing", feature = "printing"))]
#[macro_export]
macro_rules! parse_quote {
    ($($tt:tt)*) => {
        ::std::result::Result::unwrap(
            $crate::parse(
                ::std::convert::Into::into(
                    quote!($($tt)*))))
    };
}

#[cfg(all(any(feature = "full", feature = "derive"), feature = "printing"))]
struct TokensOrDefault<'a, T: 'a>(&'a Option<T>);

#[cfg(all(any(feature = "full", feature = "derive"), feature = "printing"))]
impl<'a, T> quote::ToTokens for TokensOrDefault<'a, T>
where
    T: quote::ToTokens + Default,
{
    fn to_tokens(&self, tokens: &mut quote::Tokens) {
        match *self.0 {
            Some(ref t) => t.to_tokens(tokens),
            None => T::default().to_tokens(tokens),
        }
    }
}
