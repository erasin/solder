use super::types::*;
use libc::*;

extern "C" {
    pub fn zend_parse_parameters(num_args: i32, format: *const c_char, ...) -> i32;
    pub fn array_set_zval_key(ht: *mut ZendArray, key: *mut Zval, value: *mut Zval) -> i32;
    pub fn php_printf(format: *const c_char, ...) -> size_t;
    pub fn _call_user_function_ex(
        object: *mut Zval,
        function_name: *mut Zval,
        retval_ptr: *mut Zval,
        param_count: u32,
        params: *mut Zval,
        no_separation: i32,
    ) -> i32;
    pub fn zend_get_callable_name(callable: *mut Zval) -> *mut ZendString;
    pub fn _efree(ptr: *mut c_void);
    pub fn free(ptr: *mut c_void);
}

#[cfg(feature = "php72")]
extern "C" {
    fn zend_strpprintf(max_len: size_t, format: *const c_char) -> *mut ZendString;
}

#[cfg(feature = "php72")]
pub fn create_zend_string(size: size_t, string: *const c_char) -> *mut ZendString {
    unsafe { zend_strpprintf(size, string) }
}

#[cfg(not(feature = "php72"))]
extern "C" {
    fn strpprintf(max_len: size_t, format: *const c_char) -> *mut ZendString;
}

#[cfg(not(feature = "php72"))]
pub fn create_zend_string(size: size_t, string: *const c_char) -> *mut ZendString {
    unsafe { strpprintf(size, string) }
}

#[cfg(feature = "php73")]
extern "C" {
    fn _zend_new_array_0() -> *mut ZendArray;
}

#[cfg(feature = "php73")]
pub fn create_zend_array(zval: &mut Zval) {
    unsafe {
        zval.type_info.type_info = InternalPhpTypes::ARRAY as u32;
        zval.value.array = _zend_new_array_0();
    }
}

#[cfg(not(feature = "php73"))]
extern "C" {
    fn _array_init(arg: *mut Zval, size: u32) -> i32;
}

#[cfg(not(feature = "php73"))]
pub fn create_zend_array(zval: &mut Zval) {
    unsafe {
        _array_init(zval, 0);
    }
}
