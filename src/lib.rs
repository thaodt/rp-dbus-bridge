mod access_point;
mod active_connection;
mod agent_manager;
mod checkpoint;
mod dhcp4_config;
mod dhcp6_config;
mod dns_manager;
mod ip4_config;
mod ip6_config;
mod ppp;
mod secret_agent;
mod wifi_p2p_peer;

pub mod devices;
pub mod enums;
// pub mod flags;
pub mod networkmanager;
pub mod settings;
pub mod types;
pub mod vpn;
pub mod vpn_plugin;

pub type AccessPoint<'a> = crate::access_point::AccessPointProxy<'a>;
pub type ActiveConnection<'a> = crate::active_connection::ActiveConnectionProxy<'a>;
pub type AgentManager<'a> = crate::agent_manager::AgentManagerProxy<'a>;
pub type Checkpoint<'a> = crate::checkpoint::CheckpointProxy<'a>;
pub type DHCP4Config<'a> = crate::dhcp4_config::DHCP4ConfigProxy<'a>;
pub type DHCP6Config<'a> = crate::dhcp6_config::DHCP6ConfigProxy<'a>;
pub type DnsManager<'a> = crate::dns_manager::DnsManagerProxy<'a>;
pub type IP4Config<'a> = crate::ip4_config::IP4ConfigProxy<'a>;
pub type IP6Config<'a> = crate::ip6_config::IP6ConfigProxy<'a>;
pub type NetworkManager<'a> = crate::networkmanager::NetworkManagerProxy<'a>;
pub type Ppp<'a> = crate::ppp::PPPProxy<'a>;
pub type SecretAgent<'a> = crate::secret_agent::SecretAgentProxy<'a>;
pub type WifiP2PPeer<'a> = crate::wifi_p2p_peer::WifiP2PPeerProxy<'a>;

pub mod blocking {
    pub mod devices {
        pub type Adsl<'a> = crate::devices::adsl::AdslProxyBlocking<'a>;
        pub type Bluetooth<'a> = crate::devices::bluetooth::BluetoothProxyBlocking<'a>;
        pub type Bond<'a> = crate::devices::bond::BondProxyBlocking<'a>;
        pub type Bridge<'a> = crate::devices::bridge::BridgeProxyBlocking<'a>;
        pub type Device<'a> = crate::devices::device::DeviceProxyBlocking<'a>;
        pub type Dummy<'a> = crate::devices::dummy::DummyProxyBlocking<'a>;
        pub type Generic<'a> = crate::devices::generic::GenericProxyBlocking<'a>;
        pub type IPTunnel<'a> = crate::devices::iptunnel::IPTunnelProxyBlocking<'a>;
        pub type Infiniband<'a> = crate::devices::infiniband::InfinibandProxyBlocking<'a>;
        pub type Lowpan<'a> = crate::devices::lowpan::LowpanProxyBlocking<'a>;
        pub type Macsec<'a> = crate::devices::macsec::MacsecProxyBlocking<'a>;
        pub type Macvlan<'a> = crate::devices::macvlan::MacvlanProxyBlocking<'a>;
        pub type Modem<'a> = crate::devices::modem::ModemProxyBlocking<'a>;
        pub type OlpcMesh<'a> = crate::devices::olpcmesh::OlpcMeshProxyBlocking<'a>;
        pub type OvsBridge<'a> = crate::devices::ovsbridge::OvsBridgeProxyBlocking<'a>;
        pub type OvsInterface<'a> = crate::devices::ovsinterface::OvsInterfaceProxyBlocking<'a>;
        pub type OvsPort<'a> = crate::devices::ovsport::OvsPortProxyBlocking<'a>;
        pub type Ppp<'a> = crate::devices::ppp::PppProxyBlocking<'a>;
        pub type Statistics<'a> = crate::devices::statistics::StatisticsProxyBlocking<'a>;
        pub type Team<'a> = crate::devices::team::TeamProxyBlocking<'a>;
        pub type Tun<'a> = crate::devices::tun::TunProxyBlocking<'a>;
        pub type Veth<'a> = crate::devices::veth::VethProxyBlocking<'a>;
        pub type Vlan<'a> = crate::devices::vlan::VlanProxyBlocking<'a>;
        pub type Vrf<'a> = crate::devices::vrf::VrfProxyBlocking<'a>;
        pub type Vxlan<'a> = crate::devices::vxlan::VxlanProxyBlocking<'a>;
        pub type WiMax<'a> = crate::devices::wimax::WiMaxProxyBlocking<'a>;
        pub type WifiP2P<'a> = crate::devices::wifip2p::WifiP2PProxyBlocking<'a>;
        pub type WireGuard<'a> = crate::devices::wireguard::WireGuardProxyBlocking<'a>;
        pub type Wired<'a> = crate::devices::wired::WiredProxyBlocking<'a>;
        pub type Wpan<'a> = crate::devices::wpan::WpanProxyBlocking<'a>;
        pub type Wireless<'a> = crate::devices::wireless::WirelessProxyBlocking<'a>;
    }

