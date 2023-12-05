use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.freedesktop.NetworkManager.Device.Wired")]
trait Wired {
    /// Carrier property
    #[dbus_proxy(property)]
    fn carrier(&self) -> zbus::Result<bool>;

    /// HwAddress property
    #[dbus_proxy(property)]
    fn hw_address(&self) -> zbus::Result<String>;

    /// PermHwAddress property
    #[dbus_proxy(property)]
    fn perm_hw_address(&self) -> zbus::Result<String>;

    /// S390Subchannels property
    #[dbus_proxy(property)]
    fn s390subchannels(&self) -> zbus::Result<Vec<String>>;

    /// Speed property
    #[dbus_proxy(property)]
    fn speed(&self) -> zbus::Result<u32>;
}
