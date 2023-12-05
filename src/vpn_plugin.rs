use zbus::dbus_proxy;

#[dbus_proxy(
    interface = "org.freedesktop.NetworkManager.VPN.Plugin",
    assume_defaults = true
)]
trait Plugin {
    /// Connect method
    fn connect(
        &self,
        connection: std::collections::HashMap<
            &str,
            std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
        >,
    ) -> zbus::Result<()>;

    /// ConnectInteractive method
    fn connect_interactive(
        &self,
        connection: std::collections::HashMap<
            &str,
            std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
        >,
        details: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// Disconnect method
    fn disconnect(&self) -> zbus::Result<()>;

    /// NewSecrets method
    fn new_secrets(
        &self,
        connection: std::collections::HashMap<
            &str,
            std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
        >,
    ) -> zbus::Result<()>;

    /// SetConfig method
    fn set_config(
        &self,
        config: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// SetFailure method
    fn set_failure(&self, reason: &str) -> zbus::Result<()>;

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

    /// Config signal
    #[dbus_proxy(signal)]
    fn config(
        &self,
        config: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// Failure signal
    #[dbus_proxy(signal)]
    fn failure(&self, reason: u32) -> zbus::Result<()>;

    /// Ip4Config signal
    #[dbus_proxy(signal)]
    fn ip4_config(
        &self,
        ip4config: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// Ip6Config signal
    #[dbus_proxy(signal)]
    fn ip6_config(
        &self,
        ip6config: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// LoginBanner signal
    #[dbus_proxy(signal)]
    fn login_banner(&self, banner: &str) -> zbus::Result<()>;

    /// SecretsRequired signal
    #[dbus_proxy(signal)]
    fn secrets_required(&self, message: &str, secrets: &[&str]) -> zbus::Result<()>;

    /// StateChanged signal
    #[dbus_proxy(signal, name = "StateChanged")]
    fn state_changed_sgn(&self, state: u32) -> zbus::Result<()>;

    /// State property
    #[dbus_proxy(property)]
    fn state(&self) -> zbus::Result<u32>;
}
