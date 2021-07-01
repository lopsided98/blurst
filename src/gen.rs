// This code was autogenerated with `dbus-codegen-rust -i org.bluez -c blocking --file src/bluez/bluez.xml -m None -o src/bluez/gen.rs`, see https://github.com/diwic/dbus-rs
use dbus;
#[allow(unused_imports)]
use dbus::arg;
use dbus::blocking;

pub trait AgentManager1 {
    fn register_agent(&self, agent: dbus::Path, capability: &str) -> Result<(), dbus::Error>;
    fn unregister_agent(&self, agent: dbus::Path) -> Result<(), dbus::Error>;
    fn request_default_agent(&self, agent: dbus::Path) -> Result<(), dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: std::ops::Deref<Target = T>> AgentManager1
    for blocking::Proxy<'a, C>
{
    fn register_agent(&self, agent: dbus::Path, capability: &str) -> Result<(), dbus::Error> {
        self.method_call(
            "org.bluez.AgentManager1",
            "RegisterAgent",
            (agent, capability),
        )
    }

    fn unregister_agent(&self, agent: dbus::Path) -> Result<(), dbus::Error> {
        self.method_call("org.bluez.AgentManager1", "UnregisterAgent", (agent,))
    }

    fn request_default_agent(&self, agent: dbus::Path) -> Result<(), dbus::Error> {
        self.method_call("org.bluez.AgentManager1", "RequestDefaultAgent", (agent,))
    }
}

pub trait ProfileManager1 {
    fn register_profile(
        &self,
        profile: dbus::Path,
        uuid: &str,
        options: std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>,
    ) -> Result<(), dbus::Error>;
    fn unregister_profile(&self, profile: dbus::Path) -> Result<(), dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: std::ops::Deref<Target = T>> ProfileManager1
    for blocking::Proxy<'a, C>
{
    fn register_profile(
        &self,
        profile: dbus::Path,
        uuid: &str,
        options: std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>,
    ) -> Result<(), dbus::Error> {
        self.method_call(
            "org.bluez.ProfileManager1",
            "RegisterProfile",
            (profile, uuid, options),
        )
    }

    fn unregister_profile(&self, profile: dbus::Path) -> Result<(), dbus::Error> {
        self.method_call("org.bluez.ProfileManager1", "UnregisterProfile", (profile,))
    }
}

pub trait Adapter1 {
    fn start_discovery(&self) -> Result<(), dbus::Error>;
    fn set_discovery_filter(
        &self,
        properties: std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>,
    ) -> Result<(), dbus::Error>;
    fn stop_discovery(&self) -> Result<(), dbus::Error>;
    fn remove_device(&self, device: dbus::Path) -> Result<(), dbus::Error>;
    fn get_discovery_filters(&self) -> Result<Vec<String>, dbus::Error>;
    fn address(&self) -> Result<String, dbus::Error>;
    fn address_type(&self) -> Result<String, dbus::Error>;
    fn name(&self) -> Result<String, dbus::Error>;
    fn alias(&self) -> Result<String, dbus::Error>;
    fn set_alias(&self, value: String) -> Result<(), dbus::Error>;
    fn class(&self) -> Result<u32, dbus::Error>;
    fn powered(&self) -> Result<bool, dbus::Error>;
    fn set_powered(&self, value: bool) -> Result<(), dbus::Error>;
    fn discoverable(&self) -> Result<bool, dbus::Error>;
    fn set_discoverable(&self, value: bool) -> Result<(), dbus::Error>;
    fn discoverable_timeout(&self) -> Result<u32, dbus::Error>;
    fn set_discoverable_timeout(&self, value: u32) -> Result<(), dbus::Error>;
    fn pairable(&self) -> Result<bool, dbus::Error>;
    fn set_pairable(&self, value: bool) -> Result<(), dbus::Error>;
    fn pairable_timeout(&self) -> Result<u32, dbus::Error>;
    fn set_pairable_timeout(&self, value: u32) -> Result<(), dbus::Error>;
    fn discovering(&self) -> Result<bool, dbus::Error>;
    fn uuids(&self) -> Result<Vec<String>, dbus::Error>;
    fn modalias(&self) -> Result<String, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: std::ops::Deref<Target = T>> Adapter1
    for blocking::Proxy<'a, C>
{
    fn start_discovery(&self) -> Result<(), dbus::Error> {
        self.method_call("org.bluez.Adapter1", "StartDiscovery", ())
    }

    fn set_discovery_filter(
        &self,
        properties: std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>,
    ) -> Result<(), dbus::Error> {
        self.method_call("org.bluez.Adapter1", "SetDiscoveryFilter", (properties,))
    }

    fn stop_discovery(&self) -> Result<(), dbus::Error> {
        self.method_call("org.bluez.Adapter1", "StopDiscovery", ())
    }

    fn remove_device(&self, device: dbus::Path) -> Result<(), dbus::Error> {
        self.method_call("org.bluez.Adapter1", "RemoveDevice", (device,))
    }

    fn get_discovery_filters(&self) -> Result<Vec<String>, dbus::Error> {
        self.method_call("org.bluez.Adapter1", "GetDiscoveryFilters", ())
            .and_then(|r: (Vec<String>,)| Ok(r.0))
    }

    fn address(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.bluez.Adapter1",
            "Address",
        )
    }

