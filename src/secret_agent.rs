use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.freedesktop.NetworkManager.SecretAgent")]
trait SecretAgent {
    /// CancelGetSecrets method
    fn cancel_get_secrets(
        &self,
        connection_path: &zbus::zvariant::ObjectPath<'_>,
        setting_name: &str,
    ) -> zbus::Result<()>;

    /// DeleteSecrets method
    fn delete_secrets(
        &self,
        connection: std::collections::HashMap<
            &str,
            std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
        >,
        connection_path: &zbus::zvariant::ObjectPath<'_>,
    ) -> zbus::Result<()>;

    /// GetSecrets method
    fn get_secrets(
        &self,
        connection: std::collections::HashMap<
            &str,
            std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
        >,
        connection_path: &zbus::zvariant::ObjectPath<'_>,
        setting_name: &str,
        hints: &[&str],
        flags: u32,
    ) -> zbus::Result<
        std::collections::HashMap<
            String,
            std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
        >,
    >;

    /// SaveSecrets method
    fn save_secrets(
        &self,
        connection: std::collections::HashMap<
            &str,
            std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
        >,
        connection_path: &zbus::zvariant::ObjectPath<'_>,
    ) -> zbus::Result<()>;
}
