use std::any::{type_name, Any};

#[derive(Debug)]
pub struct Object {
    pub type_name: &'static str,
    pub value: Box<dyn Any>,
}

impl Object {
    pub fn new<T: 'static>(value: T) -> Self {
        Self {
            type_name: std::any::type_name::<T>(),
            value: Box::new(value),
        }
    }

    pub fn is<T: 'static>(&self) -> bool {
        self.type_name == std::any::type_name::<T>()
    }

    pub fn get_value<T: 'static>(&self) -> Option<&T> {
        if self.type_name == std::any::type_name::<T>() {
            self.value.downcast_ref::<T>()
        } else {
            println!(
                "Expected type {}, found type {}",
                type_name::<T>(),
                self.type_name
            );
            None
        }
    }

    pub fn to_string(&self) -> String {
        if self.is::<f64>() {
            self.get_value::<f64>().unwrap().to_string()
        } else if self.is::<String>() {
            self.get_value::<String>().unwrap().to_string()
        } else if self.is::<bool>() {
            self.get_value::<bool>().unwrap().to_string()
        } else {
            "nil".to_string()
        }
    }
}
