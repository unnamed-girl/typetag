use proc_macro2::TokenStream;

use crate::RegisterArgs;
use quote::quote;

pub(crate) fn expand(args: RegisterArgs) -> TokenStream {
    let trait_ty = args.trait_ty;
    let impl_ty = args.impl_ty;

    let mut expanded = TokenStream::new();

    expanded.extend(quote! {
        typetag::__private::inventory::submit! {
            <dyn #trait_ty>::typetag_register(
                <#impl_ty as typetag::TypetagName>::typetag_name(),
                (|deserializer| typetag::__private::Result::Ok(
                    typetag::__private::Box::new(
                        typetag::__private::erased_serde::deserialize::<#impl_ty>(deserializer)?
                    ),
                )) as typetag::__private::DeserializeFn<dyn #trait_ty>,
            )
        }
    });

    expanded
}
