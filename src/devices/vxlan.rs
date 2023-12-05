use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.freedesktop.NetworkManager.Device.Vxlan")]
trait Vxlan {
    /// Ageing property
    #[dbus_proxy(property)]
    fn ageing(&self) -> zbus::Result<u32>;

    /// DstPort property
    #[dbus_proxy(property)]
    fn dst_port(&self) -> zbus::Result<u16>;

    /// Group property
    #[dbus_proxy(property)]
    fn group(&self) -> zbus::Result<String>;

    /// HwAddress property
    #[dbus_proxy(property)]
    fn hw_address(&self) -> zbus::Result<String>;

    /// Id property
    #[dbus_proxy(property)]
    fn id(&self) -> zbus::Result<u32>;

    /// L2miss property
    #[dbus_proxy(property)]
    fn l2miss(&self) -> zbus::Result<bool>;

    /// L3miss property
    #[dbus_proxy(property)]
    fn l3miss(&self) -> zbus::Result<bool>;

    /// Learning property
    #[dbus_proxy(property)]
    fn learning(&self) -> zbus::Result<bool>;

    /// Limit property
    #[dbus_proxy(property)]
    fn limit(&self) -> zbus::Result<u32>;

    /// Local property
    #[dbus_proxy(property)]
    fn local(&self) -> zbus::Result<String>;

    /// Parent property
    #[dbus_proxy(property)]
    fn parent(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// Proxy property
    #[dbus_proxy(property)]
    fn proxy(&self) -> zbus::Result<bool>;

    /// Rsc property
    #[dbus_proxy(property)]
    fn rsc(&self) -> zbus::Result<bool>;

    /// SrcPortMax property
    #[dbus_proxy(property)]
    fn src_port_max(&self) -> zbus::Result<u16>;

    /// SrcPortMin property
    #[dbus_proxy(property)]
    fn src_port_min(&self) -> zbus::Result<u16>;

    /// Tos property
    #[dbus_proxy(property)]
    fn tos(&self) -> zbus::Result<u8>;

    /// Ttl property
    #[dbus_proxy(property)]
    fn ttl(&self) -> zbus::Result<u8>;
}
