use std::ffi::{CStr};
use std::os::raw::{c_char, c_int};
use crate::implementation::{Builder};


#[no_mangle]
pub unsafe extern "C" fn BuilderCreate<'a>(name: *const c_char, number: c_int)
    -> *mut Builder<'a>
{
    println!("[INTERF] Initializing a Builder instance...");

    let name_slice = CStr::from_ptr(name).to_str()
        .expect("failed to convert foreign char* to a rust string");
    let instance = Box::new(Builder::new(name_slice, number as i32));

    Box::into_raw(instance)
}

#[no_mangle]
pub unsafe extern "C" fn BuilderFree<'a>(this: *mut Builder<'a>)
    -> ()
{
    println!("[INTERF] Dropping a Builder instance at {:p}...", this);
    let boxed = Box::from_raw(this);
    drop(boxed);
}

#[no_mangle]
pub unsafe extern "C" fn BuilderGetNumber<'a>(this: *mut Builder<'a>)
    -> c_int
{
    let builder = this.as_ref()
        .expect("failed to get builder reference");
    builder.get_number()
}

#[no_mangle]
pub unsafe extern "C" fn BuilderSetNumber<'a>(this: *mut Builder<'a>, value: c_int)
    -> ()
{
    let builder = this.as_mut()
        .expect("failed to get builder mut reference");
    builder.set_number(value as i32);
}
