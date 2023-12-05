use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.freedesktop.NetworkManager.Device.Vlan")]
trait Vlan {
    /// Carrier property
    #[dbus_proxy(property)]
    fn carrier(&self) -> zbus::Result<bool>;

    /// HwAddress property
    #[dbus_proxy(property)]
    fn hw_address(&self) -> zbus::Result<String>;

    /// Parent property
    #[dbus_proxy(property)]
    fn parent(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// VlanId property
    #[dbus_proxy(property)]
    fn vlan_id(&self) -> zbus::Result<u32>;
}
