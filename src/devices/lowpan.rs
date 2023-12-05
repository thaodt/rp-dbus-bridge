use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.freedesktop.NetworkManager.Device.Lowpan")]
trait Lowpan {
    /// HwAddress property
    #[dbus_proxy(property)]
    fn hw_address(&self) -> zbus::Result<String>;

    /// Parent property
    #[dbus_proxy(property)]
    fn parent(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;
}
