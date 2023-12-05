pub(crate) mod adsl;
pub(crate) mod bluetooth;
pub(crate) mod bond;
pub(crate) mod bridge;
pub(crate) mod device;
pub(crate) mod dummy;
pub(crate) mod generic;
pub(crate) mod infiniband;
pub(crate) mod iptunnel;
pub(crate) mod lowpan;
pub(crate) mod macsec;
pub(crate) mod macvlan;
pub(crate) mod modem;
pub(crate) mod olpcmesh;
pub(crate) mod ovsbridge;
pub(crate) mod ovsinterface;
pub(crate) mod ovsport;
pub(crate) mod ppp;
pub(crate) mod statistics;
pub(crate) mod team;
pub(crate) mod tun;
pub(crate) mod veth;
pub(crate) mod vlan;
pub(crate) mod vrf;
pub(crate) mod vxlan;
pub(crate) mod wifip2p;
pub(crate) mod wimax;
pub(crate) mod wired;
pub(crate) mod wireguard;
pub(crate) mod wireless;
pub(crate) mod wpan;

pub type Adsl<'a> = crate::devices::adsl::AdslProxy<'a>;
pub type Bluetooth<'a> = crate::devices::bluetooth::BluetoothProxy<'a>;
pub type Bond<'a> = crate::devices::bond::BondProxy<'a>;
pub type Bridge<'a> = crate::devices::bridge::BridgeProxy<'a>;
pub type Device<'a> = crate::devices::device::DeviceProxy<'a>;
pub type Dummy<'a> = crate::devices::dummy::DummyProxy<'a>;
pub type Generic<'a> = crate::devices::generic::GenericProxy<'a>;
pub type IPTunnel<'a> = crate::devices::iptunnel::IPTunnelProxy<'a>;
pub type Infiniband<'a> = crate::devices::infiniband::InfinibandProxy<'a>;
pub type Lowpan<'a> = crate::devices::lowpan::LowpanProxy<'a>;
pub type Macsec<'a> = crate::devices::macsec::MacsecProxy<'a>;
pub type Macvlan<'a> = crate::devices::macvlan::MacvlanProxy<'a>;
pub type Modem<'a> = crate::devices::modem::ModemProxy<'a>;
pub type OlpcMesh<'a> = crate::devices::olpcmesh::OlpcMeshProxy<'a>;
pub type OvsBridge<'a> = crate::devices::ovsbridge::OvsBridgeProxy<'a>;
pub type OvsInterface<'a> = crate::devices::ovsinterface::OvsInterfaceProxy<'a>;
pub type OvsPort<'a> = crate::devices::ovsport::OvsPortProxy<'a>;
pub type Ppp<'a> = crate::devices::ppp::PppProxy<'a>;
pub type Statistics<'a> = crate::devices::statistics::StatisticsProxy<'a>;
pub type Team<'a> = crate::devices::team::TeamProxy<'a>;
pub type Tun<'a> = crate::devices::tun::TunProxy<'a>;
pub type Veth<'a> = crate::devices::veth::VethProxy<'a>;
pub type Vlan<'a> = crate::devices::vlan::VlanProxy<'a>;
pub type Vrf<'a> = crate::devices::vrf::VrfProxy<'a>;
pub type Vxlan<'a> = crate::devices::vxlan::VxlanProxy<'a>;
pub type WiMax<'a> = crate::devices::wimax::WiMaxProxy<'a>;
pub type WifiP2P<'a> = crate::devices::wifip2p::WifiP2PProxy<'a>;
pub type WireGuard<'a> = crate::devices::wireguard::WireGuardProxy<'a>;
pub type Wired<'a> = crate::devices::wired::WiredProxy<'a>;
pub type Wpan<'a> = crate::devices::wpan::WpanProxy<'a>;
pub type Wireless<'a> = crate::devices::wireless::WirelessProxy<'a>;
