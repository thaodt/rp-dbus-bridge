use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.freedesktop.NetworkManager.Device.Infiniband")]
trait Infiniband {
    /// Carrier property
    #[dbus_proxy(property)]
    fn carrier(&self) -> zbus::Result<bool>;

    /// HwAddress property
    #[dbus_proxy(property)]
    fn hw_address(&self) -> zbus::Result<String>;
}
