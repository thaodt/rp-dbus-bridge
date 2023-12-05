use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.freedesktop.NetworkManager.Device.Macvlan")]
trait Macvlan {
    /// Mode property
    #[dbus_proxy(property)]
    fn mode(&self) -> zbus::Result<String>;

    /// NoPromisc property
    #[dbus_proxy(property)]
    fn no_promisc(&self) -> zbus::Result<bool>;

    /// Parent property
    #[dbus_proxy(property)]
    fn parent(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// Tap property
    #[dbus_proxy(property)]
    fn tap(&self) -> zbus::Result<bool>;
}
