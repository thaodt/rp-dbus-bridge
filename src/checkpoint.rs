use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.freedesktop.NetworkManager.Checkpoint")]
trait Checkpoint {
    /// Created property
    #[dbus_proxy(property)]
    fn created(&self) -> zbus::Result<i64>;

    /// Devices property
    #[dbus_proxy(property)]
    fn devices(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// RollbackTimeout property
    #[dbus_proxy(property)]
    fn rollback_timeout(&self) -> zbus::Result<u32>;
}
