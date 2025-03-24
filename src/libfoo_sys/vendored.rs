use std::io::Write as _;
use std::sync::OnceLock;

use libloading::Library;
use tempfile::NamedTempFile;

const LIBFOO_SO: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/libfoo.so"));
static LIBFOO_SO_FILE: OnceLock<NamedTempFile> = OnceLock::new();
static LIBFOO_LIB: OnceLock<Library> = OnceLock::new();

pub unsafe fn init_libfoo() -> &'static Library {
    let libfoo_file = LIBFOO_SO_FILE.get_or_init(|| {
        let mut file = NamedTempFile::new().expect("failed to create temp file");
        file.write_all(LIBFOO_SO)
            .expect("failed to write to temp file");
        file
    });

    LIBFOO_LIB.get_or_init(|| unsafe {
        Library::new(libfoo_file.path()).expect("failed to load libfoo.so")
    })
}
