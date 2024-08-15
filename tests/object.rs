use lox_rs::ast::object::Object;

#[test]
fn test_create_object_with_f64() {
    let obj = Object {
        type_name: std::any::type_name::<f64>(),
        value: Box::new(17.0_f64),
    };

    assert_eq!(obj.type_name, "f64");
}

#[test]
fn test_create_object_with_string() {
    let obj = Object {
        type_name: std::any::type_name::<String>(),
        value: Box::new(String::from("Hello, world!")),
    };

    assert_eq!(obj.type_name, "alloc::string::String");
}

#[test]
fn test_get_value_f64() {
    let obj = Object {
        type_name: std::any::type_name::<f64>(),
        value: Box::new(17.0_f64),
    };

    let value = obj.get_value::<f64>().unwrap();
    assert_eq!(*value, 17.0);
}

#[test]
fn test_get_value_string() {
    let obj = Object {
        type_name: std::any::type_name::<String>(),
        value: Box::new(String::from("Hello, world!")),
    };

    let value = obj.get_value::<String>().unwrap();
    assert_eq!(value, "Hello, world!");
}

#[test]
fn test_get_value_on_wrong_type() {
    let obj = Object {
        type_name: std::any::type_name::<f64>(),
        value: Box::new(17.0_f64),
    };

    let value = obj.get_value::<&str>();
    assert_eq!(value, None);
}

#[test]
fn test_new_with_f64() {
    let obj = Object::new(17.0f64);

    assert_eq!(obj.type_name, "f64");
    assert_eq!(obj.get_value::<f64>().unwrap(), &17.0);
}

#[test]
fn test_new_with_string() {
    let obj = Object::new(String::from("Hello, world!"));

    assert_eq!(obj.type_name, "alloc::string::String");
    assert_eq!(obj.get_value::<String>().unwrap(), "Hello, world!");
}

#[test]
fn test_new_with_u32() {
    let obj = Object::new(100u32);

    assert_eq!(obj.type_name, "u32");
    assert_eq!(obj.get_value::<u32>().unwrap(), &100);
}

#[test]
fn test_new_with_incorrect_type() {
    let obj = Object::new(17.0f64);

    assert!(obj.get_value::<u32>().is_none());
}

#[test]
fn test_is_with_correct_type_f64() {
    let obj = Object {
        type_name: std::any::type_name::<f64>(),
        value: Box::new(17.0f64),
    };

    assert!(obj.is::<f64>());
}

#[test]
fn test_is_with_incorrect_type_f64() {
    let obj = Object {
        type_name: std::any::type_name::<f64>(),
        value: Box::new(17.0f64),
    };

    assert!(!obj.is::<u32>());
}

#[test]
fn test_is_with_correct_type_string() {
    let obj = Object {
        type_name: std::any::type_name::<String>(),
        value: Box::new(String::from("Hello")),
    };

    assert!(obj.is::<String>());
}

#[test]
fn test_is_with_incorrect_type_string() {
    let obj = Object {
        type_name: std::any::type_name::<String>(),
        value: Box::new(String::from("Hello")),
    };

    assert!(!obj.is::<&str>());
}

#[test]
fn test_is_with_correct_type_bool() {
    let obj = Object {
        type_name: std::any::type_name::<bool>(),
        value: Box::new(true),
    };

    assert!(obj.is::<bool>());
}

#[test]
fn test_is_with_incorrect_type_bool() {
    let obj = Object {
        type_name: std::any::type_name::<bool>(),
        value: Box::new(true),
    };

    assert!(!obj.is::<i32>());
}

#[test]
fn test_to_string_with_f64() {
    let obj = Object::new(17.0f64);
    assert_eq!(obj.to_string(), "17");
}

#[test]
fn test_to_string_with_string() {
    let obj = Object::new(String::from("Hello, world!"));
    assert_eq!(obj.to_string(), "Hello, world!");
}

#[test]
fn test_to_string_with_bool_true() {
    let obj = Object::new(true);
    assert_eq!(obj.to_string(), "true");
}

#[test]
fn test_to_string_with_bool_false() {
    let obj = Object::new(false);
    assert_eq!(obj.to_string(), "false");
}

#[test]
fn test_to_string_with_nil() {
    #[derive(Debug)]
    struct MyStruct;
    let obj = Object::new(MyStruct);
    assert_eq!(obj.to_string(), "nil");
}
