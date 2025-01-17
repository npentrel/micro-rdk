// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusResponse {
    #[prost(message, optional, tag="1")]
    pub status: ::core::option::Option<super::super::super::common::v1::BoardStatus>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetGpioRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub pin: ::prost::alloc::string::String,
    #[prost(bool, tag="3")]
    pub high: bool,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetGpioResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGpioRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub pin: ::prost::alloc::string::String,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGpioResponse {
    #[prost(bool, tag="1")]
    pub high: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PwmRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub pin: ::prost::alloc::string::String,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PwmResponse {
    /// 0-1
    #[prost(double, tag="1")]
    pub duty_cycle_pct: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPwmRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub pin: ::prost::alloc::string::String,
    /// 0-1
    #[prost(double, tag="3")]
    pub duty_cycle_pct: f64,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPwmResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PwmFrequencyRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub pin: ::prost::alloc::string::String,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PwmFrequencyResponse {
    #[prost(uint64, tag="1")]
    pub frequency_hz: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPwmFrequencyRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub pin: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub frequency_hz: u64,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPwmFrequencyResponse {
}
// Analog Reader

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadAnalogReaderRequest {
    #[prost(string, tag="1")]
    pub board_name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub analog_reader_name: ::prost::alloc::string::String,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadAnalogReaderResponse {
    #[prost(int32, tag="1")]
    pub value: i32,
}
// Digital Interrupt

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDigitalInterruptValueRequest {
    #[prost(string, tag="1")]
    pub board_name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub digital_interrupt_name: ::prost::alloc::string::String,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDigitalInterruptValueResponse {
    #[prost(int64, tag="1")]
    pub value: i64,
}
// @@protoc_insertion_point(module)
