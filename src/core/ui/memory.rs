
use std::{any::{Any, TypeId}, collections::HashMap};

use super::UITree;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Id(pub(crate) u64);

pub struct Memory {
    focused: Option<Id>,
    memory: HashMap<(Id, TypeId), Box<dyn Any>>
}

impl Memory {

    pub(crate) fn new() -> Self {
        Self {
            focused: None,
            memory: HashMap::new(),
        }
    }

    /// Focus on a node
    pub fn request_focus(&mut self, node: Id) {
        self.focused = Some(node);
    }

    /// Release focus
    pub fn release_focus(&mut self) {
        self.focused = None;
    }

    /// Check if a node focused 
    pub fn is_focused(&self, node: Id) -> bool {
        self.focused == Some(node)
    }

    /// Get the focused node
    /// Returns `None` if no node is focused
    pub fn get_focus(&self) -> Option<Id> {
        self.focused
    }

    /// Get a reference to the data of type `T` for a certain node.
    /// Sets node's data to `T::default()` if it didn't previously exist.
    pub fn get<T: Default + Any>(&mut self, id: Id) -> &mut T {
        let key = (id, TypeId::of::<T>());
        if !self.memory.contains_key(&key) {
            self.memory.insert(key, Box::new(T::default()));
        }
        self.memory.get_mut(&key).unwrap().downcast_mut().unwrap()
    }

    /// Get a reference to the data of type `T` for a certain node if it exists.
    pub fn get_opt<T: Any>(&mut self, id: Id) -> Option<&mut T> {
        let key = (id, TypeId::of::<T>());
        self.memory.get_mut(&key).map(|data| data.downcast_mut().unwrap())
    }

    /// Insert the data for a certain node.
    pub fn insert<T: Any>(&mut self, id: Id, data: T) {
        let key = (id, TypeId::of::<T>());
        self.memory.insert(key, Box::new(data));
    }

    /// Remove the data of type `T` for a certain node. Returns the data.
    pub fn remove<T: Any>(&mut self, id: Id) -> Option<T> {
        let key = (id, TypeId::of::<T>());
        self.memory.remove(&key).map(|data| *data.downcast().unwrap())
    }

    /// Does a node have data of a certain type in memory?
    pub fn has<T: Default + Any>(&self, id: Id) -> bool {
        let key = (id, TypeId::of::<T>());
        self.memory.contains_key(&key)
    }

    /// Iterate through all data in memory of type `T`
    pub fn iter_mut<T: Default + Any>(&mut self) -> impl Iterator<Item = (Id, &mut T)> {
        let typ_id = TypeId::of::<T>();
        self.memory.iter_mut().filter_map(move |((id, typ), data)| if *typ == typ_id {
            Some((*id, data.downcast_mut().unwrap()))
        } else {
            None
        })
    }

    /// Remove memory for all ui nodes that are not in the ui tree.
    pub(crate) fn garbage_collect(&mut self, tree: &UITree) {
        let mut live_nodes = ahash::AHashSet::new();
        for node in &tree.nodes {
            live_nodes.insert(node.id);
        }

        self.memory.retain(|(id, _), _| live_nodes.contains(id));
    }

}