    pub type AccessPoint<'a> = crate::access_point::AccessPointProxyBlocking<'a>;
    pub type ActiveConnection<'a> = crate::active_connection::ActiveConnectionProxyBlocking<'a>;
    pub type AgentManager<'a> = crate::agent_manager::AgentManagerProxyBlocking<'a>;
    pub type Checkpoint<'a> = crate::checkpoint::CheckpointProxyBlocking<'a>;
    pub type DHCP4Config<'a> = crate::dhcp4_config::DHCP4ConfigProxyBlocking<'a>;
    pub type DHCP6Config<'a> = crate::dhcp6_config::DHCP6ConfigProxyBlocking<'a>;
    pub type DnsManager<'a> = crate::dns_manager::DnsManagerProxyBlocking<'a>;
    pub type IP4Config<'a> = crate::ip4_config::IP4ConfigProxyBlocking<'a>;
    pub type IP6Config<'a> = crate::ip6_config::IP6ConfigProxyBlocking<'a>;
    pub type Ppp<'a> = crate::ppp::PPPProxyBlocking<'a>;
    pub type SecretAgent<'a> = crate::secret_agent::SecretAgentProxyBlocking<'a>;
    pub type WifiP2PPeer<'a> = crate::wifi_p2p_peer::WifiP2PPeerProxyBlocking<'a>;

    pub mod settings {
        pub type Connection<'a> = crate::settings::connection::ConnectionProxyBlocking<'a>;
        pub type Settings<'a> = crate::settings::SettingsProxyBlocking<'a>;
    }

    pub mod vpn {
        pub type Connection<'a> = crate::vpn::connection::ConnectionProxyBlocking<'a>;
        pub type Plugin<'a> = crate::vpn::plugin::PluginProxyBlocking<'a>;
    }

    pub type NetworkManager<'a> = crate::networkmanager::NetworkManagerProxyBlocking<'a>;
}

#[cfg(test)]
mod tests {
    use zbus::zvariant;

    use crate::{
        devices::{Device, Wireless},
        enums::DeviceType,
        settings::{Connection, Settings},
        NetworkManager,
    };

    #[tokio::test]
    async fn it_works() -> zbus::Result<()> {
        let conn = zbus::Connection::system().await?;
        let nm = NetworkManager::new(&conn).await?;

        let devices_obj_paths = nm.get_devices().await?;

        for device_obj_path in &devices_obj_paths {
            let device = Device::builder(&conn)
                .destination("org.freedesktop.NetworkManager")?
                .path(device_obj_path)?
                .build()
                .await?;

            if device.device_type().await? == DeviceType::Wifi.to_string() {
                let wireless = Wireless::builder(&conn)
                    .destination("org.freedesktop.NetworkManager")?
                    .path(device_obj_path)?
                    .build()
                    .await?;

                let aps = wireless.get_access_points().await?;

                println!("{:?}", aps);
            }
        }

        let settings = Settings::builder(&conn)
            .destination("org.freedesktop.NetworkManager")?
            .path("/org/freedesktop/NetworkManager/Settings")?
            .build()
            .await?;

        let system_connections_obj_paths = settings.list_connections().await?;

        for connection_obj_path in system_connections_obj_paths {
            let connection = Connection::builder(&conn)
                .destination("org.freedesktop.NetworkManager")?
                .path(connection_obj_path)?
                .build()
                .await?;

            let conn_settings = connection.get_settings().await?;

            let wireless_settings = match conn_settings.get("802-11-wireless") {
                Some(value) => value,
                None => continue,
            };

            let ssid_bytes = match wireless_settings.get("ssid") {
                Some(value) => value,
                None => continue,
            };

            let ssid_array = match zvariant::Array::try_from(ssid_bytes.to_owned()) {
                Ok(array) => array,
                Err(_) => continue,
            };

            let ssid_vec: Vec<u8> = ssid_array
                .iter()
                .map(|e| *e.downcast_ref().unwrap())
                .collect();

            let ssid = String::from_utf8_lossy(&ssid_vec[..]);

            println!("{}", ssid);
        }

        Ok(())
    }
}
