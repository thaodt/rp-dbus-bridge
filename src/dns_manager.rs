use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.freedesktop.NetworkManager.DnsManager")]
trait DnsManager {
    /// Configuration property
    #[dbus_proxy(property)]
    fn configuration(
        &self,
    ) -> zbus::Result<Vec<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>>;

    /// Mode property
    #[dbus_proxy(property)]
    fn mode(&self) -> zbus::Result<String>;

    /// RcManager property
    #[dbus_proxy(property)]
    fn rc_manager(&self) -> zbus::Result<String>;
}
