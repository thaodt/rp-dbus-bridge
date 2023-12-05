use zbus::dbus_proxy;

pub type NetworkManager<'a> = NetworkManagerProxy<'a>;

#[dbus_proxy(interface = "org.freedesktop.NetworkManager")]
trait NetworkManager {
    /// ActivateConnection method
    fn activate_connection(
        &self,
        connection: &zbus::zvariant::ObjectPath<'_>,
        device: &zbus::zvariant::ObjectPath<'_>,
        specific_object: &zbus::zvariant::ObjectPath<'_>,
    ) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// AddAndActivateConnection method
    fn add_and_activate_connection(
        &self,
        connection: std::collections::HashMap<
            &str,
            std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
        >,
        device: &zbus::zvariant::ObjectPath<'_>,
        specific_object: &zbus::zvariant::ObjectPath<'_>,
    ) -> zbus::Result<(
        zbus::zvariant::OwnedObjectPath,
        zbus::zvariant::OwnedObjectPath,
    )>;

    /// AddAndActivateConnection2 method
    fn add_and_activate_connection2(
        &self,
        connection: std::collections::HashMap<
            &str,
            std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
        >,
        device: &zbus::zvariant::ObjectPath<'_>,
        specific_object: &zbus::zvariant::ObjectPath<'_>,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<(
        zbus::zvariant::OwnedObjectPath,
        zbus::zvariant::OwnedObjectPath,
        std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
    )>;

    /// CheckConnectivity method
    fn check_connectivity(&self) -> zbus::Result<u32>;

    /// CheckpointAdjustRollbackTimeout method
    fn checkpoint_adjust_rollback_timeout(
        &self,
        checkpoint: &zbus::zvariant::ObjectPath<'_>,
        add_timeout: u32,
    ) -> zbus::Result<()>;

    /// CheckpointCreate method
    fn checkpoint_create(
        &self,
        devices: &[zbus::zvariant::ObjectPath<'_>],
        rollback_timeout: u32,
        flags: u32,
    ) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// CheckpointDestroy method
    fn checkpoint_destroy(&self, checkpoint: &zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;

    /// CheckpointRollback method
    fn checkpoint_rollback(
        &self,
        checkpoint: &zbus::zvariant::ObjectPath<'_>,
    ) -> zbus::Result<std::collections::HashMap<String, u32>>;

    /// DeactivateConnection method
    fn deactivate_connection(
        &self,
        active_connection: &zbus::zvariant::ObjectPath<'_>,
    ) -> zbus::Result<()>;

    /// Enable method
    fn enable(&self, enable: bool) -> zbus::Result<()>;

    /// GetAllDevices method
    fn get_all_devices(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// GetDeviceByIpIface method
    fn get_device_by_ip_iface(&self, iface: &str) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// GetDevices method
    fn get_devices(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// GetLogging method
    fn get_logging(&self) -> zbus::Result<(String, String)>;

    /// GetPermissions method
    fn get_permissions(&self) -> zbus::Result<std::collections::HashMap<String, String>>;

    /// Reload method
    fn reload(&self, flags: u32) -> zbus::Result<()>;

    /// SetLogging method
    fn set_logging(&self, level: &str, domains: &str) -> zbus::Result<()>;

    /// Sleep method
    fn sleep(&self, sleep: bool) -> zbus::Result<()>;

    /// state method
    fn state(&self) -> zbus::Result<u32>;

    /// CheckPermissions signal
    #[dbus_proxy(signal)]
    fn check_permissions(&self) -> zbus::Result<()>;

    /// DeviceAdded signal
    #[dbus_proxy(signal)]
    fn device_added(&self, device_path: zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;

    /// DeviceRemoved signal
    #[dbus_proxy(signal)]
    fn device_removed(&self, device_path: zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;

    /// StateChanged signal
    #[dbus_proxy(signal, name = "StateChanged")]
    fn state_changed_signal(&self, state: u32) -> zbus::Result<()>;

    /// ActivatingConnection property
    #[dbus_proxy(property)]
    fn activating_connection(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// ActiveConnections property
    #[dbus_proxy(property)]
    fn active_connections(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// AllDevices property
    #[dbus_proxy(property)]
    fn all_devices(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// Capabilities property
    #[dbus_proxy(property)]
    fn capabilities(&self) -> zbus::Result<Vec<u32>>;

    /// Checkpoints property
    #[dbus_proxy(property)]
    fn checkpoints(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// Connectivity property
    #[dbus_proxy(property)]
    fn connectivity(&self) -> zbus::Result<u32>;

    /// ConnectivityCheckAvailable property
    #[dbus_proxy(property)]
    fn connectivity_check_available(&self) -> zbus::Result<bool>;

    /// ConnectivityCheckEnabled property
    #[dbus_proxy(property)]
    fn connectivity_check_enabled(&self) -> zbus::Result<bool>;
    #[dbus_proxy(property)]
    fn set_connectivity_check_enabled(&self, value: bool) -> zbus::Result<()>;

    /// ConnectivityCheckUri property
    #[dbus_proxy(property)]
    fn connectivity_check_uri(&self) -> zbus::Result<String>;

    /// Devices property
    #[dbus_proxy(property)]
    fn devices(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// GlobalDnsConfiguration property
    #[dbus_proxy(property)]
    fn global_dns_configuration(
        &self,
    ) -> zbus::Result<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>;
    #[dbus_proxy(property)]
    fn set_global_dns_configuration(
        &self,
        value: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// Metered property
    #[dbus_proxy(property)]
    fn metered(&self) -> zbus::Result<u32>;

    /// NetworkingEnabled property
    #[dbus_proxy(property)]
    fn networking_enabled(&self) -> zbus::Result<bool>;

    /// PrimaryConnection property
    #[dbus_proxy(property)]
    fn primary_connection(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// PrimaryConnectionType property
    #[dbus_proxy(property)]
    fn primary_connection_type(&self) -> zbus::Result<String>;

    /// RadioFlags property
    #[dbus_proxy(property)]
    fn radio_flags(&self) -> zbus::Result<u32>;

    /// Startup property
    #[dbus_proxy(property)]
    fn startup(&self) -> zbus::Result<bool>;

    /// State property
    #[dbus_proxy(property, name = "State")]
    fn state_property(&self) -> zbus::Result<u32>;

    /// Version property
    #[dbus_proxy(property)]
    fn version(&self) -> zbus::Result<String>;

    /// WimaxEnabled property
    #[dbus_proxy(property)]
    fn wimax_enabled(&self) -> zbus::Result<bool>;
    #[dbus_proxy(property)]
    fn set_wimax_enabled(&self, value: bool) -> zbus::Result<()>;

    /// WimaxHardwareEnabled property
    #[dbus_proxy(property)]
    fn wimax_hardware_enabled(&self) -> zbus::Result<bool>;

    /// WirelessEnabled property
    #[dbus_proxy(property)]
    fn wireless_enabled(&self) -> zbus::Result<bool>;
    #[dbus_proxy(property)]
    fn set_wireless_enabled(&self, value: bool) -> zbus::Result<()>;

    /// WirelessHardwareEnabled property
    #[dbus_proxy(property)]
    fn wireless_hardware_enabled(&self) -> zbus::Result<bool>;

    /// WwanEnabled property
    #[dbus_proxy(property)]
    fn wwan_enabled(&self) -> zbus::Result<bool>;
    #[dbus_proxy(property)]
    fn set_wwan_enabled(&self, value: bool) -> zbus::Result<()>;

    /// WwanHardwareEnabled property
    #[dbus_proxy(property)]
    fn wwan_hardware_enabled(&self) -> zbus::Result<bool>;
}
