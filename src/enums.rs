use std::fmt;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum ActiveConnectionState {
    Unknown,
    Activating,
    Activated,
    Deactivating,
    Deactivated,
    __Unknown(i32),
}

impl fmt::Display for ActiveConnectionState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ActiveConnectionState::{}",
            match *self {
                Self::Unknown => "Unknown",
                Self::Activating => "Activating",
                Self::Activated => "Activated",
                Self::Deactivating => "Deactivating",
                Self::Deactivated => "Deactivated",
                _ => "Unknown",
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum ActiveConnectionStateReason {
    Unknown,
    None,
    UserDisconnected,
    DeviceDisconnected,
    ServiceStopped,
    IpConfigInvalid,
    ConnectTimeout,
    ServiceStartTimeout,
    ServiceStartFailed,
    NoSecrets,
    LoginFailed,
    ConnectionRemoved,
    DependencyFailed,
    DeviceRealizeFailed,
    DeviceRemoved,
    __Unknown(i32),
}

impl fmt::Display for ActiveConnectionStateReason {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ActiveConnectionStateReason::{}",
            match *self {
                Self::Unknown => "Unknown",
                Self::None => "None",
                Self::UserDisconnected => "UserDisconnected",
                Self::DeviceDisconnected => "DeviceDisconnected",
                Self::ServiceStopped => "ServiceStopped",
                Self::IpConfigInvalid => "IpConfigInvalid",
                Self::ConnectTimeout => "ConnectTimeout",
                Self::ServiceStartTimeout => "ServiceStartTimeout",
                Self::ServiceStartFailed => "ServiceStartFailed",
                Self::NoSecrets => "NoSecrets",
                Self::LoginFailed => "LoginFailed",
                Self::ConnectionRemoved => "ConnectionRemoved",
                Self::DependencyFailed => "DependencyFailed",
                Self::DeviceRealizeFailed => "DeviceRealizeFailed",
                Self::DeviceRemoved => "DeviceRemoved",
                _ => "Unknown",
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum AgentManagerError {
    Failed,
    PermissionDenied,
    InvalidIdentifier,
    NotRegistered,
    NoSecrets,
    UserCanceled,
    __Unknown(i32),
}

impl fmt::Display for AgentManagerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "AgentManagerError::{}",
            match *self {
                Self::Failed => "Failed",
                Self::PermissionDenied => "PermissionDenied",
                Self::InvalidIdentifier => "InvalidIdentifier",
                Self::NotRegistered => "NotRegistered",
                Self::NoSecrets => "NoSecrets",
                Self::UserCanceled => "UserCanceled",
                _ => "Unknown",
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum Capability {
    Team,
    Ovs,
    __Unknown(i32),
}

impl fmt::Display for Capability {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Capability::{}",
            match *self {
                Self::Team => "Team",
                Self::Ovs => "Ovs",
                _ => "Unknown",
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum ClientError {
    Failed,
    ManagerNotRunning,
    ObjectCreationFailed,
    __Unknown(i32),
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ClientError::{}",
            match *self {
                Self::Failed => "Failed",
                Self::ManagerNotRunning => "ManagerNotRunning",
                Self::ObjectCreationFailed => "ObjectCreationFailed",
                _ => "Unknown",
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum ClientPermission {
    None,
    EnableDisableNetwork,
    EnableDisableWifi,
    EnableDisableWwan,
    EnableDisableWimax,
    SleepWake,
    NetworkControl,
    WifiShareProtected,
    WifiShareOpen,
    SettingsModifySystem,
    SettingsModifyOwn,
    SettingsModifyHostname,
    SettingsModifyGlobalDns,
    Reload,
    CheckpointRollback,
    EnableDisableStatistics,
    EnableDisableConnectivityCheck,
    WifiScan,
    __Unknown(i32),
}

impl fmt::Display for ClientPermission {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ClientPermission::{}",
            match *self {
                Self::None => "None",
                Self::EnableDisableNetwork => "EnableDisableNetwork",
                Self::EnableDisableWifi => "EnableDisableWifi",
                Self::EnableDisableWwan => "EnableDisableWwan",
                Self::EnableDisableWimax => "EnableDisableWimax",
                Self::SleepWake => "SleepWake",
                Self::NetworkControl => "NetworkControl",
                Self::WifiShareProtected => "WifiShareProtected",
                Self::WifiShareOpen => "WifiShareOpen",
                Self::SettingsModifySystem => "SettingsModifySystem",
                Self::SettingsModifyOwn => "SettingsModifyOwn",
                Self::SettingsModifyHostname => "SettingsModifyHostname",
                Self::SettingsModifyGlobalDns => "SettingsModifyGlobalDns",
                Self::Reload => "Reload",
                Self::CheckpointRollback => "CheckpointRollback",
                Self::EnableDisableStatistics => "EnableDisableStatistics",
                Self::EnableDisableConnectivityCheck => "EnableDisableConnectivityCheck",
                Self::WifiScan => "WifiScan",
                _ => "Unknown",
            }
        )
    }
}
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum ClientPermissionResult {
    Unknown,
    Yes,
    Auth,
    No,
    __Unknown(i32),
}

impl fmt::Display for ClientPermissionResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ClientPermissionResult::{}",
            match *self {
                Self::Unknown => "Unknown",
                Self::Yes => "Yes",
                Self::Auth => "Auth",
                Self::No => "No",
                _ => "Unknown",
            }
        )
    }
}
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum ConnectionError {
    Failed,
    SettingNotFound,
    PropertyNotFound,
    PropertyNotSecret,
    MissingSetting,
    InvalidSetting,
    MissingProperty,
    InvalidProperty,
    __Unknown(i32),
}

impl fmt::Display for ConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ConnectionError::{}",
            match *self {
                Self::Failed => "Failed",
                Self::SettingNotFound => "SettingNotFound",
                Self::PropertyNotFound => "PropertyNotFound",
                Self::PropertyNotSecret => "PropertyNotSecret",
                Self::MissingSetting => "MissingSetting",
                Self::InvalidSetting => "InvalidSetting",
                Self::MissingProperty => "MissingProperty",
                Self::InvalidProperty => "InvalidProperty",
                _ => "Unknown",
            }
        )
    }
}
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum ConnectionMultiConnect {
    Default,
    Single,
    ManualMultiple,
    Multiple,
    __Unknown(i32),
}

