use std::{
    any::{
        Any,
        TypeId,
    },
    collections::HashMap,
    hash::{
        Hash,
        Hasher,
    },
    marker::PhantomData,
    mem::transmute,
};
use std::collections::HashSet;
use crate::core::attribute::{AttributeF32, AttributeF64, AttributeUsize};
use crate::core::geometry::Geometry;
use crate::objects::camera::Camera;
use crate::objects::mesh::Mesh;
use crate::utils::id::{generate_id, UId};

trait ResourcePoolTrait {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub struct ResourcePool<T> {
    resources: HashMap<UId, T>,
    resource_id_set: HashSet<ResourceId<T>>
}

impl<T: 'static> ResourcePoolTrait for ResourcePool<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

fn cast_pool<T: 'static>(pool: &dyn ResourcePoolTrait) -> &ResourcePool<T> {
    pool
        .as_any()
        .downcast_ref::<ResourcePool<T>>()
        .unwrap()
}

fn cast_pool_mut<T: 'static>(pool: &mut dyn ResourcePoolTrait) -> &mut ResourcePool<T> {
    pool
        .as_any_mut()
        .downcast_mut::<ResourcePool<T>>()
        .unwrap()
}

fn cast_pool_mut_unsafe<T: 'static>(pool: &Box<dyn ResourcePoolTrait>) -> &mut ResourcePool<T> {
    let ptr = cast_pool(pool.as_ref())
        as *const ResourcePool<T> as *mut ResourcePool<T>;
    unsafe { transmute(ptr) }
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

    pub fn borrow(&self, r_id: &ResourceId<T>) -> Option<&T> {
		self.resources.get(&r_id.id)
    }

    pub fn borrow_mut(&mut self, r_id: &ResourceId<T>) -> Option<&mut T> {
		self.resources.get_mut(&r_id.id)
    }
}

pub struct ResourcePools {
    pools: HashMap<TypeId, Box<dyn ResourcePoolTrait>>,
}

impl ResourcePools {
    pub fn new() -> Self {
        let mut pools = HashMap::new();
        Self::add::<AttributeF32>(&mut pools);
        Self::add::<AttributeF64>(&mut pools);
        Self::add::<AttributeUsize>(&mut pools);
        Self::add::<Geometry>(&mut pools);
        Self::add::<Mesh>(&mut pools);
        Self::add::<Camera>(&mut pools);

        ResourcePools {
            pools: pools,
        }
    }

    fn add<T: 'static>(pools: &mut HashMap<TypeId, Box<dyn ResourcePoolTrait>>) {
        pools.insert(TypeId::of::<T>(), Box::new(ResourcePool::<T>::new()));
    }

    pub fn borrow<T: 'static>(&self) -> &ResourcePool<T> {
        if let Some(pool) = self.pools.get(&TypeId::of::<T>()) {
            cast_pool(pool.as_ref())
        } else {
            // @TODO: Proper error handling
            // @TODO: Trait bound
            panic!("Unknown Type");
        }
    }

    pub fn borrow_mut<T: 'static>(&mut self) -> &mut ResourcePool<T> {
        if let Some(pool) = self.pools.get_mut(&TypeId::of::<T>()) {
            cast_pool_mut(pool.as_mut())
        } else {
            // @TODO: Proper error handling
            panic!("Unknown Type");
        }
    }

    // @TODO: Write comment
    pub fn borrow_mut_unsafe<T: 'static>(&self) -> &mut ResourcePool<T> {
        if let Some(pool) = self.pools.get(&TypeId::of::<T>()) {
            cast_pool_mut_unsafe(pool)
        } else {
            // @TODO: Proper error handling
            // @TODO: Trait bound
            panic!("Unknown Type");
        }
    }
}

pub struct ResourceId<T> {
    pub id: UId,
    pub is_disposed: bool,
    _phantom: PhantomData<T>,
}

impl<T> ResourceId<T> {
    fn new() -> Self {
        ResourceId {
            id: generate_id(),
            is_disposed: false,
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
