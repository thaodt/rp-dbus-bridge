use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.freedesktop.NetworkManager.Device.Vrf")]
trait Vrf {
    /// Table property
    #[dbus_proxy(property)]
    fn table(&self) -> zbus::Result<u32>;
}
