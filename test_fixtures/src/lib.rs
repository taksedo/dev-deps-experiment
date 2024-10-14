#![cfg(feature = "testing")]
use domain::MyStruct;

pub fn my_struct() -> MyStruct {
    MyStruct {
        name: "test".to_string(),
    }
}

pub fn test_some_method(struc: MyStruct) {
    let result = struc.some_method();
    assert_eq!(result, ());
}