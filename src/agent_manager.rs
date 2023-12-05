use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.freedesktop.NetworkManager.AgentManager")]
trait AgentManager {
    /// Register method
    fn register(&self, identifier: &str) -> zbus::Result<()>;

    /// RegisterWithCapabilities method
    fn register_with_capabilities(&self, identifier: &str, capabilities: u32) -> zbus::Result<()>;

    /// Unregister method
    fn unregister(&self) -> zbus::Result<()>;
}
