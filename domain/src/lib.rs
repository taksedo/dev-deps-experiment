#[derive(Default)]
pub struct MyStruct {
    pub name: String,
}

impl MyStruct {
    pub fn some_method(&self) {
        println!("Hello, {}!", self.name);
    }
}

#[cfg(test)]
mod tests {
    use test_fixtures::*;
    use domain::*;

    #[test]
    fn it_works() {
        let result = my_struct();
        assert_eq!(result.name, "test");
    }

    #[test]
    fn test_some_method_test() {
        let struc = MyStruct::default();
        test_some_method(struc);
    }
}
