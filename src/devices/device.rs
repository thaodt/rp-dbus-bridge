use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.freedesktop.NetworkManager.Device")]
trait Device {
    /// Delete method
    fn delete(&self) -> zbus::Result<()>;

    /// Disconnect method
    fn disconnect(&self) -> zbus::Result<()>;

    /// GetAppliedConnection method
    fn get_applied_connection(
        &self,
        flags: u32,
    ) -> zbus::Result<(
        std::collections::HashMap<
            String,
            std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
        >,
        u64,
    )>;

    /// Reapply method
    fn reapply(
        &self,
        connection: std::collections::HashMap<
            &str,
            std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
        >,
        version_id: u64,
        flags: u32,
    ) -> zbus::Result<()>;

    /// StateChanged signal
    #[dbus_proxy(signal, name = "StateChanged")]
    fn state_changed_signal(&self, new_state: u32, old_state: u32, reason: u32)
        -> zbus::Result<()>;

    /// ActiveConnection property
    #[dbus_proxy(property)]
    fn active_connection(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// Autoconnect property
    #[dbus_proxy(property)]
    fn autoconnect(&self) -> zbus::Result<bool>;
    #[dbus_proxy(property)]
    fn set_autoconnect(&self, value: bool) -> zbus::Result<()>;

    /// AvailableConnections property
    #[dbus_proxy(property)]
    fn available_connections(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// Capabilities property
    #[dbus_proxy(property)]
    fn capabilities(&self) -> zbus::Result<u32>;

    /// DeviceType property
    #[dbus_proxy(property)]
    fn device_type(&self) -> zbus::Result<String>;

    /// Dhcp4Config property
    #[dbus_proxy(property)]
    fn dhcp4_config(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// Dhcp6Config property
    #[dbus_proxy(property)]
    fn dhcp6_config(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// Driver property
    #[dbus_proxy(property)]
    fn driver(&self) -> zbus::Result<String>;

    /// DriverVersion property
    #[dbus_proxy(property)]
    fn driver_version(&self) -> zbus::Result<String>;

    /// FirmwareMissing property
    #[dbus_proxy(property)]
    fn firmware_missing(&self) -> zbus::Result<bool>;

    /// FirmwareVersion property
    #[dbus_proxy(property)]
    fn firmware_version(&self) -> zbus::Result<String>;

    /// HwAddress property
    #[dbus_proxy(property)]
    fn hw_address(&self) -> zbus::Result<String>;

    /// Interface property
    #[dbus_proxy(property)]
    fn interface(&self) -> zbus::Result<String>;

    /// InterfaceFlags property
    #[dbus_proxy(property)]
    fn interface_flags(&self) -> zbus::Result<u32>;

    /// Ip4Address property
    #[dbus_proxy(property)]
    fn ip4_address(&self) -> zbus::Result<u32>;

    /// Ip4Config property
    #[dbus_proxy(property)]
    fn ip4_config(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// Ip4Connectivity property
    #[dbus_proxy(property)]
    fn ip4_connectivity(&self) -> zbus::Result<u32>;

    /// Ip6Config property
    #[dbus_proxy(property)]
    fn ip6_config(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// Ip6Connectivity property
    #[dbus_proxy(property)]
    fn ip6_connectivity(&self) -> zbus::Result<u32>;

    /// IpInterface property
    #[dbus_proxy(property)]
    fn ip_interface(&self) -> zbus::Result<String>;

    /// LldpNeighbors property
    #[dbus_proxy(property)]
    fn lldp_neighbors(
        &self,
    ) -> zbus::Result<Vec<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>>;

    /// Managed property
    #[dbus_proxy(property)]
    fn managed(&self) -> zbus::Result<bool>;
    #[dbus_proxy(property)]
    fn set_managed(&self, value: bool) -> zbus::Result<()>;

    /// Metered property
    #[dbus_proxy(property)]
    fn metered(&self) -> zbus::Result<u32>;

    /// Mtu property
    #[dbus_proxy(property)]
    fn mtu(&self) -> zbus::Result<u32>;

    /// NmPluginMissing property
    #[dbus_proxy(property)]
    fn nm_plugin_missing(&self) -> zbus::Result<bool>;

    /// Path property
    #[dbus_proxy(property)]
    fn path(&self) -> zbus::Result<String>;

    /// PhysicalPortId property
    #[dbus_proxy(property)]
    fn physical_port_id(&self) -> zbus::Result<String>;

    /// Ports property
    #[dbus_proxy(property)]
    fn ports(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// Real property
    #[dbus_proxy(property)]
    fn real(&self) -> zbus::Result<bool>;

    /// State property
    #[dbus_proxy(property)]
    fn state(&self) -> zbus::Result<u32>;

    /// StateReason property
    #[dbus_proxy(property)]
    fn state_reason(&self) -> zbus::Result<(u32, u32)>;

    /// Udi property
    #[dbus_proxy(property)]
    fn udi(&self) -> zbus::Result<String>;
}