impl fmt::Display for ConnectionMultiConnect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ConnectionMultiConnect::{}",
            match *self {
                Self::Default => "Default",
                Self::Single => "Single",
                Self::ManualMultiple => "ManualMultiple",
                Self::Multiple => "Multiple",
                _ => "Unknown",
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum ConnectivityState {
    Unknown,
    None,
    Portal,
    Limited,
    Full,
    __Unknown(i32),
}

impl fmt::Display for ConnectivityState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ConnectivityState::{}",
            match *self {
                Self::Unknown => "Unknown",
                Self::None => "None",
                Self::Portal => "Portal",
                Self::Limited => "Limited",
                Self::Full => "Full",
                _ => "Unknown",
            }
        )
    }
}
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum CryptoError {
    Failed,
    InvalidData,
    InvalidPassword,
    UnknownCipher,
    DecryptionFailed,
    EncryptionFailed,
    __Unknown(i32),
}

impl fmt::Display for CryptoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "CryptoError::{}",
            match *self {
                Self::Failed => "Failed",
                Self::InvalidData => "InvalidData",
                Self::InvalidPassword => "InvalidPassword",
                Self::UnknownCipher => "UnknownCipher",
                Self::DecryptionFailed => "DecryptionFailed",
                Self::EncryptionFailed => "EncryptionFailed",
                _ => "Unknown",
            }
        )
    }
}
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum DeviceError {
    Failed,
    CreationFailed,
    InvalidConnection,
    IncompatibleConnection,
    NotActive,
    NotSoftware,
    NotAllowed,
    SpecificObjectNotFound,
    VersionIdMismatch,
    MissingDependencies,
    InvalidArgument,
    __Unknown(i32),
}

impl fmt::Display for DeviceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "DeviceError::{}",
            match *self {
                Self::Failed => "Failed",
                Self::CreationFailed => "CreationFailed",
                Self::InvalidConnection => "InvalidConnection",
                Self::IncompatibleConnection => "IncompatibleConnection",
                Self::NotActive => "NotActive",
                Self::NotSoftware => "NotSoftware",
                Self::NotAllowed => "NotAllowed",
                Self::SpecificObjectNotFound => "SpecificObjectNotFound",
                Self::VersionIdMismatch => "VersionIdMismatch",
                Self::MissingDependencies => "MissingDependencies",
                Self::InvalidArgument => "InvalidArgument",
                _ => "Unknown",
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
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

impl fmt::Display for DeviceState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "DeviceState::{}",
            match *self {
                Self::Unknown => "Unknown",
                Self::Unmanaged => "Unmanaged",
                Self::Unavailable => "Unavailable",
                Self::Disconnected => "Disconnected",
                Self::Prepare => "Prepare",
                Self::Config => "Config",
                Self::NeedAuth => "NeedAuth",
                Self::IpConfig => "IpConfig",
                Self::IpCheck => "IpCheck",
                Self::Secondaries => "Secondaries",
                Self::Activated => "Activated",
                Self::Deactivating => "Deactivating",
                Self::Failed => "Failed",
            }
        )
    }
}
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
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
    __Unknown(i32),
}

