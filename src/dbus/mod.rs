use std::cell::RefCell;
use std::collections::{hash_map, HashMap, HashSet};
use std::fmt;
use std::fmt::Display;
use std::hash::Hash;
use std::marker::PhantomData;
use std::ops::Deref;
use std::rc::Rc;
use std::time::Duration;

use dbus::arg::{PropMap, RefArg};
use dbus::blocking::stdintf::org_freedesktop_dbus::{
    ObjectManager, ObjectManagerInterfacesAdded, ObjectManagerInterfacesRemoved, Properties,
    PropertiesPropertiesChanged,
};
pub use dbus::*;
use thiserror::Error;

use crate::util::Timeout;

#[cfg(test)]
mod test;

pub type Object = HashMap<String, PropMap>;

#[derive(Debug, Error)]
pub struct TypedError {
    #[source]
    pub cause: dbus::Error,
    pub kind: ErrorKind,
}

#[derive(Debug)]
pub enum ErrorKind {
    InvalidArgs,
    AccessDenied,
    NoReply,
    Custom,
}

impl Display for TypedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "D-Bus error: {:?}: {}", self.kind, self.cause)
    }
}

impl From<dbus::Error> for TypedError {
    fn from(cause: dbus::Error) -> Self {
        let kind = if let Some(name) = cause.name() {
            match name {
                "org.freedesktop.DBus.Error.InvalidArgs" => ErrorKind::InvalidArgs,
                "org.freedesktop.DBus.Error.AccessDenied" => ErrorKind::AccessDenied,
                "org.freedesktop.DBus.Error.NoReply" => ErrorKind::NoReply,
                _ => ErrorKind::Custom,
            }
        } else {
            ErrorKind::Custom
        };
        TypedError { cause, kind }
    }
}

pub trait RefArgCast<'a>: Sized {
    fn ref_arg_cast(r: &'a dyn RefArg) -> Result<Self, dbus::Error>;

    fn ref_arg_cast_variant(r: &'a dyn RefArg) -> Result<Self, dbus::Error> {
        Self::ref_arg_cast(if let dbus::arg::ArgType::Variant = r.arg_type() {
            r.as_iter().unwrap().next().unwrap()
        } else {
            r
        })
    }
}

fn cast_error(from: &dyn RefArg, to: &str) -> dbus::Error {
    dbus::Error::new_failed(&format!(
        "Cannot cast from {:?} to {}",
        &from.arg_type(),
        to
    ))
}

impl<'a> RefArgCast<'a> for &'a str {
    fn ref_arg_cast(r: &'a dyn RefArg) -> Result<Self, dbus::Error> {
        r.as_str().ok_or_else(|| cast_error(r, "str"))
    }
}

impl RefArgCast<'_> for String {
    fn ref_arg_cast(r: &dyn RefArg) -> Result<Self, dbus::Error> {
        r.as_str()
            .ok_or_else(|| cast_error(r, "String"))
            .map(str::to_owned)
    }
}

impl RefArgCast<'_> for u8 {
    fn ref_arg_cast(r: &dyn RefArg) -> Result<Self, dbus::Error> {
        r.as_u64()
            .ok_or_else(|| cast_error(r, "u8"))
            .map(|v| v as u8)
    }
}

impl RefArgCast<'_> for i64 {
    fn ref_arg_cast(r: &dyn RefArg) -> Result<Self, dbus::Error> {
        r.as_i64().ok_or_else(|| cast_error(r, "i64"))
    }
}

impl RefArgCast<'_> for u64 {
    fn ref_arg_cast(r: &dyn RefArg) -> Result<Self, dbus::Error> {
        r.as_u64().ok_or_else(|| cast_error(r, "u64"))
    }
}

impl RefArgCast<'_> for f64 {
    fn ref_arg_cast(r: &dyn RefArg) -> Result<Self, dbus::Error> {
        r.as_f64().ok_or_else(|| cast_error(r, "f64"))
    }
}

// impl<'a, V: RefArgCast<'a>> RefArgCast<'a> for Variant<V> {
//     fn ref_arg_cast(r: &'a dyn RefArg) -> Result<Self, dbus::Error> {
//         Ok(Variant(V::ref_arg_cast_variant(
//             r.as_iter()
//                 .ok_or_else(|| cast_error(r, "Variant"))?
//                 .next()
//                 .ok_or_else(|| dbus::Error::new_failed("Unable to get
// contents of Variant"))?,         )?))
//     }
// }

impl<'a, V: RefArgCast<'a>> RefArgCast<'a> for Vec<V> {
    fn ref_arg_cast(r: &'a dyn RefArg) -> Result<Self, dbus::Error> {
        r.as_iter()
            .ok_or_else(|| cast_error(r, "Vec"))
            .and_then(|i| i.map(V::ref_arg_cast_variant).collect())
    }
}

