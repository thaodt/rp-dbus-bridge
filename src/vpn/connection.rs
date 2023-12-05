use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.freedesktop.NetworkManager.VPN.Connection")]
trait Connection {
    /// VpnStateChanged signal
    #[dbus_proxy(signal, name = "VpnStateChanged")]
    fn vpn_state_changed_signal(&self, state: u32, reason: u32) -> zbus::Result<()>;

    /// Banner property
    #[dbus_proxy(property)]
    fn banner(&self) -> zbus::Result<String>;

    /// VpnState property
    #[dbus_proxy(property)]
    fn vpn_state(&self) -> zbus::Result<u32>;
}
