use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::time::Duration;

use gen::*;
use thiserror::Error;
use uuid::Uuid;

use crate::dbus::{ObjectManagerCache, RefArgCast, RefArgIter};

mod dbus;
mod gen;
mod util;

pub type DBusProxy = dbus::blocking::Proxy<'static, Rc<dbus::blocking::LocalConnection>>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{kind:?}: {cause}")]
    Bluez {
        #[source]
        cause: dbus::TypedError,
        kind: ErrorKind,
    },
    #[error("{0}")]
    DBus(dbus::TypedError),
    #[error("object missing interface: {0}")]
    MissingInterface(&'static str),
    #[error("missing property: {interface}.{property}")]
    MissingProperty {
        interface: &'static str,
        property: &'static str,
    },
    #[error("UUID error: {0}")]
    Uuid(#[from] uuid::Error),
}

#[derive(Debug)]
pub enum ErrorKind {
    InvalidArguments,
    InProgress,
    AlreadyExists,
    NotSupported,
    NotConnected,
    AlreadyConnected,
    NotAvailable,
    DoesNotExist,
    NotAuthorized,
    NotPermitted,
    NoSuchAdapter,
    AgentNotAvailable,
    NotReady,
    Failed,
    InvalidValueLength,
    InvalidOffset,
    Rejected,
    Canceled,
    AuthenticationCanceled,
    AuthenticationFailed,
    AuthenticationRejected,
    AuthenticationTimeout,
    ConnectionAttemptFailed,
    OutOfRange,
    HealthError,
    NotAcquired,
}

impl From<dbus::TypedError> for Error {
    fn from(cause: dbus::TypedError) -> Self {
        match cause.kind {
            dbus::ErrorKind::Custom => {
                if let Some(name) = cause.cause.name() {
                    let kind = match name {
                        "org.bluez.Error.InvalidArguments" => ErrorKind::InvalidArguments,
                        "org.bluez.Error.InProgress" => ErrorKind::InProgress,
                        "org.bluez.Error.AlreadyExists" => ErrorKind::AlreadyExists,
                        "org.bluez.Error.NotSupported" => ErrorKind::NotSupported,
                        "org.bluez.Error.NotConnected" => ErrorKind::NotConnected,
                        "org.bluez.Error.AlreadyConnected" => ErrorKind::AlreadyConnected,
                        "org.bluez.Error.NotAvailable" => ErrorKind::NotAvailable,
                        "org.bluez.Error.DoesNotExist" => ErrorKind::DoesNotExist,
                        "org.bluez.Error.NotAuthorized" => ErrorKind::NotAuthorized,
                        "org.bluez.Error.NotPermitted" => ErrorKind::NotPermitted,
                        "org.bluez.Error.NoSuchAdapter" => ErrorKind::NoSuchAdapter,
                        "org.bluez.Error.AgentNotAvailable" => ErrorKind::AgentNotAvailable,
                        "org.bluez.Error.NotReady" => ErrorKind::NotReady,
                        "org.bluez.Error.Failed" => ErrorKind::Failed,
                        "org.bluez.Error.InvalidValueLength" => ErrorKind::InvalidValueLength,
                        "org.bluez.Error.InvalidOffset" => ErrorKind::InvalidOffset,
                        "org.bluez.Error.Rejected" => ErrorKind::Rejected,
                        "org.bluez.Error.Canceled" => ErrorKind::Canceled,
                        "org.bluez.Error.AuthenticationCanceled" => {
                            ErrorKind::AuthenticationCanceled
                        }
                        "org.bluez.Error.AuthenticationFailed" => ErrorKind::AuthenticationFailed,
                        "org.bluez.Error.AuthenticationRejected" => {
                            ErrorKind::AuthenticationRejected
                        }
                        "org.bluez.Error.AuthenticationTimeout" => ErrorKind::AuthenticationTimeout,
                        "org.bluez.Error.ConnectionAttemptFailed" => {
                            ErrorKind::ConnectionAttemptFailed
                        }
                        "org.bluez.Error.OutOfRange" => ErrorKind::OutOfRange,
                        "org.bluez.Error.HealthError" => ErrorKind::HealthError,
                        "org.bluez.Error.NotAcquired" => ErrorKind::NotAcquired,
                        _ => return Error::DBus(cause),
                    };
                    Error::Bluez { cause, kind }
                } else {
                    Error::DBus(cause)
                }
            }
            _ => Error::DBus(cause),
        }
    }
}

impl From<dbus::Error> for Error {
    fn from(cause: dbus::Error) -> Self {
        dbus::TypedError::from(cause).into()
    }
}

pub struct Bluez {
    connection: Rc<dbus::blocking::LocalConnection>,
    objects: ObjectManagerCache<'static, Rc<dbus::blocking::LocalConnection>>,
}

impl Bluez {
    const BUS_NAME: &'static str = "org.bluez";

    pub fn new(timeout: Duration) -> Result<Self, Error> {
        let connection = Rc::new(dbus::blocking::LocalConnection::new_system()?);

        let bus_name = dbus::strings::BusName::from(Self::BUS_NAME);
        let root_path = dbus::strings::Path::from("/");

        Ok(Self {
            connection: connection.clone(),
            objects: ObjectManagerCache::new(dbus::blocking::Proxy {
                connection,
                destination: bus_name,
                path: root_path,
                timeout,
            })?,
        })
    }

    fn with_proxy(
        &self,
        path: impl Into<dbus::strings::Path<'static>>,
        timeout: Duration,
    ) -> DBusProxy {
        dbus::blocking::Proxy {
            connection: self.connection.clone(),
            destination: Self::BUS_NAME.into(),
            path: path.into(),
            timeout,
        }
    }

    /// Convenience function to find a BlueZ DBus object using a predicate and
    /// then map it into a wrapper type.
    fn find_map_object<T>(
        &self,
        pred: impl Fn(&dbus::strings::Path, &dbus::Object) -> Result<Option<T>, Error>,
        timeout: Duration,
    ) -> Result<Option<T>, Error> {
        self.objects
            .find_map_object::<Result<T, Error>, _>(
                |object, interfaces| pred(object, interfaces).transpose(),
                timeout,
            )?
            .transpose()
    }

    /// Convenience function to find a BlueZ DBus object using a predicate and
    /// then map it into a wrapper type.
    fn find_map_interface_object<T>(
        &self,
        interface: &'static str,
        pred: impl Fn(&dbus::strings::Path, &dbus::arg::PropMap) -> Result<Option<T>, Error>,
        timeout: Duration,
    ) -> Result<Option<T>, Error> {
        self.find_map_object(
            |object, interfaces| {
                interfaces
                    .get(interface)
                    .and_then(|p| pred(object, p).transpose())
                    .transpose()
            },
            timeout,
        )
    }

    pub fn get_first_adapter(
        self: Rc<Self>,
        adapter_timeout: Duration,
        timeout: Duration,
    ) -> Result<Option<Adapter>, Error> {
        self.find_map_interface_object(
            Adapter::INTERFACE,
            |object, _| {
                Ok(Some(Adapter::new(
                    self.clone(),
                    self.with_proxy(object.clone().into_static(), adapter_timeout),
                )))
            },
            timeout,
        )
    }
}

pub struct Adapter {
    bluez: Rc<Bluez>,
    adapter: DBusProxy,
}

impl Adapter {
    const INTERFACE: &'static str = "org.bluez.Adapter1";

    pub fn new(bluez: Rc<Bluez>, adapter: DBusProxy) -> Self {
        Self { bluez, adapter }
    }

    pub fn start_discovery(&self) -> Result<(), Error> {
        Ok(self.adapter.start_discovery()?)
    }

    pub fn stop_discovery(&self) -> Result<(), Error> {
        Ok(self.adapter.stop_discovery()?)
    }

    pub fn powered(&self) -> Result<bool, Error> {
        Ok(self.adapter.powered()?)
    }

    pub fn set_powered(&self, on: bool) -> Result<(), Error> {
        Ok(self.adapter.set_powered(on)?)
    }

    pub fn find_device(
        &self,
        f: impl Fn(&dbus::arg::PropMap) -> Result<bool, Error>,
        device_timeout: Duration,
        timeout: Duration,
    ) -> Result<Option<Device>, Error> {
        self.bluez.find_map_interface_object(
            Device::INTERFACE,
            |object, interface| {
                Ok(if f(interface)? {
                    Some(Device::new(
                        self.bluez.clone(),
                        self.bluez
                            .with_proxy(object.clone().into_static(), device_timeout),
                    )?)
                } else {
                    None
                })
            },
            timeout,
        )
    }

    /// Find devices satisfying the specified predicate over their properties.
    /// The predicate is allowed to return an error, in which case the
    /// search will stop and the error will be returned.
    pub fn find_devices(
        &self,
        f: impl Fn(&dbus::arg::PropMap) -> Result<bool, Error>,
        device_timeout: Duration,
        timeout: Duration,
    ) -> Result<Vec<Device>, Error> {
        let mut devices = vec![];
        self.bluez
            .objects
            .find_map_object(
                |path, interfaces| {
                    interfaces
                        .get(Device::INTERFACE)
                        .and_then(|p| match f(p) {
                            Ok(true) => Some(Ok(p)),
                            Ok(false) => None,
                            Err(e) => Some(Err(e)),
                        })
                        .and_then(|r| {
                            r.and_then(|_| {
                                devices.push(Device::new(
                                    self.bluez.clone(),
                                    self.bluez
                                        .with_proxy(path.clone().into_static(), device_timeout),
                                )?);
                                Ok(())
                            })
                            .err()
                        })
                },
                timeout,
            )?
            .map_or(Ok(devices), Err)
    }

    pub fn get_devices(
        &self,
        device_timeout: Duration,
        timeout: Duration,
    ) -> Result<Vec<Device>, Error> {
        self.find_devices(|_| Ok(true), device_timeout, timeout)
    }

    pub fn find_device_by_address(
        &self,
        address: &str,
        device_timeout: Duration,
        timeout: Duration,
    ) -> Result<Option<Device>, Error> {
        self.find_device(
            |p| {
                p.get("Address")
                    .ok_or(Error::MissingProperty {
                        interface: Device::INTERFACE,
                        property: "Address",
                    })
                    .and_then(|a| Ok(<&str>::ref_arg_cast(a)?))
                    .map(|a| a == address)
            },
            device_timeout,
            timeout,
        )
    }

    pub fn find_devices_by_uuids(
        &self,
        f: impl Fn(HashSet<Uuid>) -> bool,
        device_timeout: Duration,
        timeout: Duration,
    ) -> Result<Vec<Device>, Error> {
        self.find_devices(
            |p| {
                p.get("UUIDs")
                    .map(|u| -> Result<RefArgIter<&str>, Error> {
                        Ok(RefArgIter::ref_arg_cast(&u.0)?)
                    })
                    .map_or(Ok(false), |r| {
                        r.map(|i| {
                            i.map(|r| r.map_err(Error::from).and_then(|u| Ok(Uuid::parse_str(u)?)))
                        })
                        .and_then(|i| i.collect::<Result<HashSet<_>, _>>())
                        .map(&f)
                    })
            },
            device_timeout,
            timeout,
        )
    }

    pub fn find_devices_with_uuid(
        &self,
        uuid: &Uuid,
        device_timeout: Duration,
        timeout: Duration,
    ) -> Result<Vec<Device>, Error> {
        self.find_devices_by_uuids(
            |uuids| uuids.contains(uuid),
            device_timeout,
            timeout,
        )
    }

    pub fn find_devices_with_uuids(
        &self,
        uuids: &HashSet<Uuid>,
        device_timeout: Duration,
        timeout: Duration,
    ) -> Result<Vec<Device>, Error> {
        self.find_devices_by_uuids(
            |u| !u.is_disjoint(uuids),
            device_timeout,
            timeout,
        )
    }

    pub fn find_devices_with_all_uuids(
        &self,
        uuids: &HashSet<Uuid>,
        device_timeout: Duration,
        timeout: Duration,
    ) -> Result<Vec<Device>, Error> {
        self.find_devices_by_uuids(
            |u| u.is_superset(uuids),
            device_timeout,
            timeout,
        )
    }
}

pub struct Device {
    bluez: Rc<Bluez>,
    device: DBusProxy,
    properties: dbus::PropertyCache<'static, Rc<dbus::blocking::LocalConnection>>,
}

impl Device {
    const INTERFACE: &'static str = "org.bluez.Device1";

    pub fn new(bluez: Rc<Bluez>, device: DBusProxy) -> Result<Self, Error> {
        let properties = dbus::PropertyCache::new(device.clone())?;
        Ok(Self {
            bluez,
            device,
            properties,
        })
    }

    pub fn connect(&self) -> Result<(), Error> {
        Ok(self.device.connect()?)
    }

    pub fn disconnect(&self) -> Result<(), Error> {
        Ok(self.device.disconnect()?)
    }

    pub fn name(&self) -> Result<String, Error> {
        Ok(<String>::ref_arg_cast(
            &self
                .properties
                .get(Self::INTERFACE, "Name")?
                .ok_or(Error::MissingProperty {
                    interface: Self::INTERFACE,
                    property: "Name",
                })?,
        )?)
    }

    pub fn uuids(&self) -> Result<HashSet<Uuid>, Error> {
        Ok(Device1::uuids(&self.device)?
            .into_iter()
            .map(|u| Uuid::parse_str(&u))
            .collect::<Result<_, _>>()?)
    }

    pub fn address(&self) -> Result<String, Error> {
        Ok(Device1::address(&self.device)?)
    }

    pub fn paired(&self) -> Result<bool, Error> {
        Ok(self.device.paired()?)
    }

    /// Get the service data from the most recent advertisement. If no service
    /// data is available from BlueZ, and empty map will be returned.
    pub fn service_data(&self) -> Result<HashMap<Uuid, Vec<u8>>, Error> {
        Ok(self
            .properties
            .get(Self::INTERFACE, "ServiceData")?
            .map(|ref_arg| -> Result<_, Error> {
                <HashMap<String, Vec<u8>>>::ref_arg_cast(ref_arg.as_ref())?
                    .into_iter()
                    .map(|(uuid, buf)| {
                        let uuid = Uuid::parse_str(&uuid)?;
                        Ok((uuid, buf))
                    })
                    .collect()
            })
            .transpose()?
            .unwrap_or_default())
    }

    pub fn rssi(&self) -> Result<i16, Error> {
        Ok(self.device.rssi()?)
    }

    /// Get the battery interface for this device. If the battery interface is
    /// not available, this method will wait up to the timeout for it to appear.
    pub fn battery(
        &self,
        battery_timeout: Duration,
        timeout: Duration,
    ) -> Result<Option<Battery>, Error> {
        self.bluez.find_map_object(
            |object, interfaces| {
                Ok(
                    (object == &self.device.path && interfaces.contains_key(Battery::INTERFACE))
                        .then(|| {
                            Battery::new(
                                self.bluez
                                    .with_proxy(object.clone().into_static(), battery_timeout),
                            )
                        }),
                )
            },
            timeout,
        )
    }

    /// Block until the a device property changes, or the timeout expires.
    /// Returns `true` if a property was updated, or `false` if the timeout
    /// expired without an update.
    pub fn wait_property_change(&self, timeout: Duration) -> Result<bool, Error> {
        Ok(self.properties.wait_change(timeout)?)
    }

    pub fn find_service(
        &self,
        f: impl Fn(&dbus::arg::PropMap) -> Result<bool, Error>,
        service_timeout: Duration,
        timeout: Duration,
    ) -> Result<Option<GattService>, Error> {
        self.bluez.find_map_interface_object(
            GattService::INTERFACE,
            |object, interface| {
                if interface
                    .get("Device")
                    .ok_or(Error::MissingProperty {
                        interface: GattService::INTERFACE,
                        property: "Device",
                    })
                    .and_then(|d| Ok(<&str>::ref_arg_cast(d)?))
                    .map(dbus::Path::from)
                    .map(|device| device == self.device.path)?
                    && f(interface)?
                {
                    Ok(Some(GattService::new(
                        self.bluez.clone(),
                        self.bluez
                            .with_proxy(object.clone().into_static(), service_timeout),
                    )))
                } else {
                    Ok(None)
                }
            },
            timeout,
        )
    }

    pub fn find_service_by_uuid(
        &self,
        uuid: &Uuid,
        service_timeout: Duration,
        timeout: Duration,
    ) -> Result<Option<GattService>, Error> {
        self.find_service(
            |p| {
                p.get("UUID")
                    .ok_or(Error::MissingProperty {
                        interface: GattService::INTERFACE,
                        property: "UUID",
                    })
                    .and_then(|u| Ok(<&str>::ref_arg_cast(u)?))
                    .and_then(|u| Ok(Uuid::parse_str(u)?))
                    .map(|u| u == *uuid)
            },
            service_timeout,
            timeout,
        )
    }
}

pub struct GattService {
    bluez: Rc<Bluez>,
    service: DBusProxy,
}

impl GattService {
    const INTERFACE: &'static str = "org.bluez.GattService1";

    pub fn new(bluez: Rc<Bluez>, service: DBusProxy) -> Self {
        Self { bluez, service }
    }

    pub fn find_characteristic(
        &self,
        f: impl Fn(&dbus::arg::PropMap) -> Result<bool, Error>,
        characteristic_timeout: Duration,
        timeout: Duration,
    ) -> Result<Option<GattCharacteristic>, Error> {
        self.bluez.find_map_interface_object(
            GattCharacteristic::INTERFACE,
            |object, interface| {
                if interface
                    .get("Service")
                    .ok_or(Error::MissingProperty {
                        interface: GattCharacteristic::INTERFACE,
                        property: "Service",
                    })
                    .and_then(|s| Ok(<&str>::ref_arg_cast(s)?))
                    .map(|s| dbus::Path::from(s) == self.service.path)?
                    && f(interface)?
                {
                    Ok(Some(GattCharacteristic::new(self.bluez.with_proxy(
                        object.clone().into_static(),
                        characteristic_timeout,
                    ))))
                } else {
                    Ok(None)
                }
            },
            timeout,
        )
    }

    pub fn find_characteristic_by_uuid(
        &self,
        uuid: &Uuid,
        characteristic_timeout: Duration,
        timeout: Duration,
    ) -> Result<Option<GattCharacteristic>, Error> {
        self.find_characteristic(
            |p| {
                p.get("UUID")
                    .ok_or(Error::MissingProperty {
                        interface: GattCharacteristic::INTERFACE,
                        property: "UUID",
                    })
                    .and_then(|u| Ok(<&str>::ref_arg_cast(u)?))
                    .and_then(|u| Ok(Uuid::parse_str(u)?))
                    .map(|u| u == *uuid)
            },
            characteristic_timeout,
            timeout,
        )
    }
}

pub struct GattCharacteristic {
    characteristic: DBusProxy,
}

impl GattCharacteristic {
    const INTERFACE: &'static str = "org.bluez.GattCharacteristic1";

    pub fn new(characteristic: DBusProxy) -> Self {
        Self { characteristic }
    }

    pub fn acquire_notify(&self) -> Result<(dbus::arg::OwnedFd, u16), Error> {
        Ok(self.characteristic.acquire_notify(HashMap::new())?)
    }

    pub fn acquire_write(&self) -> Result<(dbus::arg::OwnedFd, u16), Error> {
        Ok(self.characteristic.acquire_write(HashMap::new())?)
    }

    pub fn read_value(&self) -> Result<Vec<u8>, Error> {
        Ok(GattCharacteristic1::read_value(
            &self.characteristic,
            HashMap::new(),
        )?)
    }

    pub fn start_notify(&self) -> Result<(), Error> {
        Ok(self.characteristic.start_notify()?)
    }

    pub fn stop_notify(&self) -> Result<(), Error> {
        Ok(self.characteristic.stop_notify()?)
    }

    pub fn write_value(&self, buf: Vec<u8>) -> Result<(), Error> {
        Ok(GattCharacteristic1::write_value(
            &self.characteristic,
            buf,
            HashMap::new(),
        )?)
    }
}

pub struct Battery {
    battery: DBusProxy,
}

impl Battery {
    const INTERFACE: &'static str = "org.bluez.Battery1";

    pub fn new(battery: DBusProxy) -> Self {
        Self { battery }
    }

    pub fn percentage(&self) -> Result<u8, Error> {
        Ok(self.battery.percentage()?)
    }
}
