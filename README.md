# reolink-api-rs

Rust client for the Reolink API. It is based on [the CGI/API version 8 (2023-4)](https://support.reolink.com/hc/en-us/articles/900000625763-Introduction-to-CGI-API/).

## Cargo features

- `blocking` (default): provides the blocking `ReolinkClient`
- `chrono` (default): provides `Into` and `From` conversions for the `Time` type.

## Todo

- [ ] Async client
- [ ] Automatically get a token for APIs that require it (e.g. `download`)
- [ ] Automatic token renewal
- [ ] Automatic logout when the client is dropped, to avoid token starvation (each device accepts a limited number of live tokens)
- [ ] Library-specific types/enums where applicable
- [ ] A download API that gives access to headers (e.g. byte-range request header, response content-type)

## Implementation status:

See the [Camera HTTP API Version 8 - 2023-4](docs/Camera_HTTP_API_User_Guide_v8-1.pdf) for details about each API.

System:
- [x] GetAbility
- [ ] GetDevInfo
- [ ] GetDevName
- [ ] SetDevName
- [ ] GetTime
- [ ] SetTime
- [ ] GetAutoMaint
- [ ] SetAutoMaint
- [ ] GetHddInfo
- [ ] Format
- [ ] Upgrade
- [ ] Restore
- [ ] Reboot
- [ ] UpgradePrepare
- [ ] GetAutoUpgrade
- [ ] SetAutoUpgrade
- [ ] CheckFirmware
- [ ] UpgradeOnline
- [ ] UpgradeStatus
- [x] GetChannelStatus

Security:
- [x] Login
- [x] Logout
- [x] GetUser
- [x] AddUser
- [ ] DelUser
- [ ] ModifyUser
- [ ] GetOnline
- [ ] Disconnect
- [ ] GetSysCfg
- [ ] SetSysCfg

Network:
- [ ] GetLocalLink
- [ ] SetLocalLink
- [ ] GetDdns
- [ ] SetDdns
- [ ] GetEmail
- [ ] SetEmail
- [ ] GetEmailV20
- [ ] SetEmailV20
- [ ] TestEmail
- [ ] GetFtp
- [ ] SetFtp
- [ ] GetFtpV20
- [ ] SetFtpV20
- [ ] TestFtp
- [ ] GetNtp
- [ ] SetNtp
- [ ] GetNetPort
- [ ] SetNetPort
- [ ] GetUpnp
- [ ] SetUpnp
- [ ] GetWifi
- [ ] SetWifi
- [ ] TestWifi
- [ ] ScanWifi
- [ ] GetWifiSignal
- [ ] GetPush
- [ ] SetPush
- [ ] GetPushV20
- [ ] SetPushV20
- [ ] GetPushCfg
- [ ] SetPushCfg
- [ ] GetP2p
- [ ] SetP2p
- [ ] GetCertificateInfo
- [ ] CertificateClear
- [ ] GetRtspUrl

Video input:
- [ ] GetImage
- [ ] SetImage
- [ ] GetOsd
- [ ] SetOsd
- [ ] GetIsp
- [ ] SetIsp
- [ ] GetMask
- [ ] SetMask
- [ ] GetCrop
- [ ] SetCrop
- [ ] GetStitch
- [ ] SetStitch

Enc:
- [ ] GetEnc
- [ ] SetEnc

Record:
- [x] GetRec
- [ ] SetRec
- [x] GetRecV20
- [ ] SetRecV20
- [x] Search
- [x] Download
- [x] Snap
- [ ] Playback
- [ ] NvrDownload

PTZ:
- [ ] GetPtzPreset
- [ ] SetPtzPreset
- [ ] GetPtzPatrol
- [ ] SetPtzPatrol
- [ ] PtzCtrl
- [ ] GetPtzSerial
- [ ] SetPtzSerial
- [ ] GetPtzTattern
- [ ] SetPtzTattern
- [ ] GetAutoFocus
- [ ] SetAutoFocus
- [ ] GetZoomFocus
- [ ] StartZoomFocus
- [ ] GetPtzGuard
- [ ] SetPtzGuard
- [ ] GetPtzCheckState
- [ ] PtzCheck

Alarm:
- [ ] GetAlarm
- [ ] SetAlarm
- [ ] GetMdAlarm
- [ ] SetMdAlarm
- [ ] GetMdState
- [ ] GetAudioAlarm
- [ ] SetAudioAlarm
- [ ] GetAudioAlarmV20
- [ ] SetAudioAlarmV20
- [ ] GetBuzzerAlarmV20
- [ ] SetBuzzerAlarmV20
- [ ] AudioAlarmPlay

LED:
- [ ] GetIrLights
- [ ] SetIrLights
- [ ] GetPowerLed
- [ ] SetPowerLed
- [ ] GetWhiteLed
- [ ] SetWhiteLed
- [ ] GetAiAlarm
- [ ] SetAiAlarm
- [ ] SetAlarmArea

AI:
- [ ] GetAiCfg
- [ ] SetAiCfg
- [ ] GetAiState
