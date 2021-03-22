use rustler::resource::ResourceArc;
use rustler::{Env, Term, NifStruct};

rustler::atoms! {
    ok,
    error
}

rustler::init!("PineSSL", [add], load=load);

#[derive(NifStruct)]
#[module = "MyStruct"]
pub struct MyStruct {
    pub a: i64
}

fn load(env: Env, _info: Term) -> bool {
    rustler::resource!(MyStruct, env);
    true
}

#[rustler::nif]
fn add(a: i64, b: i64) -> ResourceArc<MyStruct> {
    ResourceArc::new(MyStruct{a: a+b})
}


