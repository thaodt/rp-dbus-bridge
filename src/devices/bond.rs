use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.freedesktop.NetworkManager.Device.Bond")]
trait Bond {
    /// Carrier property
    #[dbus_proxy(property)]
    fn carrier(&self) -> zbus::Result<bool>;

    /// HwAddress property
    #[dbus_proxy(property)]
    fn hw_address(&self) -> zbus::Result<String>;

    /// Slaves property
    #[dbus_proxy(property)]
    fn slaves(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;
}
