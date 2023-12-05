use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.freedesktop.NetworkManager.Device.IPTunnel")]
trait IPTunnel {
    /// EncapsulationLimit property
    #[dbus_proxy(property)]
    fn encapsulation_limit(&self) -> zbus::Result<u8>;

    /// Flags property
    #[dbus_proxy(property)]
    fn flags(&self) -> zbus::Result<u32>;

    /// FlowLabel property
    #[dbus_proxy(property)]
    fn flow_label(&self) -> zbus::Result<u32>;

    /// InputKey property
    #[dbus_proxy(property)]
    fn input_key(&self) -> zbus::Result<String>;

    /// Local property
    #[dbus_proxy(property)]
    fn local(&self) -> zbus::Result<String>;

    /// Mode property
    #[dbus_proxy(property)]
    fn mode(&self) -> zbus::Result<u32>;

    /// OutputKey property
    #[dbus_proxy(property)]
    fn output_key(&self) -> zbus::Result<String>;

    /// Parent property
    #[dbus_proxy(property)]
    fn parent(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// PathMtuDiscovery property
    #[dbus_proxy(property)]
    fn path_mtu_discovery(&self) -> zbus::Result<bool>;

    /// Remote property
    #[dbus_proxy(property)]
    fn remote(&self) -> zbus::Result<String>;

    /// Tos property
    #[dbus_proxy(property)]
    fn tos(&self) -> zbus::Result<u8>;

    /// Ttl property
    #[dbus_proxy(property)]
    fn ttl(&self) -> zbus::Result<u8>;
}
