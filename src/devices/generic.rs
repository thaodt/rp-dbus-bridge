use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.freedesktop.NetworkManager.Device.Generic")]
trait Generic {
    /// HwAddress property
    #[dbus_proxy(property)]
    fn hw_address(&self) -> zbus::Result<String>;

    /// TypeDescription property
    #[dbus_proxy(property)]
    fn type_description(&self) -> zbus::Result<String>;
}
