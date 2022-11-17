pub mod math;

use phper::modules::Module;
use phper::php_get_module;

#[php_get_module]
pub fn get_module() -> Module {
    let mut module = Module::new(
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
    );

    math::add_functions(&mut module);

    module
}