    fn address_type(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.bluez.Adapter1",
            "AddressType",
        )
    }

    fn name(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.bluez.Adapter1",
            "Name",
        )
    }

    fn alias(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.bluez.Adapter1",
            "Alias",
        )
    }

    fn class(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.bluez.Adapter1",
            "Class",
        )
    }

    fn powered(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.bluez.Adapter1",
            "Powered",
        )
    }

    fn discoverable(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.bluez.Adapter1",
            "Discoverable",
        )
    }

    fn discoverable_timeout(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.bluez.Adapter1",
            "DiscoverableTimeout",
        )
    }

    fn pairable(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.bluez.Adapter1",
            "Pairable",
        )
    }

    fn pairable_timeout(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.bluez.Adapter1",
            "PairableTimeout",
        )
    }

    fn discovering(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.bluez.Adapter1",
            "Discovering",
        )
    }

    fn uuids(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.bluez.Adapter1",
            "UUIDs",
        )
    }

    fn modalias(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.bluez.Adapter1",
            "Modalias",
        )
    }

    fn set_alias(&self, value: String) -> Result<(), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::set(
            &self,
            "org.bluez.Adapter1",
            "Alias",
            value,
        )
    }

    fn set_powered(&self, value: bool) -> Result<(), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::set(
            &self,
            "org.bluez.Adapter1",
            "Powered",
            value,
        )
    }

    fn set_discoverable(&self, value: bool) -> Result<(), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::set(
            &self,
            "org.bluez.Adapter1",
            "Discoverable",
            value,
        )
    }

    fn set_discoverable_timeout(&self, value: u32) -> Result<(), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::set(
            &self,
            "org.bluez.Adapter1",
            "DiscoverableTimeout",
            value,
        )
    }

    fn set_pairable(&self, value: bool) -> Result<(), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::set(
            &self,
            "org.bluez.Adapter1",
            "Pairable",
            value,
        )
    }

    fn set_pairable_timeout(&self, value: u32) -> Result<(), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::set(
            &self,
            "org.bluez.Adapter1",
            "PairableTimeout",
            value,
        )
    }
}

pub trait LEAdvertisingManager1 {
    fn register_advertisement(
        &self,
        advertisement: dbus::Path,
        options: std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>,
    ) -> Result<(), dbus::Error>;
    fn unregister_advertisement(&self, service: dbus::Path) -> Result<(), dbus::Error>;
    fn active_instances(&self) -> Result<u8, dbus::Error>;
    fn supported_instances(&self) -> Result<u8, dbus::Error>;
    fn supported_includes(&self) -> Result<Vec<String>, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: std::ops::Deref<Target = T>> LEAdvertisingManager1
    for blocking::Proxy<'a, C>
{
    fn register_advertisement(
        &self,
        advertisement: dbus::Path,
        options: std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>,
    ) -> Result<(), dbus::Error> {
        self.method_call(
            "org.bluez.LEAdvertisingManager1",
            "RegisterAdvertisement",
            (advertisement, options),
        )
    }

    fn unregister_advertisement(&self, service: dbus::Path) -> Result<(), dbus::Error> {
        self.method_call(
            "org.bluez.LEAdvertisingManager1",
            "UnregisterAdvertisement",
            (service,),
        )
    }

    fn active_instances(&self) -> Result<u8, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.bluez.LEAdvertisingManager1",
            "ActiveInstances",
        )
    }

    fn supported_instances(&self) -> Result<u8, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.bluez.LEAdvertisingManager1",
            "SupportedInstances",
        )
    }

    fn supported_includes(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.bluez.LEAdvertisingManager1",
            "SupportedIncludes",
        )
    }
}

