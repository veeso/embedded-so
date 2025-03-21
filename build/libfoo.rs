use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs};

pub fn build_libfoo() {
    println!("building vendored foo library...");
    let artifacts = Build::default().build().expect("build failed");

    let shared_object = artifacts.lib_dir.join("libfoo.so");
    let dest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    // copy shared object to dest_dir
    fs::copy(&shared_object, dest_dir.join("libfoo.so"))
        .expect("failed to copy shared object to dest_dir");
}

struct Artifacts {
    lib_dir: PathBuf,
}

struct Build {
    out_dir: Option<PathBuf>,
    host: Option<String>,
    target: Option<String>,
}

impl Default for Build {
    fn default() -> Self {
        Self {
            out_dir: env::var_os("OUT_DIR").map(|s| PathBuf::from(s).join("foo-build")),
            host: env::var("HOST").ok(),
            target: env::var("TARGET").ok(),
        }
    }
}

impl Build {
    fn build(&self) -> Result<Artifacts, String> {
        let target = &self.target.as_ref().ok_or("TARGET dir not set")?[..];
        let host = &self.host.as_ref().ok_or("HOST dir not set")?[..];
        let out_dir = self.out_dir.as_ref().ok_or("OUT_DIR not set")?;
        let build_dir = out_dir.join("build");

        if build_dir.exists() {
            fs::remove_dir_all(&build_dir).map_err(|e| format!("build_dir: {e}"))?;
        }

        let inner_dir = build_dir.join("libfoo");
        fs::create_dir_all(&inner_dir).map_err(|e| format!("inner_dir: {e}"))?;

        // copy libfoo/ to build_dir
        cp_r(&Self::source_dir(), &inner_dir)?;

        // init cc
        let mut cc = cc::Build::new();
        cc.target(target).host(host).warnings(false).opt_level(2);
        let compiler = cc.get_compiler();
        let mut cc_env = compiler.cc_env();
        if cc_env.is_empty() {
            cc_env = compiler.path().to_path_buf().into_os_string();
        }

        // build dir
        let lib_build_dir = inner_dir.join("build");
        // remove build/ dir if it exists
        if lib_build_dir.exists() {
            fs::remove_dir_all(&lib_build_dir).map_err(|e| format!("lib_build_dir: {e}"))?;
        }
        fs::create_dir_all(&lib_build_dir).map_err(|e| format!("lib_build_dir: {e}"))?;

        // run cmake
        let mut cmake = Command::new("cmake");
        cmake.arg("..");
        cmake.current_dir(&lib_build_dir);
        cmake.env("CC", cc_env);

        // run
        self.run_command(cmake, "cmake")?;

        // run make
        let mut make = Command::new("make");
        make.current_dir(&lib_build_dir);
        self.run_command(make, "make")?;

        // get lib and include path
        //let include_dir = inner_dir.join("include");
        let lib_dir = lib_build_dir.join("lib");

        Ok(Artifacts { lib_dir })
    }

    fn source_dir() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("libfoo")
    }

    #[track_caller]
    fn run_command(&self, mut command: Command, desc: &str) -> Result<(), String> {
        println!("running {:?}", command);
        let status = command.status();

        let verbose_error = match status {
            Ok(status) if status.success() => return Ok(()),
            Ok(status) => format!(
                "'{exe}' reported failure with {status}",
                exe = command.get_program().to_string_lossy()
            ),
            Err(failed) => match failed.kind() {
                std::io::ErrorKind::NotFound => format!(
                    "Command '{exe}' not found. Is {exe} installed?",
                    exe = command.get_program().to_string_lossy()
                ),
                _ => format!(
                    "Could not run '{exe}', because {failed}",
                    exe = command.get_program().to_string_lossy()
                ),
            },
        };
        println!("cargo:warning={desc}: {verbose_error}");
        Err(format!(
            "Error {desc}:
    {verbose_error}
    Command failed: {command:?}"
        ))
    }
}

fn cp_r(src: &Path, dst: &Path) -> Result<(), String> {
    for f in fs::read_dir(src).map_err(|e| format!("{}: {e}", src.display()))? {
        let f = match f {
            Ok(f) => f,
            _ => continue,
        };
        let path = f.path();
        let name = path
            .file_name()
            .ok_or_else(|| format!("bad dir {}", src.display()))?;

        // Skip git metadata as it's been known to cause issues (#26) and
        // otherwise shouldn't be required
        if name.to_str() == Some(".git") {
            continue;
        }

        let dst = dst.join(name);
        let ty = f.file_type().map_err(|e| e.to_string())?;
        if ty.is_dir() {
            fs::create_dir_all(&dst).map_err(|e| e.to_string())?;
            cp_r(&path, &dst)?;
        } else if ty.is_symlink() && path.iter().any(|p| p == "cloudflare-quiche") {
            // not needed to build
            continue;
        } else {
            let _ = fs::remove_file(&dst);
            if let Err(e) = fs::copy(&path, &dst) {
                return Err(format!(
                    "failed to copy '{}' to '{}': {e}",
                    path.display(),
                    dst.display()
                ));
            }
        }
    }
    Ok(())
}
