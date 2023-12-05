use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.freedesktop.NetworkManager.Device.Modem")]
trait Modem {
    /// Apn property
    #[dbus_proxy(property)]
    fn apn(&self) -> zbus::Result<String>;

    /// CurrentCapabilities property
    #[dbus_proxy(property)]
    fn current_capabilities(&self) -> zbus::Result<u32>;

    /// DeviceId property
    #[dbus_proxy(property)]
    fn device_id(&self) -> zbus::Result<String>;

    /// ModemCapabilities property
    #[dbus_proxy(property)]
    fn modem_capabilities(&self) -> zbus::Result<u32>;

    /// OperatorCode property
    #[dbus_proxy(property)]
    fn operator_code(&self) -> zbus::Result<String>;
}