impl<'a, V: RefArgCast<'a> + Eq + Hash> RefArgCast<'a> for HashSet<V> {
    fn ref_arg_cast(r: &'a dyn RefArg) -> Result<Self, dbus::Error> {
        r.as_iter()
            .ok_or_else(|| cast_error(r, "HashSet"))
            .and_then(|i| i.map(V::ref_arg_cast_variant).collect())
    }
}

impl<'a, K: RefArgCast<'a> + Eq + Hash, V: RefArgCast<'a>> RefArgCast<'a> for HashMap<K, V> {
    fn ref_arg_cast(r: &'a dyn RefArg) -> Result<Self, dbus::Error> {
        let mut map = HashMap::<K, V>::new();
        let mut iter = r.as_iter().ok_or_else(|| cast_error(r, "HashMap"))?;
        while let Some(key) = iter.next() {
            let value = iter.next().ok_or_else(|| {
                dbus::Error::new_custom(
                    "Missing dictionary value",
                    "Dictionary does not have value corresponding to key",
                )
            })?;
            map.insert(
                K::ref_arg_cast_variant(key)?,
                V::ref_arg_cast_variant(value)?,
            );
        }
        Ok(map)
    }
}

pub struct RefArgIter<'a, V: RefArgCast<'a>> {
    iter: Box<dyn Iterator<Item = &'a dyn RefArg> + 'a>,
    phantom: PhantomData<&'a V>,
}

impl<'a, V: RefArgCast<'a>> Iterator for RefArgIter<'a, V> {
    type Item = Result<V, dbus::Error>;

    fn next(&mut self) -> Option<Result<V, dbus::Error>> {
        self.iter.next().map(V::ref_arg_cast_variant)
    }
}

impl<'a, V: RefArgCast<'a>> RefArgCast<'a> for RefArgIter<'a, V> {
    fn ref_arg_cast(r: &'a dyn RefArg) -> Result<Self, dbus::Error> {
        Ok(Self {
            iter: r.as_iter().ok_or_else(|| cast_error(r, "Iterator"))?,
            phantom: PhantomData,
        })
    }
}

struct ObjectManagerDatabase {
    pub objects: HashMap<dbus::strings::Path<'static>, Object>,
    queue: HashMap<dbus::strings::Path<'static>, Object>,
}

impl ObjectManagerDatabase {
    pub fn new(objects: HashMap<dbus::strings::Path<'static>, Object>) -> Self {
        Self {
            objects,
            queue: HashMap::new(),
        }
    }

    pub fn add_interfaces(&mut self, object: dbus::strings::Path<'static>, interfaces: Object) {
        self.queue.entry(object).or_default().extend(interfaces);
    }

    pub fn remove_interfaces(
        &mut self,
        object: dbus::strings::Path<'static>,
        interfaces: Vec<String>,
    ) {
        let remove = |objects: &mut HashMap<dbus::strings::Path<'static>, Object>| {
            if let hash_map::Entry::Occupied(mut e) = objects.entry(object.clone()) {
                let obj = e.get_mut();
                interfaces.iter().for_each(|i| {
                    obj.remove(i);
                });
                if obj.is_empty() {
                    e.remove();
                }
            }
        };
        remove(&mut self.objects);
        remove(&mut self.queue);
    }

    pub fn process_queue(&mut self, mut f: impl FnMut(&dbus::strings::Path, &Object)) {
        let queue = std::mem::replace(&mut self.queue, HashMap::new());
        queue.into_iter().for_each(move |(object, interfaces)| {
            let interfaces = match self.objects.entry(object.clone()) {
                hash_map::Entry::Occupied(e) => {
                    let existing_interfaces = e.into_mut();
                    existing_interfaces.extend(interfaces);
                    existing_interfaces
                }
                hash_map::Entry::Vacant(e) => e.insert(interfaces),
            };
            f(&object, interfaces);
        });
    }
}

pub struct ObjectManagerCache<'a, C: Deref<Target = dbus::blocking::LocalConnection>> {
    manager: dbus::blocking::Proxy<'a, C>,
    database: Rc<RefCell<ObjectManagerDatabase>>,
    interfaces_added_token: dbus::channel::Token,
    interfaces_removed_token: dbus::channel::Token,
}

