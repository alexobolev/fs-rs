use std::ffi::{CString};


pub trait Greeter {
    fn say_hi(&self, name: &str);
}

pub struct Builder {
    name: CString,
    number: i32
}

impl Builder {

    /// ```
    /// use native::implementation::Builder;
    /// let builder = Builder::new("John", 42);
    /// ```
    pub fn new(name: &str, number: i32) -> Builder {
        println!("[NATIVE] Initializing a Builder instance...");
        Builder { name: CString::new(name).unwrap(), number: number }
    }

    /// ```
    /// use native::implementation::Builder;
    /// let builder = Builder::new("Susan", 63);
    /// assert_eq!(builder.get_name(), "Susan");
    /// ```
    pub fn get_name(&self) -> &str { self.name.as_c_str().to_str().unwrap() }

    /// ```
    /// use native::implementation::Builder;
    /// let builder = Builder::new("Mary", 21);
    /// assert_eq!(builder.get_number(), 21);
    /// ```
    pub fn get_number(&self) -> i32 { self.number }

    /// ```
    /// use native::implementation::Builder;
    /// let mut builder = Builder::new("", 84);
    /// builder.set_name("Thomas");
    /// assert_eq!(builder.get_name(), "Thomas");
    /// assert_eq!(builder.get_number(), 84);
    /// ```
    pub fn set_name(&mut self, value: &str) -> &Self {
        self.name = CString::new(value)
            .expect("argument string contained an internal null");
        self
    }

    /// ```
    /// use native::implementation::Builder;
    /// let mut builder = Builder::new("Anna", 84);
    /// builder.set_number(105);
    /// assert_eq!(builder.get_name(), "Anna");
    /// assert_eq!(builder.get_number(), 105);
    /// ```
    pub fn set_number(&mut self, value: i32) -> &Self {
        self.number = value; self
    }
}

impl Greeter for Builder {
    fn say_hi(&self, name: &str) {
        println!("Hi there, {}! I'm a Builder: name = {}, number = {}.",
            name, self.name.to_str().expect("contained string was invalid"),
            self.number);
    }
}

impl Drop for Builder {
    fn drop(&mut self) {
        println!("[NATIVE] Dropping a Builder instance at {:p}...", self);
    }
}
