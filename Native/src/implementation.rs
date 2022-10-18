pub trait Greeter {
    fn say_hi(&self, name: &str);
}

pub struct Builder<'a> {
    name: &'a str,
    number: i32
}

impl<'a> Builder<'a> {

    /// ```
    /// use native::implementation::Builder;
    /// let builder = Builder::new("John", 42);
    /// ```
    pub fn new(name: &'a str, number: i32) -> Builder<'a> {
        println!("[NATIVE] Initializing a Builder instance...");
        Builder { name: name, number: number }
    }

    /// ```
    /// use native::implementation::Builder;
    /// let builder = Builder::new("Susan", 63);
    /// assert_eq!(builder.get_name(), "Susan");
    /// ```
    pub fn get_name(&self) -> &'a str { self.name }

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
    pub fn set_name(&mut self, value: &'a str) -> &Self {
        self.name = value; self
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

impl<'a> Greeter for Builder<'a> {
    fn say_hi(&self, name: &str) {
        println!("Hi there, {}! I'm a Builder: name = {}, number = {}.",
            name, self.name, self.number);
    }
}

impl<'a> Drop for Builder<'a> {
    fn drop(&mut self) {
        println!("[NATIVE] Dropping a Builder instance at {:p}...", self);
        self.name = "N/A";
        self.number = -1;
    }
}
