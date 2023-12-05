use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.freedesktop.NetworkManager.Device.Adsl")]
trait Adsl {
    /// Carrier property
    #[dbus_proxy(property)]
    fn carrier(&self) -> zbus::Result<bool>;
}
