use std::any::{Any, TypeId};

#[derive(Debug)]
pub(crate) struct Object {
    value: Box<dyn Any>,
}

impl Object {
    pub(crate) fn new<T: 'static>(value: T) -> Self {
        Self {
            value: Box::new(value),
        }
    }

    pub fn is<T: 'static>(&self) -> bool {
        self.value.is::<T>()
    }

    pub fn as_any(&self) -> &dyn Any {
        &*self.value
    }

    // pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
    //     self.value.downcast_ref::<T>()
    // }
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        if self.value.type_id() == TypeId::of::<T>() {
            self.value.downcast_ref::<T>()
        } else {
            panic!(
                "Attempted to downcast to the wrong type {:?}, {:?}.",
                self.value.type_id(),
                TypeId::of::<T>()
            );
            None
        }
    }
}
