use std::marker::PhantomData;

use cosmwasm_std::{Empty, Order, StdResult, Storage};
use cw_storage_plus::{Bound, Key, KeyDeserialize, Path, Prefix, PrimaryKey};

/// A set of non-duplicate items.
///
/// This implementation is equivalent to storing these items as keys in a `Map<T, Empty>`.
pub struct Set<'a, T> {
    namespace: &'a [u8],
    item_type: PhantomData<T>,
}

impl<'a, T> Set<'a, T> {
    pub const fn new(namespace: &'a str) -> Self {
        Set {
            namespace: namespace.as_bytes(),
            item_type: PhantomData,
        }
    }

    pub fn namespace(&self) -> &'a [u8] {
        self.namespace
    }
}

impl<'a, T> Set<'a, T>
where
    T: PrimaryKey<'a> + KeyDeserialize,
{
    /// Returns the key for storing an item
    ///
    /// This is copied from
    /// https://github.com/CosmWasm/cw-plus/blob/v0.14.0/packages/storage-plus/src/map.rs#L47-L52
    pub fn key(&self, item: T) -> Path<Empty> {
        Path::new(
            self.namespace,
            &item.key().iter().map(Key::as_ref).collect::<Vec<_>>(),
        )
    }

    /// Returns `true` if the set contains an item
    pub fn contains(&self, store: &dyn Storage, item: T) -> bool {
        self.key(item).has(store)
    }

    /// Adds an item to the set. Returns whether the item was newly added.
    pub fn insert(&mut self, store: &mut dyn Storage, item: T) -> StdResult<bool> {
        let key = self.key(item);
        let new = if key.has(store) {
            false
        } else {
            key.save(store, &Empty {})?;
            true
        };
        Ok(new)
    }

    /// Remove an item from the set. Returns whether the item was present in the set.
    pub fn remove(&mut self, store: &mut dyn Storage, item: T) -> StdResult<bool> {
        let key = self.key(item);
        let existed = if key.has(store) {
            key.remove(store);
            true
        } else {
            false
        };
        Ok(existed)
    }

    /// Iterates items in the set with the specified bounds and ordering.
    pub fn items<'c>(
        &self,
        store: &'c dyn Storage,
        min: Option<Bound<'a, T>>,
        max: Option<Bound<'a, T>>,
        order: Order,
    ) -> Box<dyn Iterator<Item = StdResult<T::Output>> + 'c>
    where
        T::Output: 'static,
    {
        Prefix::<T, Empty, T>::new(self.namespace, &[]).keys(store, min, max, order)
    }
}