impl fmt::Display for DeviceStateReason {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "DeviceStateReason::{}",
            match *self {
                Self::None => "None",
                Self::Unknown => "Unknown",
                Self::NowManaged => "NowManaged",
                Self::NowUnmanaged => "NowUnmanaged",
                Self::ConfigFailed => "ConfigFailed",
                Self::IpConfigUnavailable => "IpConfigUnavailable",
                Self::IpConfigExpired => "IpConfigExpired",
                Self::NoSecrets => "NoSecrets",
                Self::SupplicantDisconnect => "SupplicantDisconnect",
                Self::SupplicantConfigFailed => "SupplicantConfigFailed",
                Self::SupplicantFailed => "SupplicantFailed",
                Self::SupplicantTimeout => "SupplicantTimeout",
                Self::PppStartFailed => "PppStartFailed",
                Self::PppDisconnect => "PppDisconnect",
                Self::PppFailed => "PppFailed",
                Self::DhcpStartFailed => "DhcpStartFailed",
                Self::DhcpError => "DhcpError",
                Self::DhcpFailed => "DhcpFailed",
                Self::SharedStartFailed => "SharedStartFailed",
                Self::SharedFailed => "SharedFailed",
                Self::AutoipStartFailed => "AutoipStartFailed",
                Self::AutoipError => "AutoipError",
                Self::AutoipFailed => "AutoipFailed",
                Self::ModemBusy => "ModemBusy",
                Self::ModemNoDialTone => "ModemNoDialTone",
                Self::ModemNoCarrier => "ModemNoCarrier",
                Self::ModemDialTimeout => "ModemDialTimeout",
                Self::ModemDialFailed => "ModemDialFailed",
                Self::ModemInitFailed => "ModemInitFailed",
                Self::GsmApnFailed => "GsmApnFailed",
                Self::GsmRegistrationNotSearching => "GsmRegistrationNotSearching",
                Self::GsmRegistrationDenied => "GsmRegistrationDenied",
                Self::GsmRegistrationTimeout => "GsmRegistrationTimeout",
                Self::GsmRegistrationFailed => "GsmRegistrationFailed",
                Self::GsmPinCheckFailed => "GsmPinCheckFailed",
                Self::FirmwareMissing => "FirmwareMissing",
                Self::Removed => "Removed",
                Self::Sleeping => "Sleeping",
                Self::ConnectionRemoved => "ConnectionRemoved",
                Self::UserRequested => "UserRequested",
                Self::Carrier => "Carrier",
                Self::ConnectionAssumed => "ConnectionAssumed",
                Self::SupplicantAvailable => "SupplicantAvailable",
                Self::ModemNotFound => "ModemNotFound",
                Self::BtFailed => "BtFailed",
                Self::GsmSimNotInserted => "GsmSimNotInserted",
                Self::GsmSimPinRequired => "GsmSimPinRequired",
                Self::GsmSimPukRequired => "GsmSimPukRequired",
                Self::GsmSimWrong => "GsmSimWrong",
                Self::InfinibandMode => "InfinibandMode",
                Self::DependencyFailed => "DependencyFailed",
                Self::Br2684Failed => "Br2684Failed",
                Self::ModemManagerUnavailable => "ModemManagerUnavailable",
                Self::SsidNotFound => "SsidNotFound",
                Self::SecondaryConnectionFailed => "SecondaryConnectionFailed",
                Self::DcbFcoeFailed => "DcbFcoeFailed",
                Self::TeamdControlFailed => "TeamdControlFailed",
                Self::ModemFailed => "ModemFailed",
                Self::ModemAvailable => "ModemAvailable",
                Self::SimPinIncorrect => "SimPinIncorrect",
                Self::NewActivation => "NewActivation",
                Self::ParentChanged => "ParentChanged",
                Self::ParentManagedChanged => "ParentManagedChanged",
                Self::OvsdbFailed => "OvsdbFailed",
                Self::IpAddressDuplicate => "IpAddressDuplicate",
                Self::IpMethodUnsupported => "IpMethodUnsupported",
                Self::SriovConfigurationFailed => "SriovConfigurationFailed",
                Self::PeerNotFound => "PeerNotFound",
                _ => "Unknown",
            }
        )
    }
}
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum DeviceType {
    Unknown,
    Ethernet,
    Wifi,
    Unused1,
    Unused2,
    Bt,
    OlpcMesh,
    Wimax,
    Modem,
    Infiniband,
    Bond,
    Vlan,
    Adsl,
    Bridge,
    Generic,
    Team,
    Tun,
    IpTunnel,
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
    _6lowpan,
    Wireguard,
    WifiP2p,
    Vrf,
    __Unknown(i32),
}

