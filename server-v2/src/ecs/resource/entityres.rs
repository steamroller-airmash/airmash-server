use crate::ecs::{Entity, EntityDead};
use hibitset::{BitSet, BitSetLike};

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
struct RefCounts {
    cnts: Vec<usize>,
}

impl RefCounts {
    pub fn inc(&mut self, id: u32) {
        if id as usize >= self.cnts.len() {
            self.cnts.resize(id as usize + 1, 0);
        }

        self.cnts[id as usize] += 1;
    }

    pub fn dec(&mut self, id: u32) {
        self.cnts[id as usize] -= 1;
    }

    pub fn get(&self, id: u32) -> usize {
        self.cnts.get(id as usize).copied().unwrap_or(0)
    }
}

#[derive(Default)]
struct EntityResInner {
    // Entities which have not been deleted
    alive: BitSet,
    // Entities which can still be accesses (this is what is_accessible uses)
    allocated: BitSet,
    gens: Vec<u32>,
    counts: Rc<RefCell<RefCounts>>,
}

impl EntityResInner {
    fn create(&mut self) -> Entity {
        let id = match (!&self.allocated).iter().next() {
            Some(id) => id,
            None => panic!("Ran out of Entity IDs!"),
        };

        if id as usize >= self.gens.len() {
            self.gens.resize(id as usize + 1, 0);
        }

        let mut counts = self.counts.borrow_mut();
        debug_assert!(
            counts.get(id) == 0,
            "BUG: Entity ID allocator returned an ID that was already in use"
        );

        self.gens[id as usize] += 1;
        let gen = self.gens[id as usize];

        self.alive.add(id);
        self.allocated.add(id);
        counts.inc(id);

        Entity::new(id, gen)
    }
    fn delete(&mut self, entity: Entity) -> Result<(), EntityDead> {
        if !self.is_accessible(entity) {
            return Err(EntityDead::new(entity));
        }

        if !self.alive.remove(entity.id()) {
            return Err(EntityDead::new(entity));
        }

        let mut counts = self.counts.borrow_mut();
        if counts.get(entity.id()) == 0 {
            unreachable!("BUG: Deleted entity is alive with a refcount of 0");
        }
        counts.dec(entity.id());

        Ok(())
    }
    fn revive(&mut self, entity: Entity) -> Result<(), EntityDead> {
        if !self.is_accessible(entity) {
            return Err(EntityDead::new(entity));
        }

        if !self.alive.add(entity.id()) {
            self.counts.borrow_mut().inc(entity.id());
        }

        Ok(())
    }
    fn forge(&mut self, id: u32) -> Entity {
        Entity::new(id, self.gens.get(id as usize).copied().unwrap_or(0))
    }

    fn is_accessible(&self, entity: Entity) -> bool {
        if !self.allocated.contains(entity.id()) {
            return false;
        }

        self.gens
            .get(entity.id() as usize)
            .map(|&gen| gen == entity.gen())
            .unwrap_or(false)
    }
    fn is_zombie(&self, entity: Entity) -> bool {
        self.is_accessible(entity) && !self.alive.contains(entity.id())
    }
    fn is_alive(&self, entity: Entity) -> bool {
        self.alive.contains(entity.id())
    }

    fn borrow(&mut self, entity: Entity) -> Result<EntityRef, EntityDead> {
        if !self.is_accessible(entity) {
            return Err(EntityDead::new(entity));
        }

        self.counts.borrow_mut().inc(entity.id());

        Ok(EntityRef {
            counts: self.counts.clone(),
            ent: entity,
        })
    }

    fn gc(&mut self) -> BitSet {
        let candidates = &self.allocated & !&self.alive;
        let counts = self.counts.borrow_mut();
        let mut output = BitSet::new();

        for id in candidates {
            if counts.get(id) != 0 {
                continue;
            }

            output.add(id);
            self.gens[id as usize] += 1;
        }

        self.allocated &= &!&output;

        output
    }
}

pub(crate) struct EntityRes(RefCell<EntityResInner>);

impl EntityRes {
    pub fn new() -> Self {
        Self(RefCell::new(EntityResInner::default()))
    }

    pub fn create(&self) -> Entity {
        self.0.borrow_mut().create()
    }

    /// Returns a boolean indicating whether the item was
    /// actually deleted
    pub fn delete(&self, ent: Entity) -> Result<(), EntityDead> {
        self.0.borrow_mut().delete(ent)
    }
    pub fn revive(&self, ent: Entity) -> Result<(), EntityDead> {
        self.0.borrow_mut().revive(ent)
    }

    pub fn forge(&self, id: u32) -> Entity {
        self.0.borrow_mut().forge(id)
    }

    pub fn borrow(&self, ent: Entity) -> Result<EntityRef, EntityDead> {
        self.0.borrow_mut().borrow(ent)
    }

    pub fn is_accessible(&self, ent: Entity) -> bool {
        self.0.borrow().is_accessible(ent)
    }
    pub fn is_zombie(&self, ent: Entity) -> bool {
        self.0.borrow().is_zombie(ent)
    }
    pub fn is_alive(&self, ent: Entity) -> bool {
        self.0.borrow().is_alive(ent)
    }

    pub fn gc(&mut self) -> BitSet {
        self.0.get_mut().gc()
    }
}

pub struct EntityRef {
    counts: Rc<RefCell<RefCounts>>,
    ent: Entity,
}

impl EntityRef {
    pub fn entity(&self) -> Entity {
        self.ent
    }
}

impl Clone for EntityRef {
    fn clone(&self) -> Self {
        self.counts.borrow_mut().inc(self.ent.id());

        Self {
            counts: self.counts.clone(),
            ent: self.ent,
        }
    }
}

impl Drop for EntityRef {
    fn drop(&mut self) {
        self.counts.borrow_mut().dec(self.ent.id());
    }
}
