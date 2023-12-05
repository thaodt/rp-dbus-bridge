use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.freedesktop.NetworkManager.Device.Macsec")]
trait Macsec {
    /// CipherSuite property
    #[dbus_proxy(property)]
    fn cipher_suite(&self) -> zbus::Result<u64>;

    /// EncodingSa property
    #[dbus_proxy(property)]
    fn encoding_sa(&self) -> zbus::Result<u8>;

    /// Encrypt property
    #[dbus_proxy(property)]
    fn encrypt(&self) -> zbus::Result<bool>;

    /// Es property
    #[dbus_proxy(property)]
    fn es(&self) -> zbus::Result<bool>;

    /// IcvLength property
    #[dbus_proxy(property)]
    fn icv_length(&self) -> zbus::Result<u8>;

    /// IncludeSci property
    #[dbus_proxy(property)]
    fn include_sci(&self) -> zbus::Result<bool>;

    /// Parent property
    #[dbus_proxy(property)]
    fn parent(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// Protect property
    #[dbus_proxy(property)]
    fn protect(&self) -> zbus::Result<bool>;

    /// ReplayProtect property
    #[dbus_proxy(property)]
    fn replay_protect(&self) -> zbus::Result<bool>;

    /// Scb property
    #[dbus_proxy(property)]
    fn scb(&self) -> zbus::Result<bool>;

    /// Sci property
    #[dbus_proxy(property)]
    fn sci(&self) -> zbus::Result<u64>;

    /// Validation property
    #[dbus_proxy(property)]
    fn validation(&self) -> zbus::Result<String>;

    /// Window property
    #[dbus_proxy(property)]
    fn window(&self) -> zbus::Result<u32>;
}