impl fmt::Display for DeviceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "DeviceType::{}",
            match *self {
                Self::Unknown => "Unknown",
                Self::Ethernet => "Ethernet",
                Self::Wifi => "Wifi",
                Self::Unused1 => "Unused1",
                Self::Unused2 => "Unused2",
                Self::Bt => "Bt",
                Self::OlpcMesh => "OlpcMesh",
                Self::Wimax => "Wimax",
                Self::Modem => "Modem",
                Self::Infiniband => "Infiniband",
                Self::Bond => "Bond",
                Self::Vlan => "Vlan",
                Self::Adsl => "Adsl",
                Self::Bridge => "Bridge",
                Self::Generic => "Generic",
                Self::Team => "Team",
                Self::Tun => "Tun",
                Self::IpTunnel => "IpTunnel",
                Self::Macvlan => "Macvlan",
                Self::Vxlan => "Vxlan",
                Self::Veth => "Veth",
                Self::Macsec => "Macsec",
                Self::Dummy => "Dummy",
                Self::Ppp => "Ppp",
                Self::OvsInterface => "OvsInterface",
                Self::OvsPort => "OvsPort",
                Self::OvsBridge => "OvsBridge",
                Self::Wpan => "Wpan",
                Self::_6lowpan => "_6lowpan",
                Self::Wireguard => "Wireguard",
                Self::WifiP2p => "WifiP2p",
                Self::Vrf => "Vrf",
                _ => "Unknown",
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum DeviceCapabilities {
    None = 0x00000000,
    Supported = 0x00000001,
    CarrierDetect = 0x00000002,
    IsSoftware = 0x00000004,
    Sriov = 0x00000008,
}

impl fmt::Display for DeviceCapabilities {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "DeviceCapabilities::{}",
            match *self {
                Self::None => "None",
                Self::Supported => "Supported",
                Self::CarrierDetect => "CarrierDetect",
                Self::IsSoftware => "IsSoftware",
                Self::Sriov => "Sriov",
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
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

impl fmt::Display for DeviceWifiCapabilities {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "DeviceWifiCapabilities::{}",
            match *self {
                Self::None => "None",
                Self::CipherWep40 => "CipherWep40",
                Self::CipherWep104 => "CipherWep104",
                Self::CipherTkip => "CipherTkip",
                Self::CipherCcmp => "CipherCcmp",
                Self::Wpa => "Wpa",
                Self::Rsn => "Rsn",
                Self::Ap => "Ap",
                Self::Adhoc => "Adhoc",
                Self::FreqValid => "FreqValid",
                Self::Freq2GHz => "Freq2GHz",
                Self::Freq5GHz => "Freq5GHz",
                Self::Mesh => "Mesh",
                Self::IbssRsn => "IbssRsn",
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum IPTunnelMode {
    Unknown,
    Ipip,
    Gre,
    Sit,
    Isatap,
    Vti,
    Ip6ip6,
    Ipip6,
    Ip6gre,
    Vti6,
    Gretap,
    Ip6gretap,
    __Unknown(i32),
}

impl fmt::Display for IPTunnelMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "IPTunnelMode::{}",
            match *self {
                Self::Unknown => "Unknown",
                Self::Ipip => "Ipip",
                Self::Gre => "Gre",
                Self::Sit => "Sit",
                Self::Isatap => "Isatap",
                Self::Vti => "Vti",
                Self::Ip6ip6 => "Ip6ip6",
                Self::Ipip6 => "Ipip6",
                Self::Ip6gre => "Ip6gre",
                Self::Vti6 => "Vti6",
                Self::Gretap => "Gretap",
                Self::Ip6gretap => "Ip6gretap",
                _ => "Unknown",
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum KeyfileHandlerType {
    Warn,
    WriteCert,
    __Unknown(i32),
}

impl fmt::Display for KeyfileHandlerType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "KeyfileHandlerType::{}",
            match *self {
                Self::Warn => "Warn",
                Self::WriteCert => "WriteCert",
                _ => "Unknown",
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum KeyfileWarnSeverity {
    Debug,
    Info,
    InfoMissingFile,
    Warn,
    __Unknown(i32),
}

impl fmt::Display for KeyfileWarnSeverity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "KeyfileWarnSeverity::{}",
            match *self {
                Self::Debug => "Debug",
                Self::Info => "Info",
                Self::InfoMissingFile => "InfoMissingFile",
                Self::Warn => "Warn",
                _ => "Unknown",
            }
        )
    }
}
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum ManagerError {
    Failed,
    PermissionDenied,
    UnknownConnection,
    UnknownDevice,
    ConnectionNotAvailable,
    ConnectionNotActive,
    ConnectionAlreadyActive,
    DependencyFailed,
    AlreadyAsleepOrAwake,
    AlreadyEnabledOrDisabled,
    UnknownLogLevel,
    UnknownLogDomain,
    InvalidArguments,
    MissingPlugin,
    __Unknown(i32),
}

impl fmt::Display for ManagerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ManagerError::{}",
            match *self {
                Self::Failed => "Failed",
                Self::PermissionDenied => "PermissionDenied",
                Self::UnknownConnection => "UnknownConnection",
                Self::UnknownDevice => "UnknownDevice",
                Self::ConnectionNotAvailable => "ConnectionNotAvailable",
                Self::ConnectionNotActive => "ConnectionNotActive",
                Self::ConnectionAlreadyActive => "ConnectionAlreadyActive",
                Self::DependencyFailed => "DependencyFailed",
                Self::AlreadyAsleepOrAwake => "AlreadyAsleepOrAwake",
                Self::AlreadyEnabledOrDisabled => "AlreadyEnabledOrDisabled",
                Self::UnknownLogLevel => "UnknownLogLevel",
                Self::UnknownLogDomain => "UnknownLogDomain",
                Self::InvalidArguments => "InvalidArguments",
                Self::MissingPlugin => "MissingPlugin",
                _ => "Unknown",
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum Metered {
    Unknown,
    Yes,
    No,
    GuessYes,
    GuessNo,
    __Unknown(i32),
}

impl fmt::Display for Metered {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Metered::{}",
            match *self {
                Self::Unknown => "Unknown",
                Self::Yes => "Yes",
                Self::No => "No",
                Self::GuessYes => "GuessYes",
                Self::GuessNo => "GuessNo",
                _ => "Unknown",
            }
        )
    }
}

pub enum SecretAgentError {
    Failed,
    PermissionDenied,
    InvalidConnection,
    UserCanceled,
    AgentCanceled,
    NoSecrets,
    __Unknown(i32),
}

impl fmt::Display for SecretAgentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SecretAgentError::{}",
            match *self {
                Self::Failed => "Failed",
                Self::PermissionDenied => "PermissionDenied",
                Self::InvalidConnection => "InvalidConnection",
                Self::UserCanceled => "UserCanceled",
                Self::AgentCanceled => "AgentCanceled",
                Self::NoSecrets => "NoSecrets",
                _ => "Unknown",
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum Setting8021xCKFormat {
    Unknown,
    X509,
    RawKey,
    Pkcs12,
    __Unknown(i32),
}

impl fmt::Display for Setting8021xCKFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Setting8021xCKFormat::{}",
            match *self {
                Self::Unknown => "Unknown",
                Self::X509 => "X509",
                Self::RawKey => "RawKey",
                Self::Pkcs12 => "Pkcs12",
                _ => "Unknown",
            }
        )
    }
}
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum Setting8021xCKScheme {
    Unknown,
    Blob,
    Path,
    Pkcs11,
    __Unknown(i32),
}

impl fmt::Display for Setting8021xCKScheme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Setting8021xCKScheme::{}",
            match *self {
                Self::Unknown => "Unknown",
                Self::Blob => "Blob",
                Self::Path => "Path",
                Self::Pkcs11 => "Pkcs11",
                _ => "Unknown",
            }
        )
    }
}
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum SettingCompareFlags {
    Exact,
    Fuzzy,
    IgnoreId,
    IgnoreSecrets,
    IgnoreAgentOwnedSecrets,
    IgnoreNotSavedSecrets,
    DiffResultWithDefault,
    DiffResultNoDefault,
    IgnoreTimestamp,
    __Unknown(i32),
}

