extern crate libc;
extern crate solder;

use solder::info::*;
use solder::zend::*;
use solder::*;

#[no_mangle]
pub extern "C" fn php_module_info() {
    print_table_start();
    print_table_row("A demo PHP extension written in Rust", "enabled");
    print_table_end();
}

#[no_mangle]
pub extern "C" fn get_module() -> *mut zend::Module {
    let function = FunctionBuilder::new(c_str!("hello_world"), hello_world)
        .with_arg(ArgInfo::new(c_str!("name"), 0, 0, 0))
        .build();
    ModuleBuilder::new(c_str!("hello_world"), c_str!("0.1.0-dev"))
        .with_info_function(php_module_info)
        .with_function(function)
        .build()
        .into_raw()
}

#[no_mangle]
pub extern "C" fn hello_world(_data: &ExecuteData, retval: &mut Zval) {
    let mut name_zval = Zval::new_as_null();
    php_parse_parameters!(&mut name_zval);
    let name = String::try_from(name_zval).ok().unwrap();
    let hello = format!("Hello {}", name);
    php_return!(retval, hello);
}
