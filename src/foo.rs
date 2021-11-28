////////////////////////////////
///
///
// assume c function signature as follows:  see src_c/foo.c

// struct foo;
//
// int foo_create(struct foo** result);
// void foo_bar(struct foo* f, int param);
// void foo_destroy(struct foo* f);
use crate::ffi;
use std::ffi::c_void;
use std::os::raw::c_int;
use std::mem;
use std::ptr::Unique;


pub struct Foo {
  foo: Unique<c_void>,
}

impl Foo {
  pub fn new() -> Result<Foo, i32> {
    unsafe {
      // An uninitialized raw pointer:
      let mut foo: ffi::foo = mem::uninitialized();
      // Low-level constructor, pass a pointer to our pointer:
      let ret = ffi::foo_create(&mut foo);
      // Suppose the constructor returns 0 on succes and non-zero result on error:
      match ret {
        // A new instance of Unique is created here
        //  handing over the ownership of the raw pointer to it:
        0 => Ok(Foo {
          foo: Unique::new(foo).unwrap(),
        }),
        e => Err(e as i32),
      }
    }
  }

  pub fn set_a(&mut self, param: i32) {
    // Note that the Unique pointer is dereferenced to yield the raw pointer:
    unsafe {
      ffi::set_a(self.foo.as_ptr(), param as c_int);
    }
  }

  pub fn get_a(&mut self) -> i32 {
    unsafe {
      ffi::get_a(self.foo.as_ptr()) as i32
    }
  }
}

impl Drop for Foo {
  fn drop(&mut self) {
    unsafe {
      ffi::foo_destroy(self.foo.as_ptr());
    }
  }
}
