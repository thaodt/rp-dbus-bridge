use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.freedesktop.NetworkManager.Device.WireGuard")]
trait WireGuard {
    /// FwMark property
    #[dbus_proxy(property)]
    fn fw_mark(&self) -> zbus::Result<u32>;

    /// ListenPort property
    #[dbus_proxy(property)]
    fn listen_port(&self) -> zbus::Result<u16>;

    /// PublicKey property
    #[dbus_proxy(property)]
    fn public_key(&self) -> zbus::Result<Vec<u8>>;
}
