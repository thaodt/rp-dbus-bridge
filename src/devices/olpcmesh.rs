use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.freedesktop.NetworkManager.Device.OlpcMesh")]
trait OlpcMesh {
    /// ActiveChannel property
    #[dbus_proxy(property)]
    fn active_channel(&self) -> zbus::Result<u32>;

    /// Companion property
    #[dbus_proxy(property)]
    fn companion(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// HwAddress property
    #[dbus_proxy(property)]
    fn hw_address(&self) -> zbus::Result<String>;
}
