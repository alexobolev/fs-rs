use std::ffi::{CStr};
use std::os::raw::{c_char, c_int};
use crate::implementation::{Builder};


#[no_mangle]
pub unsafe extern "C" fn nat_builder_create<'a>(name: *const c_char, number: c_int)
    -> *mut Builder<'a>
{
    println!("[INTERF] Initializing a Builder instance...");

    let name_slice = CStr::from_ptr(name).to_str()
        .expect("failed to convert foreign char* to a rust string");
    let instance = Box::new(Builder::new(name_slice, number as i32));

    Box::into_raw(instance)
}

#[no_mangle]
pub unsafe extern "C" fn nat_builder_free<'a>(this: *mut Builder<'a>)
    -> ()
{
    println!("[INTERF] Dropping a Builder instance at {:p}...", this);
    let boxed = Box::from_raw(this);
    drop(boxed);
}
