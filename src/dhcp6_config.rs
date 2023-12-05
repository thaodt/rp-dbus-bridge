use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.freedesktop.NetworkManager.DHCP6Config")]
trait DHCP6Config {
    /// Options property
    #[dbus_proxy(property)]
    fn options(
        &self,
    ) -> zbus::Result<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>;
}
