use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.freedesktop.NetworkManager.Device.Statistics")]
trait Statistics {
    /// RefreshRateMs property
    #[dbus_proxy(property)]
    fn refresh_rate_ms(&self) -> zbus::Result<u32>;
    #[dbus_proxy(property)]
    fn set_refresh_rate_ms(&self, value: u32) -> zbus::Result<()>;

    /// RxBytes property
    #[dbus_proxy(property)]
    fn rx_bytes(&self) -> zbus::Result<u64>;

    /// TxBytes property
    #[dbus_proxy(property)]
    fn tx_bytes(&self) -> zbus::Result<u64>;
}
