
#![ allow( warnings ) ]

static PROTO_VERSION: u8 = 1;

// the core part has the code that can run on any platform
mod core {
    pub mod memstore;
    pub mod instance;
    pub mod item;
    pub mod id;
    pub mod proto;
    pub mod error;
    pub mod types;
}

pub use core::*;
pub use core::instance::module::Module;
pub use core::instance::Instance;
pub use core::error::MizeError;
pub use core::error::MizeResult;

// platform specific stuff
pub mod platform {
    #[cfg(feature = "wasm-target")]
    pub mod wasm;

    #[cfg(feature = "os-target")]
    pub mod os;

    pub mod any {
        #[cfg(feature = "wasm-target")]
        pub use super::wasm::wasm_instance_init as instance_init;

        #[cfg(feature = "os-target")]
        pub use super::os::os_instance_init as instance_init;

        #[cfg(not(any(feature = "os-target", feature = "wasm-target")))]
        pub use super::super::instance_init;


        #[cfg(feature = "os-target")]
        pub use super::os::load_module;

        #[cfg(not(any(feature = "os-target", feature = "wasm-target")))]
        pub use super::super::load_module;

    }
}


pub fn instance_init(instance: &mut core::instance::Instance) {}


pub fn load_module(instance: &mut core::instance::Instance, name: &str) -> MizeResult<()> { Ok(()) }


