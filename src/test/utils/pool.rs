use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use crate::utils::id::{generate_id, UID};

pub struct ResourceId<T> {
    pub id: UID,
    _phantom: PhantomData<T>,
}

impl<T> ResourceId<T> {
    fn new() -> Self {
        ResourceId {
            id: generate_id(),
            _phantom: PhantomData,
        }
    }

    pub fn format(uid: UID) -> Self {
        ResourceId {
            id: uid,
            _phantom: PhantomData,
        }
    }
}

impl<T> Copy for ResourceId<T> {}

impl<T> Clone for ResourceId<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Hash for ResourceId<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<T> PartialEq for ResourceId<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T> Eq for ResourceId<T> {}


pub struct ResourcePool<T> {
    resources: HashMap<UID, T>,
    resource_id_set: HashSet<ResourceId<T>>
}

// TODO 支持遍历扫描销毁资源释放
impl<T: 'static> ResourcePool<T> {
    pub fn new() -> Self {
        ResourcePool {
            resources: HashMap::new(),
            resource_id_set: HashSet::new(),
        }
    }

    pub fn add(&mut self, resource: T) -> ResourceId<T> {
        let rid = ResourceId::new();
        self.resources.insert(rid.id, resource);
        self.resource_id_set.insert(rid);
        rid
    }

    pub fn set(&mut self, uid: UID, resource: T) -> ResourceId<T> {
        let rid = ResourceId::format(uid);
        self.resources.insert(rid.id, resource);
        self.resource_id_set.insert(rid);
        rid
    }

    pub fn borrow(&self, r_id: &ResourceId<T>) -> Option<&T> {
        self.resources.get(&r_id.id)
    }

    pub fn borrow_mut(&mut self, r_id: &ResourceId<T>) -> Option<&mut T> {
        self.resources.get_mut(&r_id.id)
    }
}
