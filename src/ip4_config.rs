use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.freedesktop.NetworkManager.IP4Config")]
trait IP4Config {
    /// AddressData property
    #[dbus_proxy(property)]
    fn address_data(
        &self,
    ) -> zbus::Result<Vec<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>>;

    /// Addresses property
    #[dbus_proxy(property)]
    fn addresses(&self) -> zbus::Result<Vec<Vec<u32>>>;

    /// DnsOptions property
    #[dbus_proxy(property)]
    fn dns_options(&self) -> zbus::Result<Vec<String>>;

    /// DnsPriority property
    #[dbus_proxy(property)]
    fn dns_priority(&self) -> zbus::Result<i32>;

    /// Domains property
    #[dbus_proxy(property)]
    fn domains(&self) -> zbus::Result<Vec<String>>;

    /// Gateway property
    #[dbus_proxy(property)]
    fn gateway(&self) -> zbus::Result<String>;

    /// NameserverData property
    #[dbus_proxy(property)]
    fn nameserver_data(
        &self,
    ) -> zbus::Result<Vec<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>>;

    /// Nameservers property
    #[dbus_proxy(property)]
    fn nameservers(&self) -> zbus::Result<Vec<u32>>;

    /// RouteData property
    #[dbus_proxy(property)]
    fn route_data(
        &self,
    ) -> zbus::Result<Vec<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>>;

    /// Routes property
    #[dbus_proxy(property)]
    fn routes(&self) -> zbus::Result<Vec<Vec<u32>>>;

    /// Searches property
    #[dbus_proxy(property)]
    fn searches(&self) -> zbus::Result<Vec<String>>;

    /// WinsServerData property
    #[dbus_proxy(property)]
    fn wins_server_data(&self) -> zbus::Result<Vec<String>>;

    /// WinsServers property
    #[dbus_proxy(property)]
    fn wins_servers(&self) -> zbus::Result<Vec<u32>>;
}
