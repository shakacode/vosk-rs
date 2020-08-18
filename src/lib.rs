extern crate cblas;
extern crate openblas_src;

#[used]
#[no_mangle]
pub static FLAGS_v: i32 = 0;

mod ffi {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(dead_code)]
    
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

mod model;
mod session;
mod speaker;

pub use model::VoskModel;
pub use session::{VoskSession, VoskSessionConfig, VoskSessionConfigBuilder};
pub use speaker::SpeakerModel;