use panduza_platform_core::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let crate_out_dir = std::env::var("CRATE_OUT_DIR").unwrap();

    let lib_path = PathBuf::from(crate_out_dir).join("pza_plugin_fakes.dll");

    let system_plugins_dir = env::system_default_plugins_dir().unwrap();

    fs::create_dir_all(system_plugins_dir.clone()).unwrap();

    let system_plugins_path = system_plugins_dir.join("pza_plugin_fakes.dll");

    println!(
        "copy ({}) into ({})",
        lib_path.display(),
        system_plugins_path.display()
    );

    fs::copy(lib_path, system_plugins_path).unwrap();
}
