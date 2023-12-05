use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.freedesktop.NetworkManager.Device.Wpan")]
trait Wpan {
    /// HwAddress property
    #[dbus_proxy(property)]
    fn hw_address(&self) -> zbus::Result<String>;
}
