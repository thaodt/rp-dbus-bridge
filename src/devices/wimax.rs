use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.freedesktop.NetworkManager.Device.WiMax")]
trait WiMax {
    /// GetNspList method
    fn get_nsp_list(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// NspAdded signal
    #[dbus_proxy(signal)]
    fn nsp_added(&self, nsp: zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;

    /// NspRemoved signal
    #[dbus_proxy(signal)]
    fn nsp_removed(&self, nsp: zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;

    /// ActiveNsp property
    #[dbus_proxy(property)]
    fn active_nsp(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// Bsid property
    #[dbus_proxy(property)]
    fn bsid(&self) -> zbus::Result<String>;

    /// CenterFrequency property
    #[dbus_proxy(property)]
    fn center_frequency(&self) -> zbus::Result<u32>;

    /// Cinr property
    #[dbus_proxy(property)]
    fn cinr(&self) -> zbus::Result<i32>;

    /// HwAddress property
    #[dbus_proxy(property)]
    fn hw_address(&self) -> zbus::Result<String>;

    /// Nsps property
    #[dbus_proxy(property)]
    fn nsps(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// Rssi property
    #[dbus_proxy(property)]
    fn rssi(&self) -> zbus::Result<i32>;

    /// TxPower property
    #[dbus_proxy(property)]
    fn tx_power(&self) -> zbus::Result<i32>;
}
