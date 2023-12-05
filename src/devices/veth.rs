use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.freedesktop.NetworkManager.Device.Veth")]
trait Veth {
    /// Peer property
    #[dbus_proxy(property)]
    fn peer(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;
}
