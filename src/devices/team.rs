use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.freedesktop.NetworkManager.Device.Team")]
trait Team {
    /// Carrier property
    #[dbus_proxy(property)]
    fn carrier(&self) -> zbus::Result<bool>;

    /// Config property
    #[dbus_proxy(property)]
    fn config(&self) -> zbus::Result<String>;

    /// HwAddress property
    #[dbus_proxy(property)]
    fn hw_address(&self) -> zbus::Result<String>;

    /// Slaves property
    #[dbus_proxy(property)]
    fn slaves(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;
}
