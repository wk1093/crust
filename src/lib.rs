// Macro for import
#[macro_export]
macro_rules! i{
    ($name:ident) => {
        mod $name {
            include!(concat!(env!("OUT_DIR"), "/crust", "/", stringify!($name), ".rs"));
        }
    }
}

// Stuff for conv .crs files to .rs files
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::env;
use regex::Regex;

// LANGUAGE
// "type name = val": "let mut name: type = val"
// "const type name = val": "let name: type = val"
// "a b(...type name...)": "name: type"
// "type name(args)": "fn name(args) -> type"

fn compile(contentsoffile: &str) -> String {
    let mut contents = contentsoffile.to_string();

    // "const type name = val": "let name: type = val"
    let cvar_re: Regex = Regex::new(r"const(?: |\t)*([a-zA-Z_][a-zA-Z0-9_]*)(?: |\t)*([a-zA-Z_][a-zA-Z0-9_]*)(?: |\t)*=(?: |\t)*(.*)(?: |\t)*;").unwrap(); // cvar(?: |\t)*([a-zA-Z_][a-zA-Z0-9_]*)(?: |\t)*([a-zA-Z_][a-zA-Z0-9_]*)(?: |\t)*=(?: |\t)*(.*)(?: |\t)*;
    for cap in cvar_re.captures_iter(&contents.clone()) {
        println!("    CVAR: type '{}', name '{}', val '{}'", &cap[1], &cap[2], &cap[3]);
        contents = contents.replace(&cap[0], &format!("let {}: {} = {};", &cap[2], &cap[1], &cap[3]));
    }

    // "a b(...type name...)": "name: type"   ftype                            fname                       \( ...         type                         name              ... \)
    loop {
        let fvar_re: Regex = Regex::new(r"([a-zA-Z_][a-zA-Z0-9_]*)(?: |\t)+([a-zA-Z_][a-zA-Z0-9_]*)(?: |\t)*\((.*)([a-zA-Z_][a-zA-Z0-9_]*)(?: |\t)([a-zA-Z_][a-zA-Z0-9_]*)(.*)\)").unwrap();
        for cap in fvar_re.captures_iter(&contents.clone()) {
            println!("    FVAR: type '{}', name '{}'", &cap[4], &cap[5]);
            contents = contents.replace(&cap[0], &format!("{} {}({}{}: {}{})", &cap[1], &cap[2], &cap[3], &cap[5], &cap[4], &cap[6]));
        }
        if !fvar_re.is_match(&contents.clone()) {
            break;
        }
    }

    // "type name = val": "let mut name: type = val"
    let var_re: Regex = Regex::new(r"([a-zA-Z_][a-zA-Z0-9_]*)(?: |\t)*([a-zA-Z_][a-zA-Z0-9_]*)(?: |\t)*=(?: |\t)*(.*)(?: |\t)*;").unwrap(); //var(?: |\t)*([a-zA-Z_][a-zA-Z0-9_]*)(?: |\t)*([a-zA-Z_][a-zA-Z0-9_]*)(?: |\t)*=(?: |\t)*(.*)(?: |\t)*;
    for cap in var_re.captures_iter(&contents.clone()) {
        println!("    VAR: type '{}', name '{}', val '{}'", &cap[1], &cap[2], &cap[3]);
        contents = contents.replace(&cap[0], &format!("let mut {}: {} = {};", &cap[2], &cap[1], &cap[3]));
    }

    // "type name(args)": "fn name(args) -> type"
    let dfun_re: Regex = Regex::new(r"([a-zA-Z_][a-zA-Z0-9_]*)(?: |\t)+([a-zA-Z_][a-zA-Z0-9_]*)(?: |\t)*\((.*)\)").unwrap(); // dfun(?: |\t)*([a-zA-Z_][a-zA-Z0-9_]*)(?: |\t)*([a-zA-Z_][a-zA-Z0-9_]*)(?: |\t)*\((.*)\)
    for cap in dfun_re.captures_iter(&contents.clone()) {
        println!("    DFUN: type '{}', name '{}'", &cap[1], &cap[2]);
        let mut typen: String = cap[1].to_string();
        if typen == "void" {
            typen = "()".to_string();
        }
        contents = contents.replace(&cap[0], &format!("fn {}({}) -> {}", &cap[2], &cap[3], typen));
    }

    return contents;
}

fn conv(inpath: String, outpath: String, basepath: String) { // loops through crs files in inpath, and creates the respective rs file inside of outpath
    let paths = fs::read_dir(inpath).unwrap();
    for path in paths {
        let path = path.unwrap().path();
        if path.is_dir() {
            conv(path.to_str().unwrap().to_string(), outpath.clone(), basepath.clone());
        } else {
            let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
            if file_name.ends_with(".crs") {
                println!("CRUST: '{}'", path.to_str().unwrap());
                let filepath = Path::new(&path).strip_prefix(Path::new(&basepath)).unwrap().to_str().unwrap().to_string().replace(".crs", ".rs");
                let dest_path = Path::new(&outpath).join(filepath);
                let contents = fs::read_to_string(path).unwrap();
                let mut new_contents = String::new();
                new_contents.push_str(&compile(&contents));
                let mut dest_filepath = PathBuf::new();
                dest_filepath.push(&dest_path);
                dest_filepath.pop();
                fs::create_dir_all(&dest_filepath).unwrap();
                fs::write(&dest_path, new_contents).unwrap();
            }
        }
    }
}

pub fn build() {
    println!("cargo:rerun-if-changed=src/*.crs\n");
    let out_dir = env::var("OUT_DIR").unwrap() + "/crust";
    let in_dir = env::var("CARGO_MANIFEST_DIR").unwrap() + "/src";
    conv(in_dir.clone(), out_dir, in_dir);
}