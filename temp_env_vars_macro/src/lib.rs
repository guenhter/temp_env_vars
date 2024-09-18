extern crate proc_macro;
use quote::quote;

#[proc_macro_attribute]
pub fn temp_env_vars(
    _: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item_fn: Result<syn::ItemFn, _> = syn::parse(item.clone());
    let item_fn = match item_fn {
        Ok(item_fn) => item_fn,
        _ => return item,
    };
    let attrs: Vec<syn::Attribute> = item_fn.attrs.into_iter().collect();
    let vis = item_fn.vis;
    let name = item_fn.sig.ident;

    let return_type = match item_fn.sig.output {
        syn::ReturnType::Default => None,
        syn::ReturnType::Type(_, ref box_type) => Some(box_type.clone()),
    };
    let returning = if let Some(ret) = return_type {
        quote! { -> #ret }
    } else {
        quote! {}
    };

    let asynciness = if item_fn.sig.asyncness.is_some() {
        quote! { async }
    } else {
        quote! {}
    };
    let block = item_fn.block;

    let gen = quote! {
        #(#attrs)
        *
        #vis #asynciness fn #name () #returning {
            let _temp_env_vars_scope_lock = temp_env_vars::TEMP_ENV_VAR_MACRO_MUTEX.lock();
            let _temp_env_vars_scope = temp_env_vars::TempEnvScope::new();
            #block
        }
    };

    // The "#[serial]" macro should be applied before the "#[temp_env_vars]" macro
    if gen.to_string().contains("serial_test ::") {
        panic!("Apply the '#[serial]' after the '#[temp_env_vars]' macro");
    }

    #[cfg(all(feature = "debug_temp_env_vars", not(test)))]
    {
        std::fs::write("target/temp_env_vars_debug.rs", gen.to_string()).unwrap();
        std::process::Command::new("rustfmt")
            .arg("target/temp_env_vars_debug.rs")
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }

    gen.into()
}
