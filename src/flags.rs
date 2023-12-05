use bitflags::bitflags;
use std::fmt;

bitflags! {
    #[derive(Debug)]
    pub struct ActivationStateFlags: u32 {
        const NONE = 0;
        const IS_MASTER = 0x1;
        const IS_SLAVE = 0x2;
        const LAYER2_READY = 0x4;
        const IP4_READY = 0x8;
        const IP6_READY = 0x10;
        const MASTER_HAS_SLAVES = 0x20;
        const LIFETIME_BOUND_TO_PROFILE_VISIBILITY = 0x40;
        const EXTERNAL = 0x80;
    }
}

impl fmt::Display for ActivationStateFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

bitflags! {
    #[derive(Debug)]
    pub struct BluetoothCapabilities: u32 {
        const NONE = 0x00000000;
        const DUN = 0x00000001;
        const NAP = 0x00000002;
    }
}

impl fmt::Display for BluetoothCapabilities {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

bitflags! {
    pub struct CheckpointCreateFlags: u32 {
        const NONE = 0x00;
        const DESTROY_ALL = 0x01;
        const DELETE_NEW_CONNECTIONS = 0x02;
        const DISCONNECT_NEW_DEVICES = 0x04;
        const ALLOW_OVERLAPPING = 0x08;
    }
}

impl fmt::Display for CheckpointCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

bitflags! {
    pub struct ClientInstanceFlags: u32 {
        const NONE = 0x00;
        const NO_AUTO_FETCH_PERMISSIONS = 0x01;
    }
}

impl fmt::Display for ClientInstanceFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

bitflags! {
    pub struct ConnectionSerializationFlags: u32 {
        const ALL = 0x00;
        const WITH_NON_SECRET = 0x01;
        const NO_SECRETS = 0x01;
        const WITH_SECRETS = 0x02;
        const ONLY_SECRETS = 0x02;
        const WITH_SECRETS_AGENT_OWNED = 0x04;
        const WITH_SECRETS_SYSTEM_OWNED = 0x08;
        const WITH_SECRETS_NOT_SAVED = 0x10;
    }
}

impl fmt::Display for ConnectionSerializationFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

bitflags! {
    pub struct DeviceCapabilities: u32 {
        const NONE = 0x00000000;
        const NM_SUPPORTED = 0x00000001;
        const CARRIER_DETECT = 0x00000002;
        const IS_SOFTWARE = 0x00000004;
        const SRIOV = 0x00000008;
    }
}

impl fmt::Display for DeviceCapabilities {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

bitflags! {
    pub struct DeviceInterfaceFlags: u32 {
        const NONE = 0x0;
        const UP = 0x1;
        const LOWER_UP = 0x2;
        const PROMISC = 0x4;
        const CARRIER = 0x10000;
        const LLDP_CLIENT_ENABLED = 0x20000;
    }
}

impl fmt::Display for DeviceInterfaceFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

bitflags! {
    pub struct DeviceModemCapabilities: u32 {
        const NONE = 0x0;
        const POTS = 0x1;
        const CDMA_EVDO = 0x2;
        const GSM_UMTS = 0x4;
        const LTE = 0x8;
    }
}

impl fmt::Display for DeviceModemCapabilities {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

bitflags! {
    pub struct DeviceWifiCapabilities: u32 {
        const NONE = 0x0000;
        const CIPHER_WEP40 = 0x0001;
        const CIPHER_WEP104 = 0x0002;
        const CIPHER_TKIP = 0x0004;
        const CIPHER_CCMP = 0x0008;
        const WPA = 0x0010;
        const RSN = 0x0020;
        const AP = 0x0040;
        const ADHOC = 0x0080;
        const FREQ_VALID = 0x0100;
        const FREQ_2GHZ = 0x0200;
        const FREQ_5GHZ = 0x0400;
        const MESH = 0x0800;
        const IBSS_RSN = 0x1000;
    }
}

impl fmt::Display for DeviceWifiCapabilities {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

bitflags! {
    pub struct DhcpHostnameFlags: u32 {
        const NONE = 0x0;
        const FQDN_SERV_UPDATE = 0x1;
        const FQDN_ENCODED = 0x2;
        const FQDN_NO_UPDATE = 0x4;
        const FQDN_CLEAR_FLAGS = 0x8;
    }
}

impl fmt::Display for DhcpHostnameFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

bitflags! {
    pub struct IPAddressCmpFlags: u32 {
        const NONE = 0x0;
        const WITH_ATTRS = 0x1;
    }
}

impl fmt::Display for IPAddressCmpFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

bitflags! {
    pub struct IPRoutingRuleAsStringFlags: u32 {
        const NONE = 0x0;
        const AF_INET = 0x1;
        const AF_INET6 = 0x2;
        const VALIDATE = 0x4;
    }
}

impl fmt::Display for IPRoutingRuleAsStringFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

bitflags! {
    pub struct IPTunnelFlags: u32 {
        const NONE = 0x0;
        const IP6_IGN_ENCAP_LIMIT = 0x1;
        const IP6_USE_ORIG_TCLASS = 0x2;
        const IP6_USE_ORIG_FLOWLABEL = 0x4;
        const IP6_MIP6_DEV = 0x8;
        const IP6_RCV_DSCP_COPY = 0x10;
        const IP6_USE_ORIG_FWMARK = 0x20;
    }
}

impl fmt::Display for IPTunnelFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

bitflags! {
    pub struct KeyfileHandlerFlags: u32 {
        const NONE = 0x0;
    }
}

impl fmt::Display for KeyfileHandlerFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

bitflags! {
    pub struct ManagerReloadFlags: u32 {
        const CONF = 0x1;
        const DNS_RC = 0x2;
        const DNS_FULL = 0x4;
    }
}

impl fmt::Display for ManagerReloadFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

bitflags! {
    pub struct SecretAgentCapabilities: u32 {
        const NONE = 0x0;
        const VPN_HINTS = 0x1;
        const LAST = 0x2;
    }
}

impl fmt::Display for SecretAgentCapabilities {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

bitflags! {
    pub struct SecretAgentGetSecretsFlags: u32 {
        const NONE = 0x0;
        const ALLOW_INTERACTION = 0x1;
        const REQUEST_NEW = 0x2;
        const USER_REQUESTED = 0x4;
        const WPS_PBC_ACTIVE = 0x8;
        const ONLY_SYSTEM = 0x80000000;
        const NO_ERRORS = 0x40000000;
    }
}

impl fmt::Display for SecretAgentGetSecretsFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

bitflags! {
    pub struct Setting8021xAuthFlags: u32 {
        const NONE = 0x0;
        const TLS_1_0_DISABLE = 0x1;
        const TLS_1_1_DISABLE = 0x2;
        const TLS_1_2_DISABLE = 0x4;
        const ALLOW_UNSAFE_RENEGOTIATION = 0x8;
        const ALL = 0xF;
    }
}

impl fmt::Display for Setting8021xAuthFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

bitflags! {
    pub struct SettingDcbFlags: u32 {
        const NONE = 0x0;
        const ENABLE = 0x1;
        const ADVERTISE = 0x2;
        const WILLING = 0x4;
    }
}

impl fmt::Display for SettingDcbFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

bitflags! {
    pub struct SettingSecretFlags: u32 {
        const NONE = 0x0;
        const AGENT_OWNED = 0x1;
        const NOT_SAVED = 0x2;
        const NOT_REQUIRED = 0x4;
    }
}

impl fmt::Display for SettingSecretFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

bitflags! {
    pub struct SettingWiredWakeOnLan: u32 {
        const PHY = 0x2;
        const UNICAST = 0x4;
        const MULTICAST = 0x8;
        const BROADCAST = 0x10;
        const ARP = 0x20;
        const MAGIC = 0x40;
        const DEFAULT = 0x1;
        const IGNORE = 0x8000;
    }
}

impl fmt::Display for SettingWiredWakeOnLan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

bitflags! {
    pub struct SettingWirelessSecurityWpsMethod: u32 {
        const DEFAULT = 0x0;
        const DISABLED = 0x1;
        const AUTO = 0x2;
        const PBC = 0x4;
        const PIN = 0x8;
    }
}

impl fmt::Display for SettingWirelessSecurityWpsMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

bitflags! {
    pub struct SettingWirelessWakeOnWLan: u32 {
        const ANY = 0x2;
        const DISCONNECT = 0x4;
        const MAGIC = 0x8;
        const GTK_REKEY_FAILURE = 0x10;
        const EAP_IDENTITY_REQUEST = 0x20;
        const __4WAY_HANDSHAKE = 0x40;
        const RFKILL_RELEASE = 0x80;
        const TCP = 0x100;
        const ALL = 0x1FE;
        const DEFAULT = 0x1;
        const IGNORE = 0x8000;
    }
}

impl fmt::Display for SettingWirelessWakeOnWLan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

bitflags! {
    pub struct SettingsAddConnection2Flags: u32 {
        const NONE = 0x0;
        const TO_DISK = 0x1;
        const IN_MEMORY = 0x2;
        const BLOCK_AUTOCONNECT = 0x20;
    }
}

impl fmt::Display for SettingsAddConnection2Flags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

bitflags! {
    pub struct SettingsConnectionFlags: u32 {
        const NONE = 0x0;
        const UNSAVED = 0x1;
        const NM_GENERATED = 0x2;
        const VOLATILE = 0x4;
        const EXTERNAL = 0x8;
    }
}

impl fmt::Display for SettingsConnectionFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

bitflags! {
    pub struct SettingsUpdate2Flags: u32 {
        const NONE = 0x0;
        const TO_DISK = 0x1;
        const IN_MEMORY = 0x2;
        const IN_MEMORY_DETACHED = 0x4;
        const IN_MEMORY_ONLY = 0x8;
        const VOLATILE = 0x10;
        const BLOCK_AUTOCONNECT = 0x20;
        const NO_REAPPLY = 0x40;
    }
}

impl fmt::Display for SettingsUpdate2Flags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

bitflags! {
    pub struct TeamLinkWatcherArpPingFlags: u32 {
        const VALIDATE_ACTIVE = 0x2;
        const VALIDATE_INACTIVE = 0x4;
        const SEND_ALWAYS = 0x8;
    }
}

impl fmt::Display for TeamLinkWatcherArpPingFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

bitflags! {
    pub struct VlanFlags: u32 {
        const REORDER_HEADERS = 0x1;
        const GVRP = 0x2;
        const LOOSE_BINDING = 0x4;
        const MVRP = 0x8;
    }
}

impl fmt::Display for VlanFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

bitflags! {
    pub struct VpnEditorPluginCapability: u32 {
        const NONE = 0x0;
        const IMPORT = 0x1;
        const EXPORT = 0x2;
        const IPV6 = 0x4;
    }
}

impl fmt::Display for VpnEditorPluginCapability {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

bitflags! {
    pub struct IEEE80211ApFlags: u32 {
        const NONE = 0x0;
        const PRIVACY = 0x1;
        const WPS = 0x2;
        const WPS_PBC = 0x4;
        const WPS_PIN = 0x8;
    }
}

impl fmt::Display for IEEE80211ApFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

bitflags! {
    pub struct IEEE80211ApSecurityFlags: u32 {
        const NONE = 0x00000000;
        const PAIR_WEP40 = 0x00000001;
        const PAIR_WEP104 = 0x00000002;
        const PAIR_TKIP = 0x00000004;
        const PAIR_CCMP = 0x00000008;
        const GROUP_WEP40 = 0x00000010;
        const GROUP_WEP104 = 0x00000020;
        const GROUP_TKIP = 0x00000040;
        const GROUP_CCMP = 0x00000080;
        const KEY_MGMT_PSK = 0x00000100;
        const KEY_MGMT_802_1X = 0x00000200;
        const KEY_MGMT_SAE = 0x00000400;
        const KEY_MGMT_OWE = 0x00000800;
        const KEY_MGMT_OWE_TM = 0x00001000;
        const KEY_MGMT_EAP_SUITE_B_192 = 0x00002000;
    }
}

impl fmt::Display for IEEE80211ApSecurityFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}
