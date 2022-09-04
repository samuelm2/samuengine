
use std::env;
use std::path::Path;
use std::path::PathBuf;
use fs_extra::dir::CopyOptions;
use fs_extra::copy_items;


fn get_output_path() -> PathBuf {
    //<root or manifest path>/target/<profile>/
    let manifest_dir_string = env::var("CARGO_MANIFEST_DIR").unwrap();
    let build_type = env::var("PROFILE").unwrap();
    let path = Path::new(&manifest_dir_string).join("target").join(build_type);
    return PathBuf::from(path);
}

fn main() {
    let target_dir = get_output_path();
    let resource_source = Path::join(&env::current_dir().unwrap(), "src/resources/");
    let resource_dest = Path::new(&target_dir);

    let mut options = CopyOptions::new();
    options.overwrite = true;

    let mut from_paths = Vec::new();
    from_paths.push(resource_source);
    copy_items(&from_paths, resource_dest, &options).expect("Failed to copy shaders");
}