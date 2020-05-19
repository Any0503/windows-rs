use crate::types::RequiredInterface;
use crate::types::TypeName;
use proc_macro2::TokenStream;
use quote::quote;

pub fn debug_tokens(name: &TypeName, interfaces: &Vec<RequiredInterface>) -> TokenStream {
    let constraints = name.constraints();
    let clean_name = name.clean_name();
    let name = name.to_tokens(&name.namespace);

    let default_impl = quote! { ::std::format!("{}({:?})", #clean_name, <Self as ::winrt::RuntimeType>::abi(self)) };

    let implements_istringable = interfaces.iter().any(|interface| {
        interface.name.name == "IStringable" && interface.name.namespace == "Windows.Foundation"
    });
    let is_istringable = &interfaces.get(0).map(|i| i.name.name.as_str()) == &Some("IStringable")
        && &interfaces[0].name.namespace == "Windows.Foundation";

    let implementation = if implements_istringable && !is_istringable {
        quote! {
            "{:?}",
            {
                let s: crate::windows::foundation::IStringable = self.into();
                s
            }
        }
    } else if is_istringable {
        quote! {
            "{}",
            match self.to_string(){
                Ok(s) => s.to_string(),
                Err(_) => #default_impl
            }
        }
    } else {
        quote! {
            "{}",
            {
                // TODO: assume IStringable
                // let s: ::winrt::Result<IStringable> = <self as ::winrt::TryInto>::try_into();
                // match s {
                //     ::std::result::Result::Ok(s) => format!("{}", s),
                //     ::std::result::Result::Err(_) => #default_impl
                // }
                #default_impl
            }

        }
    };
    quote! {
        impl<#constraints> ::std::fmt::Debug for #name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(
                    f,
                    #implementation
                )
            }
        }
    }
}
