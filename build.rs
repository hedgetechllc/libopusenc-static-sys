use std::{env, fs, path::PathBuf, process::Command};

fn main() {
  if cfg!(target_os = "macos") {
    if let Ok(output) = Command::new("rustc").args(&["--print", "deployment-target"]).output() {
      if output.status.success() {
        if let Some(target) = std::str::from_utf8(&output.stdout)
          .unwrap()
          .strip_prefix("deployment_target=")
          .map(|v| v.trim())
          .map(ToString::to_string)
        {
          unsafe {
            std::env::set_var("MACOSX_DEPLOYMENT_TARGET", target);
          }
        }
      }
    }
  }

  let opus_include = PathBuf::from(std::env::var_os("DEP_OPUS_INCLUDE").unwrap()).join("opus");
  let opus_lib = PathBuf::from(std::env::var_os("DEP_OPUS_LIB").unwrap());
  let dest = PathBuf::from(env::var_os("OUT_DIR").unwrap());
  let build_dir = dest.join("build");
  let mut builder = cc::Build::new();
  builder
    .include(&opus_include)
    .include("libopusenc/include")
    .file("libopusenc/src/ogg_packer.c")
    .file("libopusenc/src/opus_header.c")
    .file("libopusenc/src/opusenc.c")
    .file("libopusenc/src/picture.c")
    .file("libopusenc/src/resample.c")
    .file("libopusenc/src/unicode_support.c")
    .define("OUTSIDE_SPEEX", "TRUE")
    .define("RANDOM_PREFIX", "opusenc_")
    .define("PACKAGE_NAME", "\"libopusenc\"")
    .define("PACKAGE_VERSION", "\"v0.2.1-16\"")
    .warnings(false)
    .static_crt(true)
    .opt_level(3)
    .out_dir(&build_dir);
  #[cfg(target_os = "windows")]
  builder.flag("/Zl");
  #[cfg(not(target_os = "windows"))]
  builder.flag("-fvisibility=hidden").flag("-flto");
  builder.compile("opusenc");

  fs::create_dir_all(dest.join("lib/pkgconfig")).unwrap();
  fs::create_dir_all(dest.join("include")).unwrap();
  for entry in fs::read_dir(&opus_include).unwrap() {
    let entry = entry.unwrap();
    let path = entry.path();
    if path.is_file() {
      fs::copy(&path, dest.join("include").join(path.file_name().unwrap())).unwrap();
    }
  }
  fs::copy("libopusenc/include/opusenc.h", dest.join("include/opusenc.h")).unwrap();
  #[cfg(target_os = "windows")]
  {
    fs::copy(build_dir.join("opusenc.lib"), dest.join("lib\\opusenc.lib")).unwrap();
    fs::copy(opus_lib, dest.join("lib\\opus.lib")).unwrap();
  }
  #[cfg(not(target_os = "windows"))]
  {
    fs::copy(build_dir.join("libopusenc.a"), dest.join("lib/libopusenc.a")).unwrap();
    fs::copy(opus_lib, dest.join("lib/libopus.a")).unwrap();
  }
  fs::write(
    dest.join("lib/pkgconfig/libopusenc.pc"),
    fs::read_to_string("libopusenc/libopusenc.pc.in")
      .unwrap()
      .replace("@prefix@", dest.to_str().unwrap())
      .replace("@exec_prefix@", "${prefix}")
      .replace("@libdir@", "${exec_prefix}/lib")
      .replace("@includedir@", "${prefix}/include")
      .replace("@PACKAGE_VERSION@", "0.2.1")
      .replace("@lrintf_lib@", ""),
  )
  .unwrap();

  println!("cargo:root={}", dest.display());
  #[cfg(target_os = "windows")]
  {
    println!("cargo:include={}\\include", dest.display());
    println!("cargo:lib_path={}\\lib", dest.display());
    println!("cargo:lib={}\\lib\\opusenc.lib", dest.display());
    println!("cargo:rustc-link-search=native={}\\lib", dest.display());
  }
  #[cfg(not(target_os = "windows"))]
  {
    println!("cargo:include={}/include", dest.display());
    println!("cargo:lib_path={}/lib", dest.display());
    println!("cargo:lib={}/lib/libopusenc.a", dest.display());
    println!("cargo:rustc-link-search=native={}/lib", dest.display());
  }
  println!("cargo:rustc-link-lib=static=opusenc");
  println!("cargo:rustc-link-lib=static=opus");
}
