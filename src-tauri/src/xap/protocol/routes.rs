// This file is just an "aggregator" for all querys/responses, which are divided into different files, one per subsystem

mod route_imports;
mod xap;
mod qmk;
mod rgblight;

pub use xap::{
    XAPVersion,
    XAPVersionQuery,
    XAPCapabilities,
    XAPCapabilitiesQuery,
    XAPEnabledSubsystems,
    XAPEnabledSubsystemsQuery,
    XAPSecureStatus,
    XAPSecureStatusQuery,
    XAPSecureStatusUnlock,
    XAPSecureStatusLock,
};
pub use qmk::{
    QMKVersion,
    QMKVersionQuery,
    QMKCapabilities,
    QMKCapabilitiesQuery,
};
pub use rgblight::{
    RGBConfig,
    RGBLightCapabilitiesQuery,
    RGBLightEffects,
    RGBLightEffectsQuery,
    RGBLightConfigGet,
    RGBLightConfigSet,
    RGBLightConfigSave,
};