pub trait Media1 {
    fn register_endpoint(
        &self,
        endpoint: dbus::Path,
        properties: std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>,
    ) -> Result<(), dbus::Error>;
    fn unregister_endpoint(&self, endpoint: dbus::Path) -> Result<(), dbus::Error>;
    fn register_player(
        &self,
        player: dbus::Path,
        properties: std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>,
    ) -> Result<(), dbus::Error>;
    fn unregister_player(&self, player: dbus::Path) -> Result<(), dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: std::ops::Deref<Target = T>> Media1
    for blocking::Proxy<'a, C>
{
    fn register_endpoint(
        &self,
        endpoint: dbus::Path,
        properties: std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>,
    ) -> Result<(), dbus::Error> {
        self.method_call(
            "org.bluez.Media1",
            "RegisterEndpoint",
            (endpoint, properties),
        )
    }

    fn unregister_endpoint(&self, endpoint: dbus::Path) -> Result<(), dbus::Error> {
        self.method_call("org.bluez.Media1", "UnregisterEndpoint", (endpoint,))
    }

    fn register_player(
        &self,
        player: dbus::Path,
        properties: std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>,
    ) -> Result<(), dbus::Error> {
        self.method_call("org.bluez.Media1", "RegisterPlayer", (player, properties))
    }

    fn unregister_player(&self, player: dbus::Path) -> Result<(), dbus::Error> {
        self.method_call("org.bluez.Media1", "UnregisterPlayer", (player,))
    }
}

pub trait NetworkServer1 {
    fn register(&self, uuid: &str, bridge: &str) -> Result<(), dbus::Error>;
    fn unregister(&self, uuid: &str) -> Result<(), dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: std::ops::Deref<Target = T>> NetworkServer1
    for blocking::Proxy<'a, C>
{
    fn register(&self, uuid: &str, bridge: &str) -> Result<(), dbus::Error> {
        self.method_call("org.bluez.NetworkServer1", "Register", (uuid, bridge))
    }

    fn unregister(&self, uuid: &str) -> Result<(), dbus::Error> {
        self.method_call("org.bluez.NetworkServer1", "Unregister", (uuid,))
    }
}

pub trait Device1 {
    fn disconnect(&self) -> Result<(), dbus::Error>;
    fn connect(&self) -> Result<(), dbus::Error>;
    fn connect_profile(&self, uuid: &str) -> Result<(), dbus::Error>;
    fn disconnect_profile(&self, uuid: &str) -> Result<(), dbus::Error>;
    fn pair(&self) -> Result<(), dbus::Error>;
    fn cancel_pairing(&self) -> Result<(), dbus::Error>;
    fn address(&self) -> Result<String, dbus::Error>;
    fn address_type(&self) -> Result<String, dbus::Error>;
    fn name(&self) -> Result<String, dbus::Error>;
    fn alias(&self) -> Result<String, dbus::Error>;
    fn set_alias(&self, value: String) -> Result<(), dbus::Error>;
    fn class(&self) -> Result<u32, dbus::Error>;
    fn appearance(&self) -> Result<u16, dbus::Error>;
    fn icon(&self) -> Result<String, dbus::Error>;
    fn paired(&self) -> Result<bool, dbus::Error>;
    fn trusted(&self) -> Result<bool, dbus::Error>;
    fn set_trusted(&self, value: bool) -> Result<(), dbus::Error>;
    fn blocked(&self) -> Result<bool, dbus::Error>;
    fn set_blocked(&self, value: bool) -> Result<(), dbus::Error>;
    fn legacy_pairing(&self) -> Result<bool, dbus::Error>;
    fn rssi(&self) -> Result<i16, dbus::Error>;
    fn connected(&self) -> Result<bool, dbus::Error>;
    fn uuids(&self) -> Result<Vec<String>, dbus::Error>;
    fn modalias(&self) -> Result<String, dbus::Error>;
    fn adapter(&self) -> Result<dbus::Path<'static>, dbus::Error>;
    fn manufacturer_data(
        &self,
    ) -> Result<
        std::collections::HashMap<u16, arg::Variant<Box<dyn arg::RefArg + 'static>>>,
        dbus::Error,
    >;
    fn service_data(
        &self,
    ) -> Result<
        std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>,
        dbus::Error,
    >;
    fn tx_power(&self) -> Result<i16, dbus::Error>;
    fn services_resolved(&self) -> Result<bool, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: std::ops::Deref<Target = T>> Device1
    for blocking::Proxy<'a, C>
{
    fn disconnect(&self) -> Result<(), dbus::Error> {
        self.method_call("org.bluez.Device1", "Disconnect", ())
    }

    fn connect(&self) -> Result<(), dbus::Error> {
        self.method_call("org.bluez.Device1", "Connect", ())
    }

    fn connect_profile(&self, uuid: &str) -> Result<(), dbus::Error> {
        self.method_call("org.bluez.Device1", "ConnectProfile", (uuid,))
    }

    fn disconnect_profile(&self, uuid: &str) -> Result<(), dbus::Error> {
        self.method_call("org.bluez.Device1", "DisconnectProfile", (uuid,))
    }

    fn pair(&self) -> Result<(), dbus::Error> {
        self.method_call("org.bluez.Device1", "Pair", ())
    }

    fn cancel_pairing(&self) -> Result<(), dbus::Error> {
        self.method_call("org.bluez.Device1", "CancelPairing", ())
    }

    fn address(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.bluez.Device1",
            "Address",
        )
    }

