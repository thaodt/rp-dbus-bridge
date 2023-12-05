use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.freedesktop.NetworkManager.Device.OvsBridge")]
trait OvsBridge {
    /// Slaves property
    #[dbus_proxy(property)]
    fn slaves(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;
}
