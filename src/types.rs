#[repr(C)]
pub enum DeviceType {
    Unknown = 0,
    Ethernet,
    Wifi,
    Unused1,
    Unused2,
    Bluetooth,
    OlpcMesh,
    WiMax,
    Modem,
    Infiniband,
    Bond,
    Vlan,
    Adsl,
    Bridge,
    Generic,
    Team,
    Tun,
    IPTunnel,
    Macvlan,
    Vxlan,
    Veth,
    Macsec,
    Dummy,
    Ppp,
    OvsInterface,
    OvsPort,
    OvsBridge,
    Wpan,
    Lowpan,
    Wireguard,
    WifiP2P,
    Vrf,
}

#[repr(C)]
pub enum Capability {
    Team = 1,
    Ovs,
}

#[repr(C)]
pub enum State {
    Unknown = 0,
    Asleep = 10,
    Disconnected = 20,
    Disconnecting = 30,
    Connecting = 40,
    ConnectedLocal = 50,
    ConnectedSite = 60,
    ConnectedGlobal = 70,
}

#[repr(C)]
pub enum ConnectivityState {
    Unknown = 0,
    None,
    Portal,
    Limited,
    Full,
}

#[repr(C)]
pub enum DeviceCapabilities {
    None = 0x00000000,
    Supported = 0x00000001,
    CarrierDetect = 0x00000002,
    IsSoftware = 0x00000004,
    Sriov = 0x00000008,
}

pub enum DeviceWifiCapabilities {
    None = 0x00000000,
    CipherWep40 = 0x00000001,
    CipherWep104 = 0x00000002,
    CipherTkip = 0x00000004,
    CipherCcmp = 0x00000008,
    Wpa = 0x00000010,
    Rsn = 0x00000020,
    Ap = 0x00000040,
    Adhoc = 0x00000080,
    FreqValid = 0x00000100,
    Freq2GHz = 0x00000200,
    Freq5GHz = 0x00000400,
    Mesh = 0x00001000,
    IbssRsn = 0x00002000,
}

pub enum IEE80211ApFlags {
    None = 0x00000000,
    Privacy = 0x00000001,
    Wps = 0x00000002,
    WpsPbc = 0x00000003,
    WpsPin = 0x00000004,
}

pub enum IEE80211ApSecurityFlags {
    None = 0x00000000,
    PairWep40 = 0x00000001,
    PairWep104 = 0x00000002,
    PairTkip = 0x00000004,
    PairCcmp = 0x00000008,
    GroupWep40 = 0x00000010,
    GroupWep104 = 0x00000020,
    GroupTkip = 0x00000040,
    GroupCcmp = 0x00000080,
    MgmtPsk = 0x00000100,
    Mgmt8021X = 0x00000200,
    MgmtSae = 0x00000400,
    MgmtOwe = 0x00000800,
    MgmtOweTm = 0x00001000,
    MgmtEapSuiteB192 = 0x00002000,
}

pub enum IEE80211Mode {
    Unknown = 0,
    Adhoc,
    Infra,
    Ap,
    Mesh,
}

pub enum BluetoothCapabilities {
    None = 0x00000000,
    Dun = 0x00000001,
    Nap = 0x00000002,
}

pub enum DeviceModemCapabilities {
    None = 0x00000000,
    Pots = 0x00000001,
    CdmaEvdo = 0x00000002,
    GsmUmts = 0x00000004,
    Lte = 0x00000008,
}

pub enum WimaxNspNetworkType {
    Unknown = 0,
    Home,
    Partner,
    RoamingPartner,
}

pub enum DeviceState {
    Unknown = 0,
    Unmanaged = 10,
    Unavailable = 20,
    Disconnected = 30,
    Prepare = 40,
    Config = 50,
    NeedAuth = 60,
    IpConfig = 70,
    IpCheck = 80,
    Secondaries = 90,
    Activated = 100,
    Deactivating = 110,
    Failed = 120,
}

pub enum DeviceStateReason {
    None,
    Unknown,
    NowManaged,
    NowUnmanaged,
    ConfigFailed,
    IpConfigUnavailable,
    IpConfigExpired,
    NoSecrets,
    SupplicantDisconnect,
    SupplicantConfigFailed,
    SupplicantFailed,
    SupplicantTimeout,
    PppStartFailed,
    PppDisconnect,
    PppFailed,
    DhcpStartFailed,
    DhcpError,
    DhcpFailed,
    SharedStartFailed,
    SharedFailed,
    AutoipStartFailed,
    AutoipError,
    AutoipFailed,
    ModemBusy,
    ModemNoDialTone,
    ModemNoCarrier,
    ModemDialTimeout,
    ModemDialFailed,
    ModemInitFailed,
    GsmApnFailed,
    GsmRegistrationNotSearching,
    GsmRegistrationDenied,
    GsmRegistrationTimeout,
    GsmRegistrationFailed,
    GsmPinCheckFailed,
    FirmwareMissing,
    Removed,
    Sleeping,
    ConnectionRemoved,
    UserRequested,
    Carrier,
    ConnectionAssumed,
    SupplicantAvailable,
    ModemNotFound,
    BtFailed,
    GsmSimNotInserted,
    GsmSimPinRequired,
    GsmSimPukRequired,
    GsmSimWrong,
    InfinibandMode,
    DependencyFailed,
    Br2684Failed,
    ModemManagerUnavailable,
    SsidNotFound,
    SecondaryConnectionFailed,
    DcbFcoeFailed,
    TeamdControlFailed,
    ModemFailed,
    ModemAvailable,
    SimPinIncorrect,
    NewActivation,
    ParentChanged,
    ParentManagedChanged,
    OvsdbFailed,
    IpAddressDuplicate,
    IpMethodUnsupported,
    SriovConfigurationFailed,
    PeerNotFound,
}

pub enum Metered {
    Unknown,
    Yes,
    No,
    GuessYes,
    GuessNo,
}