    fn address_type(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.bluez.Device1",
            "AddressType",
        )
    }

    fn name(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.bluez.Device1",
            "Name",
        )
    }

    fn alias(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.bluez.Device1",
            "Alias",
        )
    }

    fn class(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.bluez.Device1",
            "Class",
        )
    }

    fn appearance(&self) -> Result<u16, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.bluez.Device1",
            "Appearance",
        )
    }

    fn icon(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.bluez.Device1",
            "Icon",
        )
    }

    fn paired(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.bluez.Device1",
            "Paired",
        )
    }

    fn trusted(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.bluez.Device1",
            "Trusted",
        )
    }

    fn blocked(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.bluez.Device1",
            "Blocked",
        )
    }

    fn legacy_pairing(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.bluez.Device1",
            "LegacyPairing",
        )
    }

    fn rssi(&self) -> Result<i16, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.bluez.Device1",
            "RSSI",
        )
    }

    fn connected(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.bluez.Device1",
            "Connected",
        )
    }

    fn uuids(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.bluez.Device1",
            "UUIDs",
        )
    }

    fn modalias(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.bluez.Device1",
            "Modalias",
        )
    }

    fn adapter(&self) -> Result<dbus::Path<'static>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.bluez.Device1",
            "Adapter",
        )
    }

    fn manufacturer_data(
        &self,
    ) -> Result<
        std::collections::HashMap<u16, arg::Variant<Box<dyn arg::RefArg + 'static>>>,
        dbus::Error,
    > {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.bluez.Device1",
            "ManufacturerData",
        )
    }

    fn service_data(
        &self,
    ) -> Result<
        std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>,
        dbus::Error,
    > {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.bluez.Device1",
            "ServiceData",
        )
    }

    fn tx_power(&self) -> Result<i16, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.bluez.Device1",
            "TxPower",
        )
    }

    fn services_resolved(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.bluez.Device1",
            "ServicesResolved",
        )
    }

    fn set_alias(&self, value: String) -> Result<(), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::set(
            &self,
            "org.bluez.Device1",
            "Alias",
            value,
        )
    }

    fn set_trusted(&self, value: bool) -> Result<(), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::set(
            &self,
            "org.bluez.Device1",
            "Trusted",
            value,
        )
    }

    fn set_blocked(&self, value: bool) -> Result<(), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::set(
            &self,
            "org.bluez.Device1",
            "Blocked",
            value,
        )
    }
}

pub trait Battery1 {
    fn percentage(&self) -> Result<u8, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: std::ops::Deref<Target = T>> Battery1
    for blocking::Proxy<'a, C>
{
    fn percentage(&self) -> Result<u8, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.bluez.Battery1",
            "Percentage",
        )
    }
}

pub trait GattService1 {
    fn uuid(&self) -> Result<String, dbus::Error>;
    fn device(&self) -> Result<dbus::Path<'static>, dbus::Error>;
    fn primary(&self) -> Result<bool, dbus::Error>;
    fn includes(&self) -> Result<Vec<dbus::Path<'static>>, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: std::ops::Deref<Target = T>> GattService1
    for blocking::Proxy<'a, C>
{
    fn uuid(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.bluez.GattService1",
            "UUID",
        )
    }

    fn device(&self) -> Result<dbus::Path<'static>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.bluez.GattService1",
            "Device",
        )
    }

    fn primary(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.bluez.GattService1",
            "Primary",
        )
    }

    fn includes(&self) -> Result<Vec<dbus::Path<'static>>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.bluez.GattService1",
            "Includes",
        )
    }
}

