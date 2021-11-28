
use std::ffi::c_void;
use std::os::raw::c_int;

pub type foo = *mut c_void;

#[link(name = "foo", kind = "dylib")]
extern "C" {
  pub fn foo_create(result: *mut foo) -> c_int;
  pub fn set_a(f: foo, param: c_int);
  pub fn foo_destroy(f: foo);
  pub fn get_a(f: foo) -> c_int;
}