impl fmt::Display for SettingCompareFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SettingCompareFlags::{}",
            match *self {
                Self::Exact => "Exact",
                Self::Fuzzy => "Fuzzy",
                Self::IgnoreId => "IgnoreId",
                Self::IgnoreSecrets => "IgnoreSecrets",
                Self::IgnoreAgentOwnedSecrets => "IgnoreAgentOwnedSecrets",
                Self::IgnoreNotSavedSecrets => "IgnoreNotSavedSecrets",
                Self::DiffResultWithDefault => "DiffResultWithDefault",
                Self::DiffResultNoDefault => "DiffResultNoDefault",
                Self::IgnoreTimestamp => "IgnoreTimestamp",
                _ => "Unknown",
            }
        )
    }
}
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum SettingConnectionAutoconnectSlaves {
    Default,
    No,
    Yes,
    __Unknown(i32),
}

impl fmt::Display for SettingConnectionAutoconnectSlaves {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SettingConnectionAutoconnectSlaves::{}",
            match *self {
                Self::Default => "Default",
                Self::No => "No",
                Self::Yes => "Yes",
                _ => "Unknown",
            }
        )
    }
}
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum SettingConnectionLldp {
    Default,
    Disable,
    EnableRx,
    __Unknown(i32),
}

impl fmt::Display for SettingConnectionLldp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SettingConnectionLldp::{}",
            match *self {
                Self::Default => "Default",
                Self::Disable => "Disable",
                Self::EnableRx => "EnableRx",
                _ => "Unknown",
            }
        )
    }
}
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum SettingConnectionLlmnr {
    Default,
    No,
    Resolve,
    Yes,
    __Unknown(i32),
}

