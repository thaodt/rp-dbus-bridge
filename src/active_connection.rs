use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.freedesktop.NetworkManager.Connection.Active")]
trait ActiveConnection {
    /// StateChanged signal
    #[dbus_proxy(signal, name = "StateChanged")]
    fn state_changed_signal(&self, state: u32, reason: u32) -> zbus::Result<()>;

    /// Connection property
    #[dbus_proxy(property)]
    fn connection(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// Default property
    #[dbus_proxy(property)]
    fn default(&self) -> zbus::Result<bool>;

    /// Default6 property
    #[dbus_proxy(property)]
    fn default6(&self) -> zbus::Result<bool>;

    /// Devices property
    #[dbus_proxy(property)]
    fn devices(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// Dhcp4Config property
    #[dbus_proxy(property)]
    fn dhcp4_config(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// Dhcp6Config property
    #[dbus_proxy(property)]
    fn dhcp6_config(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// Id property
    #[dbus_proxy(property)]
    fn id(&self) -> zbus::Result<String>;

    /// Ip4Config property
    #[dbus_proxy(property)]
    fn ip4_config(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// Ip6Config property
    #[dbus_proxy(property)]
    fn ip6_config(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// Master property
    #[dbus_proxy(property)]
    fn master(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// SpecificObject property
    #[dbus_proxy(property)]
    fn specific_object(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// State property
    #[dbus_proxy(property)]
    fn state(&self) -> zbus::Result<u32>;

    /// StateFlags property
    #[dbus_proxy(property)]
    fn state_flags(&self) -> zbus::Result<u32>;

    /// Type property
    #[dbus_proxy(property)]
    fn type_(&self) -> zbus::Result<String>;

    /// Uuid property
    #[dbus_proxy(property)]
    fn uuid(&self) -> zbus::Result<String>;

    /// Vpn property
    #[dbus_proxy(property)]
    fn vpn(&self) -> zbus::Result<bool>;
}
