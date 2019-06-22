extern crate proc_macro;
use proc_macro::TokenStream;
use syn;

#[macro_use]
extern crate quote;

#[proc_macro_attribute]
pub fn v8_fn(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(item as syn::ItemFn);
    let name = ast.ident;
    let inputs = ast.decl.inputs;
    let block = ast.block;
    let vis = ast.vis;

    (quote! {
        #vis extern "C" fn #name(args: *const osgood_v8::V8::FunctionCallbackInfo) {
            let args = osgood_v8::wrapper::FunctionCallbackInfo::new(args);
            handle_scope!({
                (|#inputs|#block)(args);
            });
        }
    })
    .into()
}
