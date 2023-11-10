// src/main.rs

// dependencies
use deno_core::error::AnyError;
use std::env;
use std::rc::Rc;

async fn run_js(file_path: &str) -> Result<(), AnyError> {
    let main_module = deno_core::resolve_path(file_path, env::current_dir()?.as_path())?;
    let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
        module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
        ..Default::default()
    });

    // had to change to execute_script_static to get it to work, the blog article is outdated
    // got rid of the .unwrap() on the creation of the js_runtime, panics if it fails
    js_runtime
        .execute_script_static("[runjs:runtime.js]", include_str!("./runtime.js"))
        .expect("Failed to execute runtime.js");

    let mod_id = js_runtime.load_main_module(&main_module, None).await?;
    let result = js_runtime.mod_evaluate(mod_id).await?;
    js_runtime.run_event_loop(false).await?;
    result
}

// got rid of the .unwrap() on the creation of the runtime, panics if it fails
fn main() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Unable to create a runtime");
    if let Err(error) = runtime.block_on(run_js("./example.js")) {
        eprintln!("error: {}", error);
    }
}