impl fmt::Display for SettingConnectionLlmnr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SettingConnectionLlmnr::{}",
            match *self {
                Self::Default => "Default",
                Self::No => "No",
                Self::Resolve => "Resolve",
                Self::Yes => "Yes",
                _ => "Unknown",
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum SettingConnectionMdns {
    Default,
    No,
    Resolve,
    Yes,
    __Unknown(i32),
}

impl fmt::Display for SettingConnectionMdns {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SettingConnectionMdns::{}",
            match *self {
                Self::Default => "Default",
                Self::No => "No",
                Self::Resolve => "Resolve",
                Self::Yes => "Yes",
                _ => "Unknown",
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum SettingDiffResult {
    Unknown,
    InA,
    InB,
    InADefault,
    InBDefault,
    __Unknown(i32),
}

impl fmt::Display for SettingDiffResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SettingDiffResult::{}",
            match *self {
                Self::Unknown => "Unknown",
                Self::InA => "InA",
                Self::InB => "InB",
                Self::InADefault => "InADefault",
                Self::InBDefault => "InBDefault",
                _ => "Unknown",
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum SettingIP6ConfigAddrGenMode {
    Eui64,
    StablePrivacy,
    __Unknown(i32),
}

impl fmt::Display for SettingIP6ConfigAddrGenMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SettingIP6ConfigAddrGenMode::{}",
            match *self {
                Self::Eui64 => "Eui64",
                Self::StablePrivacy => "StablePrivacy",
                _ => "Unknown",
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum SettingIP6ConfigPrivacy {
    Unknown,
    Disabled,
    PreferPublicAddr,
    PreferTempAddr,
    __Unknown(i32),
}

impl fmt::Display for SettingIP6ConfigPrivacy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SettingIP6ConfigPrivacy::{}",
            match *self {
                Self::Unknown => "Unknown",
                Self::Disabled => "Disabled",
                Self::PreferPublicAddr => "PreferPublicAddr",
                Self::PreferTempAddr => "PreferTempAddr",
                _ => "Unknown",
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum SettingMacRandomization {
    Default,
    Never,
    Always,
    __Unknown(i32),
}

impl fmt::Display for SettingMacRandomization {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SettingMacRandomization::{}",
            match *self {
                Self::Default => "Default",
                Self::Never => "Never",
                Self::Always => "Always",
                _ => "Unknown",
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum SettingMacsecMode {
    Psk,
    Eap,
    __Unknown(i32),
}

impl fmt::Display for SettingMacsecMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SettingMacsecMode::{}",
            match *self {
                Self::Psk => "Psk",
                Self::Eap => "Eap",
                _ => "Unknown",
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum SettingMacsecValidation {
    Disable,
    Check,
    Strict,
    __Unknown(i32),
}

impl fmt::Display for SettingMacsecValidation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SettingMacsecValidation::{}",
            match *self {
                Self::Disable => "Disable",
                Self::Check => "Check",
                Self::Strict => "Strict",
                _ => "Unknown",
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum SettingMacvlanMode {
    Unknown,
    Vepa,
    Bridge,
    Private,
    Passthru,
    Source,
    __Unknown(i32),
}

impl fmt::Display for SettingMacvlanMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SettingMacvlanMode::{}",
            match *self {
                Self::Unknown => "Unknown",
                Self::Vepa => "Vepa",
                Self::Bridge => "Bridge",
                Self::Private => "Private",
                Self::Passthru => "Passthru",
                Self::Source => "Source",
                _ => "Unknown",
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum SettingProxyMethod {
    None,
    Auto,
    __Unknown(i32),
}

impl fmt::Display for SettingProxyMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SettingProxyMethod::{}",
            match *self {
                Self::None => "None",
                Self::Auto => "Auto",
                _ => "Unknown",
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum SettingSerialParity {
    None,
    Even,
    Odd,
    __Unknown(i32),
}

impl fmt::Display for SettingSerialParity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SettingSerialParity::{}",
            match *self {
                Self::None => "None",
                Self::Even => "Even",
                Self::Odd => "Odd",
                _ => "Unknown",
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum SettingTunMode {
    Unknown,
    Tun,
    Tap,
    __Unknown(i32),
}

impl fmt::Display for SettingTunMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SettingTunMode::{}",
            match *self {
                Self::Unknown => "Unknown",
                Self::Tun => "Tun",
                Self::Tap => "Tap",
                _ => "Unknown",
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum SettingWirelessPowersave {
    Default,
    Ignore,
    Disable,
    Enable,
    __Unknown(i32),
}

impl fmt::Display for SettingWirelessPowersave {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SettingWirelessPowersave::{}",
            match *self {
                Self::Default => "Default",
                Self::Ignore => "Ignore",
                Self::Disable => "Disable",
                Self::Enable => "Enable",
                _ => "Unknown",
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum SettingWirelessSecurityFils {
    Default,
    Disable,
    Optional,
    Required,
    __Unknown(i32),
}

impl fmt::Display for SettingWirelessSecurityFils {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SettingWirelessSecurityFils::{}",
            match *self {
                Self::Default => "Default",
                Self::Disable => "Disable",
                Self::Optional => "Optional",
                Self::Required => "Required",
                _ => "Unknown",
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum SettingWirelessSecurityPmf {
    Default,
    Disable,
    Optional,
    Required,
    __Unknown(i32),
}

impl fmt::Display for SettingWirelessSecurityPmf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SettingWirelessSecurityPmf::{}",
            match *self {
                Self::Default => "Default",
                Self::Disable => "Disable",
                Self::Optional => "Optional",
                Self::Required => "Required",
                _ => "Unknown",
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum SettingsError {
    Failed,
    PermissionDenied,
    NotSupported,
    InvalidConnection,
    ReadOnlyConnection,
    UuidExists,
    InvalidHostname,
    InvalidArguments,
    __Unknown(i32),
}

impl fmt::Display for SettingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SettingsError::{}",
            match *self {
                Self::Failed => "Failed",
                Self::PermissionDenied => "PermissionDenied",
                Self::NotSupported => "NotSupported",
                Self::InvalidConnection => "InvalidConnection",
                Self::ReadOnlyConnection => "ReadOnlyConnection",
                Self::UuidExists => "UuidExists",
                Self::InvalidHostname => "InvalidHostname",
                Self::InvalidArguments => "InvalidArguments",
                _ => "Unknown",
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum SriovVFVlanProtocol {
    _1q,
    _1ad,
    __Unknown(i32),
}

impl fmt::Display for SriovVFVlanProtocol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SriovVFVlanProtocol::{}",
            match *self {
                Self::_1q => "_1q",
                Self::_1ad => "_1ad",
                _ => "Unknown",
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
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

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "State::{}",
            match *self {
                Self::Unknown => "Unknown",
                Self::Asleep => "Asleep",
                Self::Disconnected => "Disconnected",
                Self::Disconnecting => "Disconnecting",
                Self::Connecting => "Connecting",
                Self::ConnectedLocal => "ConnectedLocal",
                Self::ConnectedSite => "ConnectedSite",
                Self::ConnectedGlobal => "ConnectedGlobal",
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum Ternary {
    Default = -1,
    False = 0,
    True = 1,
}

impl fmt::Display for Ternary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Ternary::{}",
            match *self {
                Self::Default => "Default",
                Self::False => "False",
                Self::True => "True",
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum UtilsSecurityType {
    Invalid,
    None,
    StaticWep,
    Leap,
    DynamicWep,
    WpaPsk,
    WpaEnterprise,
    Wpa2Psk,
    Wpa2Enterprise,
    Sae,
    Owe,
    Wpa3SuiteB192,
    __Unknown(i32),
}

impl fmt::Display for UtilsSecurityType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "UtilsSecurityType::{}",
            match *self {
                Self::Invalid => "Invalid",
                Self::None => "None",
                Self::StaticWep => "StaticWep",
                Self::Leap => "Leap",
                Self::DynamicWep => "DynamicWep",
                Self::WpaPsk => "WpaPsk",
                Self::WpaEnterprise => "WpaEnterprise",
                Self::Wpa2Psk => "Wpa2Psk",
                Self::Wpa2Enterprise => "Wpa2Enterprise",
                Self::Sae => "Sae",
                Self::Owe => "Owe",
                Self::Wpa3SuiteB192 => "Wpa3SuiteB192",
                _ => "Unknown",
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum VlanPriorityMap {
    IngressMap,
    EgressMap,
    __Unknown(i32),
}

impl fmt::Display for VlanPriorityMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "VlanPriorityMap::{}",
            match *self {
                Self::IngressMap => "IngressMap",
                Self::EgressMap => "EgressMap",
                _ => "Unknown",
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum VpnConnectionState {
    Unknown,
    Prepare,
    NeedAuth,
    Connect,
    IpConfigGet,
    Activated,
    Failed,
    Disconnected,
    __Unknown(i32),
}

impl fmt::Display for VpnConnectionState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "VpnConnectionState::{}",
            match *self {
                Self::Unknown => "Unknown",
                Self::Prepare => "Prepare",
                Self::NeedAuth => "NeedAuth",
                Self::Connect => "Connect",
                Self::IpConfigGet => "IpConfigGet",
                Self::Activated => "Activated",
                Self::Failed => "Failed",
                Self::Disconnected => "Disconnected",
                _ => "Unknown",
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum VpnConnectionStateReason {
    Unknown,
    None,
    UserDisconnected,
    DeviceDisconnected,
    ServiceStopped,
    IpConfigInvalid,
    ConnectTimeout,
    ServiceStartTimeout,
    ServiceStartFailed,
    NoSecrets,
    LoginFailed,
    ConnectionRemoved,
    __Unknown(i32),
}

impl fmt::Display for VpnConnectionStateReason {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "VpnConnectionStateReason::{}",
            match *self {
                Self::Unknown => "Unknown",
                Self::None => "None",
                Self::UserDisconnected => "UserDisconnected",
                Self::DeviceDisconnected => "DeviceDisconnected",
                Self::ServiceStopped => "ServiceStopped",
                Self::IpConfigInvalid => "IpConfigInvalid",
                Self::ConnectTimeout => "ConnectTimeout",
                Self::ServiceStartTimeout => "ServiceStartTimeout",
                Self::ServiceStartFailed => "ServiceStartFailed",
                Self::NoSecrets => "NoSecrets",
                Self::LoginFailed => "LoginFailed",
                Self::ConnectionRemoved => "ConnectionRemoved",
                _ => "Unknown",
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum VpnPluginError {
    Failed,
    StartingInProgress,
    AlreadyStarted,
    StoppingInProgress,
    AlreadyStopped,
    WrongState,
    BadArguments,
    LaunchFailed,
    InvalidConnection,
    InteractiveNotSupported,
    __Unknown(i32),
}

impl fmt::Display for VpnPluginError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "VpnPluginError::{}",
            match *self {
                Self::Failed => "Failed",
                Self::StartingInProgress => "StartingInProgress",
                Self::AlreadyStarted => "AlreadyStarted",
                Self::StoppingInProgress => "StoppingInProgress",
                Self::AlreadyStopped => "AlreadyStopped",
                Self::WrongState => "WrongState",
                Self::BadArguments => "BadArguments",
                Self::LaunchFailed => "LaunchFailed",
                Self::InvalidConnection => "InvalidConnection",
                Self::InteractiveNotSupported => "InteractiveNotSupported",
                _ => "Unknown",
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum VpnPluginFailure {
    LoginFailed,
    ConnectFailed,
    BadIpConfig,
    __Unknown(i32),
}

impl fmt::Display for VpnPluginFailure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "VpnPluginFailure::{}",
            match *self {
                Self::LoginFailed => "LoginFailed",
                Self::ConnectFailed => "ConnectFailed",
                Self::BadIpConfig => "BadIpConfig",
                _ => "Unknown",
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum VpnServiceState {
    Unknown,
    Init,
    Shutdown,
    Starting,
    Started,
    Stopping,
    Stopped,
    __Unknown(i32),
}

impl fmt::Display for VpnServiceState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "VpnServiceState::{}",
            match *self {
                Self::Unknown => "Unknown",
                Self::Init => "Init",
                Self::Shutdown => "Shutdown",
                Self::Starting => "Starting",
                Self::Started => "Started",
                Self::Stopping => "Stopping",
                Self::Stopped => "Stopped",
                _ => "Unknown",
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum WepKeyType {
    Unknown,
    Key,
    Passphrase,
    __Unknown(i32),
}

impl fmt::Display for WepKeyType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "WepKeyType::{}",
            match *self {
                Self::Unknown => "Unknown",
                Self::Key => "Key",
                Self::Passphrase => "Passphrase",
                _ => "Unknown",
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum WimaxNspNetworkType {
    Unknown,
    Home,
    Partner,
    RoamingPartner,
    __Unknown(i32),
}

impl fmt::Display for WimaxNspNetworkType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "WimaxNspNetworkType::{}",
            match *self {
                Self::Unknown => "Unknown",
                Self::Home => "Home",
                Self::Partner => "Partner",
                Self::RoamingPartner => "RoamingPartner",
                _ => "Unknown",
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum IEEE80211Mode {
    Unknown,
    Adhoc,
    Infra,
    Ap,
    Mesh,
    __Unknown(i32),
}

impl fmt::Display for IEEE80211Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "IEEE80211Mode::{}",
            match *self {
                Self::Unknown => "Unknown",
                Self::Adhoc => "Adhoc",
                Self::Infra => "Infra",
                Self::Ap => "Ap",
                Self::Mesh => "Mesh",
                _ => "Unknown",
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum BluetoothCapabilities {
    None = 0x00000000,
    Dun = 0x00000001,
    Nap = 0x00000002,
}

impl fmt::Display for BluetoothCapabilities {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "BluetoothCapabilities::{}",
            match *self {
                Self::None => "None",
                Self::Dun => "Dun",
                Self::Nap => "Nap",
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum DeviceModemCapabilities {
    None = 0x00000000,
    Pots = 0x00000001,
    CdmaEvdo = 0x00000002,
    GsmUmts = 0x00000004,
    Lte = 0x00000008,
}

impl fmt::Display for DeviceModemCapabilities {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "BluetoothCapabilities::{}",
            match *self {
                Self::None => "None",
                Self::Pots => "Pots",
                Self::CdmaEvdo => "CdmaEvdo",
                Self::GsmUmts => "GsmUmts",
                Self::Lte => "Lte",
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum RollbackResult {
    Ok,
    NoDevice,
    DeviceUnmanaged,
    Failed,
}

impl fmt::Display for RollbackResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "RollbackResult::{}",
            match *self {
                Self::Ok => "Ok",
                Self::NoDevice => "NoDevice",
                Self::DeviceUnmanaged => "DeviceUnmanaged",
                Self::Failed => "Failed",
            }
        )
    }
}
