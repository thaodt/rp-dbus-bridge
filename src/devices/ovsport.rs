use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.freedesktop.NetworkManager.Device.OvsPort")]
trait OvsPort {
    /// Slaves property
    #[dbus_proxy(property)]
    fn slaves(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;
}
