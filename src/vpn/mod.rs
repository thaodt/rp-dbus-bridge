pub(crate) mod connection;
pub(crate) mod plugin;

pub type Connection<'a> = crate::vpn::connection::ConnectionProxy<'a>;
pub type Plugin<'a> = crate::vpn::plugin::PluginProxy<'a>;
