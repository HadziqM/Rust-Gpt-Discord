use proc_macro::TokenStream;
use quote::{quote,format_ident};
use syn::{parse_macro_input, ItemFn, AttributeArgs, NestedMeta, LitBool, LitInt};

fn get_bool<'a>(inp:&'a NestedMeta)->&'a LitBool{
    match inp{
        syn::NestedMeta::Lit(x) => match x {
            syn::Lit::Bool(y)=>y,
            _=>panic!("doesnt see bool")
        }
        _=>panic!("doesnt see bool")
    }
}
fn get_cooldown<'a>(inp:&'a NestedMeta)->&'a LitInt{
    match inp{
        syn::NestedMeta::Lit(x) => match x {
            syn::Lit::Int(y)=>y,
            _=>panic!("doesnt see int")
        }
        _=>panic!("doesnt see int")
    }
}


#[proc_macro_attribute]
pub fn test(_args: TokenStream, input: TokenStream) -> TokenStream {
    let idk = parse_macro_input!(input as ItemFn);
    let qt = quote!{
        #idk
    };
    qt.into()
}
#[proc_macro_attribute]
pub fn slash(args: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as ItemFn);
    let args = parse_macro_input!(args as AttributeArgs);
    let mut iter = args.iter();
    let cooldown = get_cooldown(iter.next().unwrap());
    let defer = get_bool(iter.next().unwrap());
    let fname = &item.sig.ident;
    let new_name = format_ident!("discord_{}",fname.to_string());
    let quete = quote!{
        #item
        pub async fn #new_name(bnd:&SlashBundle<'_>){
            let cmd = bnd.cmd;
            let on = bnd.cmd.data.name.to_owned();
            let cd = #cooldown;
            let defer = #defer;
            let mut err = crate::ErrorLog::from_bnd(bnd).await;
            if let Err(why)=bnd.cd_check(cd).await{
                return why.log(cmd, &on, false, &mut err).await;
            }
            if defer{
                cmd.defer_res(&mut err, &on,false).await;
            }
            match #fname(bnd).await{
                Err(why)=>{
                    match !defer{
                        true=>why.log(cmd, &on, false, &mut err).await,
                        false=>why.log_defer(cmd, &on, &mut err).await,
                    };
                }
                Ok(_)=>{
                    bnd.cooldown(cd).await
                }
            }
        }
    };
    quete.into()
}
#[proc_macro_attribute]
pub fn modal(args: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as ItemFn);
    let args = parse_macro_input!(args as AttributeArgs);
    let mut iter = args.iter();
    let cooldown = get_cooldown(iter.next().unwrap());
    let defer = get_bool(iter.next().unwrap());
    let fname = &item.sig.ident;
    let new_name = format_ident!("discord_{}",fname.to_string());
    let quete = quote!{
        #item
        pub async fn #new_name(bnd:&ModalBundle<'_>){
            let cmd = bnd.cmd();
            let on = bnd.name();
            let cd = #cooldown;
            let defer = #defer;
            let mut err = crate::ErrorLog::from_bnd(bnd).await;
            if let Err(why)=bnd.cd_check(cd).await{
                return why.log(cmd, &on, false, &mut err).await;
            }
            if defer{
                cmd.defer_res(&mut err, &on,false).await;
            }
            match #fname(bnd).await{
                Err(why)=>{
                    match !defer{
                        true=>why.log(cmd, &on, false, &mut err).await,
                        false=>why.log_defer(cmd, &on, &mut err).await,
                    };
                }
                Ok(_)=>{
                    bnd.cooldown(cd).await
                }
            }
        }
    };
    quete.into()
}
#[proc_macro_attribute]
pub fn button(args: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as ItemFn);
    let args = parse_macro_input!(args as AttributeArgs);
    let mut iter = args.iter();
    let cooldown = get_cooldown(iter.next().unwrap());
    let defer = get_bool(iter.next().unwrap());
    let fname = &item.sig.ident;
    let new_name = format_ident!("discord_{}",fname.to_string());
    let quete = quote!{
        #item
        pub async fn #new_name(bnd:&ComponentBundle<'_>){
            let cmd = bnd.cmd();
            let on = bnd.name();
            let cd = #cooldown;
            let defer = #defer;
            let mut err = crate::ErrorLog::from_bnd(bnd).await;
            if let Err(why)=bnd.cd_check(cd).await{
                return why.log(cmd, &on, false, &mut err).await;
            }
            if defer{
                cmd.defer_res(&mut err, &on,false).await;
            }
            match #fname(bnd).await{
                Err(why)=>{
                    match !defer{
                        true=>why.log(cmd, &on, false, &mut err).await,
                        false=>why.log_defer(cmd, &on, &mut err).await,
                    };
                }
                Ok(_)=>{
                    bnd.cooldown(cd).await
                }
            }
        }
    };
    quete.into()
}
