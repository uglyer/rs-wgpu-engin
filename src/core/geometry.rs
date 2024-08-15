use std::collections::HashMap;
use crate::core::attribute::{Attribute, AttributeF32, AttributeUsize};
use crate::core::resource::ResourceId;

pub struct Geometry {
    attributes: HashMap<&'static str, ResourceId<AttributeF32>>,
    index: Option<ResourceId<AttributeUsize>>,
}

impl Geometry {
    pub fn new() -> Self {
        Geometry {
            attributes: HashMap::new(),
            index: None,
        }
    }

    pub fn set_attribute(&mut self, key: &'static str, attribute: ResourceId<AttributeF32>) -> &mut Self {
        self.attributes.insert(key, attribute);
        self
    }

    pub fn borrow_attribute(&self, key: &'static str) -> Option<&ResourceId<AttributeF32>> {
        self.attributes.get(key)
    }

    pub fn set_index(&mut self, index: ResourceId<AttributeUsize>) -> &mut Self {
        self.index = Some(index);
        self
    }

    pub fn remove_index(&mut self) -> &mut Self {
        self.index = None;
        self
    }

    pub fn borrow_index(&self) -> Option<&ResourceId<AttributeUsize>> {
        self.index.as_ref()
    }
}