impl<'a, C: Deref<Target = dbus::blocking::LocalConnection>> ObjectManagerCache<'a, C> {
    pub fn new(manager: dbus::blocking::Proxy<'a, C>) -> Result<Self, TypedError> {
        let database = Rc::new(RefCell::new(ObjectManagerDatabase::new(
            manager.get_managed_objects()?,
        )));

        // Subscribe to interfaces added and removed signals
        let interfaces_added_token = {
            let objects = database.clone();
            manager.match_signal(move |h: ObjectManagerInterfacesAdded, _: &_, _: &_| {
                objects.borrow_mut().add_interfaces(h.object, h.interfaces);
                true
            })?
        };
        let interfaces_removed_token = {
            let objects = database.clone();
            manager.match_signal(move |h: ObjectManagerInterfacesRemoved, _: &_, _: &_| {
                objects
                    .borrow_mut()
                    .remove_interfaces(h.object, h.interfaces);
                true
            })?
        };

        Ok(Self {
            manager,
            database,
            interfaces_added_token,
            interfaces_removed_token,
        })
    }

    pub fn find_map_object<T, F: FnMut(&dbus::strings::Path, &Object) -> Option<T>>(
        &self,
        mut f: F,
        timeout: Duration,
    ) -> Result<Option<T>, TypedError> {
        let timeout = Timeout::start(timeout);
        Ok({
            let object = self
                .database
                .borrow()
                .objects
                .iter()
                // Try to find the object in the existing database
                .find_map(|(path, obj)| f(path, obj));
            match object {
                s @ Some(_) => s,
                None => {
                    // If we couldn't find anything, wait for a signal that might contain the
                    // desired object
                    loop {
                        if self.manager.connection.process(timeout.get())? {
                            let mut t = None;
                            self.database.borrow_mut().process_queue(|o, i| t = f(o, i));
                            if t.is_some() {
                                break t;
                            }
                        } else {
                            break None;
                        }
                    }
                }
            }
        })
    }
}

impl<'a, C: Deref<Target = dbus::blocking::LocalConnection>> Drop for ObjectManagerCache<'a, C> {
    fn drop(&mut self) {
        self.manager
            .match_stop(self.interfaces_added_token, true)
            .ok();
        self.manager
            .match_stop(self.interfaces_removed_token, true)
            .ok();
    }
}

pub struct PropertyCache<'a, C: Deref<Target = dbus::blocking::LocalConnection>> {
    proxy: dbus::blocking::Proxy<'a, C>,
    properties: Rc<RefCell<Object>>,
    properties_changed_token: dbus::channel::Token,
}

impl<'a, C: Deref<Target = dbus::blocking::LocalConnection>> PropertyCache<'a, C> {
    pub fn new(proxy: dbus::blocking::Proxy<'a, C>) -> Result<Self, TypedError> {
        let properties = Rc::new(RefCell::new(HashMap::<_, PropMap>::new()));

        // Subscribe to interfaces added and removed signals
        let properties_changed_token = {
            let properties = properties.clone();
            proxy.match_signal(move |h: PropertiesPropertiesChanged, _: &_, _: &_| {
                if let Some(interface) = properties.borrow_mut().get_mut(&h.interface_name) {
                    interface.extend(h.changed_properties);
                    h.invalidated_properties.iter().for_each(|k| {
                        interface.remove(k);
                    });
                }
                true
            })?
        };

        Ok(Self {
            proxy,
            properties,
            properties_changed_token,
        })
    }

    /// Get the value of an interface property for this object. The value will
    /// be retrieved from the cache if available, otherwise it will be queried.
    /// If the property does not exist or is currently unavailable, `Ok(None)`
    /// will be returned.
    pub fn get(
        &self,
        interface_name: &str,
        property_name: &str,
    ) -> Result<Option<Box<dyn dbus::arg::RefArg>>, TypedError> {
        // Process any updates to the cache
        while self.wait_change(Duration::from_millis(0))? {}
        self.properties
            .borrow()
            .get(interface_name)
            .and_then(|interface| interface.get(property_name))
            // Have to clone from cache, should still be faster than DBus call. Ideally Cow could be
            // used but the RefCell API makes that impossible
            .map(|val| Ok(val.0.box_clone()))
            .or_else(|| {
                // The value was not found in the
                match self
                    .proxy
                    .get(interface_name, property_name)
                    .map_err(TypedError::from)
                {
                    // Interface or property was not found
                    Err(TypedError {
                        kind: ErrorKind::InvalidArgs,
                        ..
                    }) => Ok(None),
                    Err(e) => Err(e),
                    Ok(val) => Ok(Some(val)),
                }
                .transpose()
            })
            .transpose()
    }

    pub fn wait_change(&self, timeout: Duration) -> Result<bool, TypedError> {
        Ok(self.proxy.connection.process(timeout)?)
    }
}

impl<'a, C: Deref<Target = dbus::blocking::LocalConnection>> Drop for PropertyCache<'a, C> {
    fn drop(&mut self) {
        self.proxy
            .match_stop(self.properties_changed_token, true)
            .ok();
    }
}
