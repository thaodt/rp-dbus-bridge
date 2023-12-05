use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.freedesktop.NetworkManager.Device.Tun")]
trait Tun {
    /// Group property
    #[dbus_proxy(property)]
    fn group(&self) -> zbus::Result<i64>;

    /// HwAddress property
    #[dbus_proxy(property)]
    fn hw_address(&self) -> zbus::Result<String>;

    /// Mode property
    #[dbus_proxy(property)]
    fn mode(&self) -> zbus::Result<String>;

    /// MultiQueue property
    #[dbus_proxy(property)]
    fn multi_queue(&self) -> zbus::Result<bool>;

    /// NoPi property
    #[dbus_proxy(property)]
    fn no_pi(&self) -> zbus::Result<bool>;

    /// Owner property
    #[dbus_proxy(property)]
    fn owner(&self) -> zbus::Result<i64>;

    /// VnetHdr property
    #[dbus_proxy(property)]
    fn vnet_hdr(&self) -> zbus::Result<bool>;
}
