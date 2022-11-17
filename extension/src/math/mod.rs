mod functions;

use phper::modules::Module;
use phper::functions::Argument;

pub fn add_functions(module: &mut Module) {
    module.add_function("Psl\\Math\\from_base", functions::from_base, vec![
        Argument::by_val("number"),
        Argument::by_val("base"),
    ]);

    module.add_function("Psl\\Math\\to_base", functions::to_base, vec![
        Argument::by_val("number"),
        Argument::by_val("base"),
    ]);
}