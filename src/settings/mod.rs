pub mod connection;
use zbus::dbus_proxy;

pub type Connection<'a> = crate::settings::connection::ConnectionProxy<'a>;
pub type Settings<'a> = crate::settings::SettingsProxy<'a>;

#[dbus_proxy(interface = "org.freedesktop.NetworkManager.Settings")]
trait Settings {
    /// AddConnection method
    fn add_connection(
        &self,
        connection: std::collections::HashMap<
            &str,
            std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
        >,
    ) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// AddConnection2 method
    fn add_connection2(
        &self,
        settings: std::collections::HashMap<
            &str,
            std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
        >,
        flags: u32,
        args: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<(
        zbus::zvariant::OwnedObjectPath,
        std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
    )>;

    /// AddConnectionUnsaved method
    fn add_connection_unsaved(
        &self,
        connection: std::collections::HashMap<
            &str,
            std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
        >,
    ) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// GetConnectionByUuid method
    fn get_connection_by_uuid(&self, uuid: &str) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// ListConnections method
    fn list_connections(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// LoadConnections method
    fn load_connections(&self, filenames: &[&str]) -> zbus::Result<(bool, Vec<String>)>;

    /// ReloadConnections method
    fn reload_connections(&self) -> zbus::Result<bool>;

    /// SaveHostname method
    fn save_hostname(&self, hostname: &str) -> zbus::Result<()>;

    /// ConnectionRemoved signal
    #[dbus_proxy(signal)]
    fn connection_removed(&self, connection: zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;

    /// NewConnection signal
    #[dbus_proxy(signal)]
    fn new_connection(&self, connection: zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;

    /// CanModify property
    #[dbus_proxy(property)]
    fn can_modify(&self) -> zbus::Result<bool>;

    /// Connections property
    #[dbus_proxy(property)]
    fn connections(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// Hostname property
    #[dbus_proxy(property)]
    fn hostname(&self) -> zbus::Result<String>;
}
