use num_enum::TryFromPrimitive;

pub enum Response {
    SessionInit([u8; 2]),
    Other,
}
#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum Gid {
    Core(OidCore) = 0x0,
    Session(OidSession) = 0x1,
    Ranging(OidRanging) = 0x2,
    Test(OidTest) = 0xd,
    Unknown = 0xf,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, TryFromPrimitive)]
pub enum OidCore {
    Reset = 0x00,
    DeviceStatus = 0x01,
    GetDeviceInfo = 0x02,
    GetCaps = 0x03,
    SetConfig = 0x04,
    GetConfig = 0x05,
    GenericError = 0x07,
    GetTime = 0x08,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, TryFromPrimitive)]
pub enum OidSession {
    Init = 0x00,
    Deinit = 0x01,
    Status = 0x02,
    SetAppConfig = 0x03,
    GetAppConfig = 0x04,
    GetCount = 0x05,
    GetState = 0x06,
    UpdateMulticastList = 0x07,
    SetAnchorRangingRounds = 0x08,
    SetTagActivity = 0x09,
    GetDataSize = 0xB,
    UpdateHus = 0xC,
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, TryFromPrimitive)]
pub enum OidRanging {
    Start = 0x00,
    Stop = 0x01,
    GetCount = 0x03,
    DataCredit = 0x04,
    DataTransferStatus = 0x05,
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, TryFromPrimitive)]
pub enum OidTest {
    ConfigSet = 0x00,
    ConfigGet = 0x01,
    PeriodicTx = 0x02,
    PerRx = 0x03,
    Rx = 0x05,
    Loopback = 0x06,
    StopSession = 0x07,
    SsTwr = 0x08,
}
#[repr(u8)]
pub enum Status {
    Ok = 0x0,
    Rejected = 0x01,
    Failed = 0x02,
    SyntaxErr = 0x03,
    InvalidParam = 0x04,
    InvalidRange = 0x05,
    InvalidMessageSize = 0x06,
    UnknownGid = 0x07,
    UnknownOid = 0x08,
    ReadOnly = 0x09,
    CommandRetry = 0x0A,
    ErrorSessionNotExist = 0x11,
    ErrorSessionDuplicate = 0x12,
    ErrorSessionActive = 0x13,
    ErrorMaxSessionsExceeded = 0x14,
    ErrorSessionNotConfigured = 0x15,
    ErrorActiveSessionsOngoing = 0x16,
    ErrorMulticastListFull = 0x17,
    ErrorUwbInitializationTimeTooOld = 0x1A,
    RangingNegativeDistance = 0x1B,
    RangingTxFailed = 0x20,
    RangingRxTimeout = 0x21,
    RangingRxPhyDecFailed = 0x22,
    RangingRxPhyToaFailed = 0x23,
    RangingRxPhyStsFailed = 0x24,
    RangingRxMacDecFailed = 0x25,
    RangingRxMacIeDecFailed = 0x26,
    RangingRxMacIeMissing = 0x27,
    ErrorRoundIndexNotActivated = 0x28,
    ErrorNumberOfActiveRoundExceeded = 0x29,
    ErrorDlTdoaDeviceAddressNotMatchingInReplyTimeList = 0x2A,
    ErrorSeBusy = 0x50,
    ErrorCccLifeCycle = 0x51,
    Unknown = 0xFF,
}
#[repr(u32)]
pub enum SessionType {
    Ranging = 0x00,
    RangingAndData = 0x01,
    Data = 0x02,
    RangingPhase = 0x03,
    DataPhase = 0x04,
    RangingAndDataPhase = 0x05,
    HusPrimarySession = 0x9F,
    DeviceTestMode = 0xD0,
    Unknown = 0x100,
}
#[repr(i8)]
pub enum SessionState {
    Init = 0x0,
    DeInit = 0x1,
    Active = 0x2,
    Idle = 0x3,
    Unknown = -1,
}

#[repr(u8)]
#[derive(Debug, Copy, PartialEq, Clone, TryFromPrimitive)]
pub enum Mt {
    DataPacket = 0,
    Command = 1,
    Response = 2,
    Notif = 3,
    Unknown = 255,
}
