use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.freedesktop.NetworkManager.AccessPoint")]
trait AccessPoint {
    /// Flags property
    #[dbus_proxy(property)]
    fn flags(&self) -> zbus::Result<u32>;

    /// Frequency property
    #[dbus_proxy(property)]
    fn frequency(&self) -> zbus::Result<u32>;

    /// HwAddress property
    #[dbus_proxy(property)]
    fn hw_address(&self) -> zbus::Result<String>;

    /// LastSeen property
    #[dbus_proxy(property)]
    fn last_seen(&self) -> zbus::Result<i32>;

    /// MaxBitrate property
    #[dbus_proxy(property)]
    fn max_bitrate(&self) -> zbus::Result<u32>;

    /// Mode property
    #[dbus_proxy(property)]
    fn mode(&self) -> zbus::Result<u32>;

    /// RsnFlags property
    #[dbus_proxy(property)]
    fn rsn_flags(&self) -> zbus::Result<u32>;

    /// Ssid property
    #[dbus_proxy(property)]
    fn ssid(&self) -> zbus::Result<Vec<u8>>;

    /// Strength property
    #[dbus_proxy(property)]
    fn strength(&self) -> zbus::Result<u8>;

    /// WpaFlags property
    #[dbus_proxy(property)]
    fn wpa_flags(&self) -> zbus::Result<u32>;
}
