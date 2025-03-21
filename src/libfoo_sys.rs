use libc::c_int;

#[cfg(feature = "vendored")]
mod vendored;

#[cfg(feature = "vendored")]
pub use self::vendored::*;

#[cfg(not(feature = "vendored"))]
macro_rules! clib {
    ($name:ident
        (
            $( $arg_name:ident : $arg_ty:ty ),*
            $(,)?
        )
        -> $ret:ty) => {
            #[link(name = "foo")]
            unsafe extern "C" {
                pub fn $name( $( $arg_name : $arg_ty ),* ) -> $ret;
            }

    };
}

#[cfg(feature = "vendored")]
macro_rules! clib {
    ($name:ident
        (
            $( $arg_name:ident : $arg_ty:ty ),*
            $(,)?
        )
        -> $ret:ty) => {
            pub unsafe fn $name( $( $arg_name : $arg_ty ),* ) -> $ret {
                let libfoo = unsafe { init_libfoo() };

                let func = unsafe { libfoo.get::<unsafe extern "C" fn($( $arg_ty ),*) -> $ret>(concat!(stringify!($name), "\0").as_bytes()) }
                    .expect("failed to get function");

                unsafe { func( $( $arg_name ),* ) }
            }
    };
}

clib!(sum(x: c_int, y: c_int) -> c_int);
