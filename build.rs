use std::collections::HashSet;
use std::{env, fs};
use std::path::{Path, PathBuf};

#[derive(Debug)]
struct IgnoreMacros(HashSet<String>);

impl bindgen::callbacks::ParseCallbacks for IgnoreMacros {
    fn will_parse_macro(&self, name: &str) -> bindgen::callbacks::MacroParsingBehavior {
        if self.0.contains(name) {
            bindgen::callbacks::MacroParsingBehavior::Ignore
        } else {
            bindgen::callbacks::MacroParsingBehavior::Default
        }
    }
}

fn main() {
  println!("cargo:rustc-link-lib=m");

  let replacements = vec![
    "setulb", "mainlb", "lnsrlb", "dcsrch", "dcstep",
    "active", "bmv", "cauchy", "hpsolb", "cmprlb",
    "formk", "formt", "freev", "matupd", "projgr",
    "subsm", "prn1lb", "prn2lb", "prn3lb", "errclb",
    "dpofa", "dtrsl", "daxpyRef", "dcopyRef", "ddotRef", "dscalRef", "timer",
  ];

  let files = vec![
    "lbfgsb.h",
    "lbfgsb.c",
    "linesearch.c",
    "subalgorithms.c",
    "print.c",
    "linpack.c",
    "miniCBLAS.c",
    "timer.c",
  ];

  let mut lib = "".to_string();
  let out_dir = env::var("OUT_DIR").unwrap();

  // God help us
  for i in 0..64 {
    
    for file in &files {
      let contents = fs::read_to_string(Path::new(format!("lib/src/{}", file).as_str())).unwrap();
      let contents = replacements.iter().fold(contents, |acc, &r| acc.replace(r, format!("{}_{}", r, i).as_str()));
      let out_dir_file = PathBuf::from(out_dir.clone()).join(file);
      fs::write(out_dir_file.clone(), contents).expect("Couldn't write");
    }

    cc::Build::new()
      .cpp(false)
      .include(out_dir.clone())
      .file(out_dir.clone()+"/lbfgsb.c")
      .file(out_dir.clone()+"/linesearch.c")
      .file(out_dir.clone()+"/subalgorithms.c")
      .file(out_dir.clone()+"/print.c")
      .file(out_dir.clone()+"/linpack.c")
      .file(out_dir.clone()+"/miniCBLAS.c")
      .file(out_dir.clone()+"/timer.c")
      .compile(format!("liblbfgsb_{}.a", i).as_str());

    let ignored_macros = IgnoreMacros(
      vec![
        "FP_INFINITE".into(),
        "FP_NAN".into(),
        "FP_NORMAL".into(),
        "FP_SUBNORMAL".into(),
        "FP_ZERO".into(),
        "IPPORT_RESERVED".into(),
      ]
      .into_iter()
      .collect(),
    );

    let bindings = bindgen::Builder::default()
      .header(out_dir.clone()+"/lbfgsb.h")
      .parse_callbacks(Box::new(ignored_macros))
      .generate()
      .expect("Unable to generate bindings");

    let bindings_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join(format!("bindings_{}.rs", i).as_str());
    bindings
      .write_to_file(bindings_path)
      .expect("Couldn't write bindings!");


    let module = fs::read_to_string(Path::new("src/lbfgsb.rs")).unwrap();
    let module = module.replace("setulb", format!("setulb_{}", i).as_str()).replace("bindings.rs", format!("bindings_{}.rs", i).as_str());
    let out_dir_file = PathBuf::from(out_dir.clone()).join(format!("lbfgsb_{}.rs", i).as_str());
    fs::write(out_dir_file.clone(), module).expect("Couldn't write");


    lib = format!("{}pub mod lbfgsb_{};\n", lib, i);
  }

  let lib_file = PathBuf::from(out_dir.clone()).join("lib.rs");
  fs::write(lib_file.clone(), lib).expect("Couldn't write lib file");
}
// build.rs:1 ends here