pub trait GattCharacteristic1 {
    fn read_value(
        &self,
        options: std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>,
    ) -> Result<Vec<u8>, dbus::Error>;
    fn write_value(
        &self,
        value: Vec<u8>,
        options: std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>,
    ) -> Result<(), dbus::Error>;
    fn acquire_write(
        &self,
        options: std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>,
    ) -> Result<(arg::OwnedFd, u16), dbus::Error>;
    fn acquire_notify(
        &self,
        options: std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>,
    ) -> Result<(arg::OwnedFd, u16), dbus::Error>;
    fn start_notify(&self) -> Result<(), dbus::Error>;
    fn stop_notify(&self) -> Result<(), dbus::Error>;
    fn uuid(&self) -> Result<String, dbus::Error>;
    fn service(&self) -> Result<dbus::Path<'static>, dbus::Error>;
    fn value(&self) -> Result<Vec<u8>, dbus::Error>;
    fn notifying(&self) -> Result<bool, dbus::Error>;
    fn flags(&self) -> Result<Vec<String>, dbus::Error>;
    fn write_acquired(&self) -> Result<bool, dbus::Error>;
    fn notify_acquired(&self) -> Result<bool, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: std::ops::Deref<Target = T>> GattCharacteristic1
    for blocking::Proxy<'a, C>
{
    fn read_value(
        &self,
        options: std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>,
    ) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.bluez.GattCharacteristic1", "ReadValue", (options,))
            .and_then(|r: (Vec<u8>,)| Ok(r.0))
    }

    fn write_value(
        &self,
        value: Vec<u8>,
        options: std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>,
    ) -> Result<(), dbus::Error> {
        self.method_call(
            "org.bluez.GattCharacteristic1",
            "WriteValue",
            (value, options),
        )
    }

    fn acquire_write(
        &self,
        options: std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>,
    ) -> Result<(arg::OwnedFd, u16), dbus::Error> {
        self.method_call("org.bluez.GattCharacteristic1", "AcquireWrite", (options,))
    }

    fn acquire_notify(
        &self,
        options: std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>,
    ) -> Result<(arg::OwnedFd, u16), dbus::Error> {
        self.method_call("org.bluez.GattCharacteristic1", "AcquireNotify", (options,))
    }

    fn start_notify(&self) -> Result<(), dbus::Error> {
        self.method_call("org.bluez.GattCharacteristic1", "StartNotify", ())
    }

    fn stop_notify(&self) -> Result<(), dbus::Error> {
        self.method_call("org.bluez.GattCharacteristic1", "StopNotify", ())
    }

    fn uuid(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.bluez.GattCharacteristic1",
            "UUID",
        )
    }

    fn service(&self) -> Result<dbus::Path<'static>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.bluez.GattCharacteristic1",
            "Service",
        )
    }

    fn value(&self) -> Result<Vec<u8>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.bluez.GattCharacteristic1",
            "Value",
        )
    }

    fn notifying(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.bluez.GattCharacteristic1",
            "Notifying",
        )
    }

    fn flags(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.bluez.GattCharacteristic1",
            "Flags",
        )
    }

    fn write_acquired(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.bluez.GattCharacteristic1",
            "WriteAcquired",
        )
    }

    fn notify_acquired(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.bluez.GattCharacteristic1",
            "NotifyAcquired",
        )
    }
}

pub trait GattDescriptor1 {
    fn read_value(
        &self,
        options: std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>,
    ) -> Result<Vec<u8>, dbus::Error>;
    fn write_value(
        &self,
        value: Vec<u8>,
        options: std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>,
    ) -> Result<(), dbus::Error>;
    fn uuid(&self) -> Result<String, dbus::Error>;
    fn characteristic(&self) -> Result<dbus::Path<'static>, dbus::Error>;
    fn value(&self) -> Result<Vec<u8>, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: std::ops::Deref<Target = T>> GattDescriptor1
    for blocking::Proxy<'a, C>
{
    fn read_value(
        &self,
        options: std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>,
    ) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.bluez.GattDescriptor1", "ReadValue", (options,))
            .and_then(|r: (Vec<u8>,)| Ok(r.0))
    }

    fn write_value(
        &self,
        value: Vec<u8>,
        options: std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>,
    ) -> Result<(), dbus::Error> {
        self.method_call("org.bluez.GattDescriptor1", "WriteValue", (value, options))
    }

    fn uuid(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.bluez.GattDescriptor1",
            "UUID",
        )
    }

    fn characteristic(&self) -> Result<dbus::Path<'static>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.bluez.GattDescriptor1",
            "Characteristic",
        )
    }

    fn value(&self) -> Result<Vec<u8>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.bluez.GattDescriptor1",
            "Value",
        )
    }
}
