use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.freedesktop.NetworkManager.Device.Bluetooth")]
trait Bluetooth {
    /// BtCapabilities property
    #[dbus_proxy(property)]
    fn bt_capabilities(&self) -> zbus::Result<u32>;

    /// HwAddress property
    #[dbus_proxy(property)]
    fn hw_address(&self) -> zbus::Result<String>;

    /// Name property
    #[dbus_proxy(property)]
    fn name(&self) -> zbus::Result<String>;
}
