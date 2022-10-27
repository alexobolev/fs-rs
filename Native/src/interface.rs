use std::ffi::{CStr};
use std::os::raw::{c_char, c_int};
use crate::implementation::{Builder};


#[no_mangle]
pub unsafe extern "C" fn BuilderCreate(name: *const c_char, number: c_int) -> *mut Builder {
    println!("[INTERF] Initializing a Builder instance...");

    let name_slice = CStr::from_ptr(name).to_str()
        .expect("failed to convert foreign char* to a rust string");
    let instance = Box::new(Builder::new(name_slice, number as i32));

    let allocated_ptr = Box::into_raw(instance);
    println!("[INTERF]   (returning) allocated_ptr = {:p}", allocated_ptr);

    allocated_ptr
}

#[no_mangle]
pub unsafe extern "C" fn BuilderFree<'a>(this: *mut Builder) -> () {
    println!("[INTERF] Dropping a Builder instance at {:p}...", this);
    let boxed = Box::from_raw(this);
    drop(boxed);
}

#[no_mangle]
pub unsafe extern "C" fn BuilderGetNumber<'a>(this: *mut Builder) -> c_int {
    let builder = this.as_ref().expect("failed to get builder reference");
    builder.get_number()
}

#[no_mangle]
pub unsafe extern "C" fn BuilderSetNumber<'a>(this: *mut Builder, value: c_int) -> () {
    let builder = this.as_mut().expect("failed to get builder mut reference");
    builder.set_number(value as i32);
}

#[no_mangle]
pub unsafe extern "C" fn BuilderGetName<'a>(this: *mut Builder) -> *const c_char {
    println!("Getting name of a builder at {:p} => {} ...", this, (*this).get_name());
    let builder = this.as_mut().expect("failed to get builder mut reference");
    builder.get_name().as_ptr() as *const c_char
}

#[no_mangle]
pub unsafe extern "C" fn BuilderSetName<'a>(this: *mut Builder, name: *const c_char) -> () {
    let builder = this.as_mut().expect("failed to get builder mut reference");
    let name_slice = CStr::from_ptr(name).to_str()
        .expect("failed to convert foreign char* to a rust string");
    builder.set_name(name_slice);
}
