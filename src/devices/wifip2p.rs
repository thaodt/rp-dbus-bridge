use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.freedesktop.NetworkManager.Device.WifiP2P")]
trait WifiP2P {
    /// StartFind method
    fn start_find(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// StopFind method
    fn stop_find(&self) -> zbus::Result<()>;

    /// PeerAdded signal
    #[dbus_proxy(signal)]
    fn peer_added(&self, peer: zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;

    /// PeerRemoved signal
    #[dbus_proxy(signal)]
    fn peer_removed(&self, peer: zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;

    /// HwAddress property
    #[dbus_proxy(property)]
    fn hw_address(&self) -> zbus::Result<String>;

    /// Peers property
    #[dbus_proxy(property)]
    fn peers(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;
}
