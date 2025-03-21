#[cfg(feature = "vendored")]
mod libfoo;

fn main() {
    #[cfg(feature = "vendored")]
    libfoo::build_libfoo();
}
