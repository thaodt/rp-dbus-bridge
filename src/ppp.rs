use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.freedesktop.NetworkManager.PPP")]
trait PPP {
    /// NeedSecrets method
    fn need_secrets(&self) -> zbus::Result<(String, String)>;

    /// SetIfindex method
    fn set_ifindex(&self, ifindex: i32) -> zbus::Result<()>;

    /// SetIp4Config method
    fn set_ip4_config(
        &self,
        config: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// SetIp6Config method
    fn set_ip6_config(
        &self,
        config: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// SetState method
    fn set_state(&self, state: u32) -> zbus::Result<()>;
}
