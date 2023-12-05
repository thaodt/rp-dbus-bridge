use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.freedesktop.NetworkManager.Settings.Connection")]
trait Connection {
    /// ClearSecrets method
    fn clear_secrets(&self) -> zbus::Result<()>;

    /// Delete method
    fn delete(&self) -> zbus::Result<()>;

    /// GetSecrets method
    fn get_secrets(
        &self,
        setting_name: &str,
    ) -> zbus::Result<
        std::collections::HashMap<
            String,
            std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
        >,
    >;

    /// GetSettings method
    fn get_settings(
        &self,
    ) -> zbus::Result<
        std::collections::HashMap<
            String,
            std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
        >,
    >;

    /// Save method
    fn save(&self) -> zbus::Result<()>;

    /// Update method
    fn update(
        &self,
        properties: std::collections::HashMap<
            &str,
            std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
        >,
    ) -> zbus::Result<()>;

    /// Update2 method
    fn update2(
        &self,
        settings: std::collections::HashMap<
            &str,
            std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
        >,
        flags: u32,
        args: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>;

    /// UpdateUnsaved method
    fn update_unsaved(
        &self,
        properties: std::collections::HashMap<
            &str,
            std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
        >,
    ) -> zbus::Result<()>;

    /// Removed signal
    #[dbus_proxy(signal)]
    fn removed(&self) -> zbus::Result<()>;

    /// Updated signal
    #[dbus_proxy(signal)]
    fn updated(&self) -> zbus::Result<()>;

    /// Filename property
    #[dbus_proxy(property)]
    fn filename(&self) -> zbus::Result<String>;

    /// Flags property
    #[dbus_proxy(property)]
    fn flags(&self) -> zbus::Result<u32>;

    /// Unsaved property
    #[dbus_proxy(property)]
    fn unsaved(&self) -> zbus::Result<bool>;
}
