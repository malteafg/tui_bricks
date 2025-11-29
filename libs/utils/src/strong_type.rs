pub use bincode::{Decode, Encode};
pub use paste::paste;
use serde::{Deserialize, Serialize};

use std::marker::PhantomData;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Serialize, Decode, Encode)]
#[serde(transparent)]
pub struct Strong<T, Tag> {
    internal: T,
    _marker: PhantomData<Tag>,
}

impl<T, Tag> Strong<T, Tag> {
    pub fn new(internal: T) -> Self {
        Self {
            internal,
            _marker: PhantomData::<Tag>,
        }
    }

    pub fn into_inner(self) -> T {
        self.internal
    }
}

impl<T, Tag> Clone for Strong<T, Tag>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self::new(self.internal.clone())
    }
}

impl<T, Tag> Copy for Strong<T, Tag> where T: Copy {}

impl<T, Tag> From<T> for Strong<T, Tag> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<Tag> From<&str> for Strong<String, Tag> {
    fn from(value: &str) -> Self {
        Self::new(value.into())
    }
}

impl<Tag> From<&Path> for Strong<PathBuf, Tag> {
    fn from(value: &Path) -> Self {
        Self::new(value.into())
    }
}

impl<T: std::fmt::Display, Tag> std::fmt::Display for Strong<T, Tag> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.internal)
    }
}

impl<T: std::fmt::Debug, Tag> std::fmt::Debug for Strong<T, Tag> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(std::any::type_name::<Strong<T, Tag>>())
            .field(&self.internal)
            .finish()
    }
}

impl<T: PartialEq, Tag> PartialEq for Strong<T, Tag> {
    fn eq(&self, other: &Self) -> bool {
        self.internal.eq(&other.internal)
    }
}

impl<T: Eq, Tag> Eq for Strong<T, Tag> {}

impl<T: std::hash::Hash, Tag> std::hash::Hash for Strong<T, Tag> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.internal.hash(state)
    }
}

impl<T, Tag> std::ops::Deref for Strong<T, Tag> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}

impl<T, Tag> std::ops::DerefMut for Strong<T, Tag> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.internal
    }
}

impl<T, Tag> std::convert::AsRef<T> for Strong<T, Tag> {
    fn as_ref(&self) -> &T {
        &self.internal
    }
}

impl<T, Tag> std::convert::AsMut<T> for Strong<T, Tag> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.internal
    }
}

#[macro_export]
macro_rules! strong_type {
    ($name:ident, $inner:ty) => {
        $crate::strong_type::paste! {
            #[derive(Encode, Decode)]
            pub struct [<$name Tag>];
            pub type $name = Strong<$inner, [<$name Tag>]>;
        }
    };
}

// use proc_macro::TokenStream;
// use quote::{quote, format_ident};
// use syn::{parse_macro_input, DeriveInput};

// #[proc_macro_derive(StrongType)]
// pub fn strong_type_derive(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);

//     let name = &input.ident;

//     // Ensure it's a tuple struct with exactly one field
//     let inner_type = if let syn::Data::Struct(data_struct) = &input.data {
//         if data_struct.fields.len() == 1 {
//             &data_struct.fields.iter().next().unwrap().ty
//         } else {
//             panic!("#[derive(StrongType)] only supports tuple structs with one field");
//         }
//     } else {
//         panic!("#[derive(StrongType)] only supports tuple structs");
//     };

//     // Check if inner is String (special From<&str>)
//     let from_str_impl = quote! {
//         impl From<&str> for #name
//         where
//             #inner_type: From<String>,
//         {
//             fn from(s: &str) -> Self {
//                 #name(String::from(s))
//             }
//         }
//     };

//     // Check if inner is Vec<T> (special From<&[T]>)
//     let from_slice_impl = if let syn::Type::Path(type_path) = inner_type {
//         if type_path.path.segments.last().unwrap().ident == "Vec" {
//             let generic = match &type_path.path.segments.last().unwrap().arguments {
//                 syn::PathArguments::AngleBracketed(args) => args.args.first().unwrap(),
//                 _ => panic!("Vec must have one generic argument"),
//             };
//             quote! {
//                 impl<T: Clone> From<&[T]> for #name
//                 where
//                     #inner_type: From<Vec<T>>,
//                 {
//                     fn from(slice: &[T]) -> Self {
//                         #name(slice.to_vec())
//                     }
//                 }
//             }
//         } else {
//             quote! {}
//         }
//     } else {
//         quote! {}
//     };

//     let expanded = quote! {
//         // Deref
//         impl std::ops::Deref for #name {
//             type Target = #inner_type;
//             fn deref(&self) -> &Self::Target { &self.0 }
//         }

//         // DerefMut
//         impl std::ops::DerefMut for #name
//         where
//             #inner_type: std::ops::DerefMut,
//         {
//             fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
//         }

//         // From<U> where inner implements From<U>
//         impl<U> From<U> for #name
//         where
//             #inner_type: From<U>,
//         {
//             fn from(value: U) -> Self {
//                 #name(#inner_type::from(value))
//             }
//         }

//         // Conditional traits
//         impl std::hash::Hash for #name
//         where
//             #inner_type: std::hash::Hash
//         {
//             fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//                 self.0.hash(state)
//             }
//         }

//         impl Copy for #name where #inner_type: Copy {}
//         impl Clone for #name where #inner_type: Clone {
//             fn clone(&self) -> Self { #name(self.0.clone()) }
//         }

//         impl PartialOrd for #name where #inner_type: PartialOrd {
//             fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//                 self.0.partial_cmp(&other.0)
//             }
//         }

//         impl Ord for #name where #inner_type: Ord {
//             fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//                 self.0.cmp(&other.0)
//             }
//         }

//         // Special From impls
//         #from_str_impl
//         #from_slice_impl
//     };

//     TokenStream::from(expanded)
// }
