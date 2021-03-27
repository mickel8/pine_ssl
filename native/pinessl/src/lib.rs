use rustler::resource::ResourceArc;
use rustler::{Env, Term};
use std::sync::Mutex;

rustler::atoms! {
    ok,
    error
}

rustler::init!("Elixir.PineSSL", [ssl_ctx_new, ssl_new], load=load);

fn load(env: Env, _info: Term) -> bool {
    rustler::resource!(SSL_CTX, env);
    rustler::resource!(SSL, env);
    true
}

#[allow(non_camel_case_types)]
pub struct SSL_CTX(Mutex<boring_sys::SSL_CTX>);

#[allow(non_camel_case_types)]
pub struct SSL(Mutex<boring_sys::SSL>);

#[rustler::nif]
fn ssl_ctx_new() -> ResourceArc<SSL_CTX> {
    unsafe {
        let ctx = boring_sys::SSL_CTX_new(boring_sys::TLS_method());
        //println!("{:#?}", ctx);
        let resource = ResourceArc::new(SSL_CTX(Mutex::new(*ctx)));
        //println!("{:#?}", resource.0);
        resource 
    }
}

#[rustler::nif]
fn ssl_new(ssl_ctx: ResourceArc<SSL_CTX>) -> ResourceArc<SSL> {
    unsafe {
        let ssl_ctx = &mut *ssl_ctx.0.try_lock().unwrap();
        //println!("{:#?}", ssl_ctx);
        let ssl = boring_sys::SSL_new(&mut *ssl_ctx as *mut boring_sys::SSL_CTX);
        ResourceArc::new(SSL(Mutex::new(*ssl)))
    }
}

