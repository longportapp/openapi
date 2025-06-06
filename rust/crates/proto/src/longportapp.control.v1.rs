// This file is @generated by prost-build.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Close {
    #[prost(enumeration = "close::Code", tag = "1")]
    pub code: i32,
    #[prost(string, tag = "2")]
    pub reason: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Close`.
pub mod close {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Code {
        /// 心跳超时
        HeartbeatTimeout = 0,
        /// 服务端错误
        ServerError = 1,
        /// 服务端关闭
        ServerShutdown = 2,
        /// 数据截取错误
        UnpackError = 3,
        /// 鉴权失败
        AuthError = 4,
        /// session 过期
        SessExpired = 5,
        /// 单个 session 重复连接
        ConnectDuplicate = 6,
    }
    impl Code {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::HeartbeatTimeout => "HeartbeatTimeout",
                Self::ServerError => "ServerError",
                Self::ServerShutdown => "ServerShutdown",
                Self::UnpackError => "UnpackError",
                Self::AuthError => "AuthError",
                Self::SessExpired => "SessExpired",
                Self::ConnectDuplicate => "ConnectDuplicate",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "HeartbeatTimeout" => Some(Self::HeartbeatTimeout),
                "ServerError" => Some(Self::ServerError),
                "ServerShutdown" => Some(Self::ServerShutdown),
                "UnpackError" => Some(Self::UnpackError),
                "AuthError" => Some(Self::AuthError),
                "SessExpired" => Some(Self::SessExpired),
                "ConnectDuplicate" => Some(Self::ConnectDuplicate),
                _ => None,
            }
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Heartbeat {
    #[prost(int64, tag = "1")]
    pub timestamp: i64,
    #[prost(int32, optional, tag = "2")]
    pub heartbeat_id: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthRequest {
    #[prost(string, tag = "1")]
    pub token: ::prost::alloc::string::String,
    #[prost(map = "string, string", tag = "2")]
    pub metadata: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthResponse {
    #[prost(string, tag = "1")]
    pub session_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub expires: i64,
    #[prost(uint32, tag = "3")]
    pub limit: u32,
    #[prost(uint32, tag = "4")]
    pub online: u32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReconnectRequest {
    #[prost(string, tag = "1")]
    pub session_id: ::prost::alloc::string::String,
    #[prost(map = "string, string", tag = "2")]
    pub metadata: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReconnectResponse {
    #[prost(string, tag = "1")]
    pub session_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub expires: i64,
    #[prost(uint32, tag = "3")]
    pub limit: u32,
    #[prost(uint32, tag = "4")]
    pub online: u32,
}
/// control command, see document: <https://open.longportapp.com/docs/socket/control-command>
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Command {
    CmdClose = 0,
    CmdHeartbeat = 1,
    CmdAuth = 2,
    CmdReconnect = 3,
}
impl Command {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::CmdClose => "CMD_CLOSE",
            Self::CmdHeartbeat => "CMD_HEARTBEAT",
            Self::CmdAuth => "CMD_AUTH",
            Self::CmdReconnect => "CMD_RECONNECT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CMD_CLOSE" => Some(Self::CmdClose),
            "CMD_HEARTBEAT" => Some(Self::CmdHeartbeat),
            "CMD_AUTH" => Some(Self::CmdAuth),
            "CMD_RECONNECT" => Some(Self::CmdReconnect),
            _ => None,
        }
    }
}
