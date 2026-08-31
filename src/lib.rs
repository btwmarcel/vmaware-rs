use std::ffi::{CStr, c_char};

#[derive(Debug)]
pub enum VmawareError {
    Ffi(String),
    Unknown,
}

impl std::fmt::Display for VmawareError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VmawareError::Ffi(msg) => write!(f, "vmaware error: {msg}"),
            VmawareError::Unknown => write!(f, "vmaware returned an unknown error"),
        }
    }
}

impl std::error::Error for VmawareError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VmType {
    /// A type 1 hypervisor
    Hv1,
    /// A type 2 hypervisor
    Hv2,
    /// A hypervisor of unknown type (Lockheed Martin LMHS)
    HvUnknown,
    /// A hosted hypervisor / accelerator (Intel HAXM)
    HostedAccelerator,
    /// An emulator
    Emulator,
    /// Combined emulator/hypervisor (QEMU)
    EmulatorHv2,
    /// Paravirtualised type 2 hypervisor (User-mode Linux)
    Paravirtualised,
    /// A sandbox
    Sandbox,
    /// A container
    Container,
    /// A compatibility layer (Wine)
    CompatibilityLayer,
    /// A VM encryptor (AMD SEV family)
    VmEncryptor,
    /// A trusted domain (Intel TDX)
    TrustedDomain,
    /// A partitioning hypervisor (Unisys s-Par, Jailhouse)
    PartitioningHv,
    /// A cloud VM service (Google Compute Engine)
    CloudVmService,
    /// The host is running with Hyper-V as a type 1 hypervisor, not as a guest VM
    HyperVRoot,
    /// No type could be determined
    Unknown,
    /// brand_enum::INVALID
    Invalid,
    /// A type not recognised by vmaware-rs currently
    Other(String),
}

impl From<String> for VmType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Hypervisor (Type 1)" => Self::Hv1,
            "Hypervisor (Type 2)" => Self::Hv2,
            "Hypervisor (unknown type)" => Self::HvUnknown,
            "Hosted hypervisor / accelerator (Type 2)" => Self::HostedAccelerator,
            "Emulator" => Self::Emulator,
            "Emulator/Hypervisor (Type 2)" => Self::EmulatorHv2,
            "Paravirtualised/Hypervisor (Type 2)" => Self::Paravirtualised,
            "Sandbox" => Self::Sandbox,
            "Container" => Self::Container,
            "Compatibility layer" => Self::CompatibilityLayer,
            "VM encryptor" => Self::VmEncryptor,
            "Trusted Domain" => Self::TrustedDomain,
            "Partitioning Hypervisor" => Self::PartitioningHv,
            "Cloud VM service" => Self::CloudVmService,
            "Host machine" => Self::HyperVRoot,
            "Unknown" => Self::Unknown,
            "Invalid" => Self::Invalid,
            _ => Self::Other(s),
        }
    }
}

impl From<&str> for VmType {
    fn from(s: &str) -> Self {
        Self::from(s.to_owned())
    }
}

impl std::fmt::Display for VmType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Hv1 => write!(f, "Hypervisor (Type 1)"),
            Self::Hv2 => write!(f, "Hypervisor (Type 2)"),
            Self::HvUnknown => write!(f, "Hypervisor (unknown type)"),
            Self::HostedAccelerator => write!(f, "Hosted hypervisor / accelerator (Type 2)"),
            Self::Emulator => write!(f, "Emulator"),
            Self::EmulatorHv2 => write!(f, "Emulator/Hypervisor (Type 2)"),
            Self::Paravirtualised => write!(f, "Paravirtualised/Hypervisor (Type 2)"),
            Self::Sandbox => write!(f, "Sandbox"),
            Self::Container => write!(f, "Container"),
            Self::CompatibilityLayer => write!(f, "Compatibility layer"),
            Self::VmEncryptor => write!(f, "VM encryptor"),
            Self::TrustedDomain => write!(f, "Trusted Domain"),
            Self::PartitioningHv => write!(f, "Partitioning Hypervisor"),
            Self::CloudVmService => write!(f, "Cloud VM service"),
            Self::HyperVRoot => write!(f, "Host machine"),
            Self::Unknown => write!(f, "Unknown"),
            Self::Invalid => write!(f, "Invalid"),
            Self::Other(s) => write!(f, "{s}"),
        }
    }
}

unsafe extern "C" {
    fn vmaware_detect(out: *mut bool, err: *mut *mut c_char, high_threshold: bool) -> bool;
    fn vmaware_check(flag: u8, out: *mut bool, err: *mut *mut c_char) -> bool;
    fn vmaware_type(out: *mut *mut c_char, err: *mut *mut c_char) -> bool;
    fn vmaware_percentage(out: *mut u8, err: *mut *mut c_char) -> bool;
    fn vmaware_conclusion(out: *mut *mut c_char, err: *mut *mut c_char) -> bool;
    fn vmaware_detected_count(out: *mut u8, err: *mut *mut c_char) -> bool;
    fn vmaware_brand(out: *mut *mut c_char, err: *mut *mut c_char, multiple: bool) -> bool;
    fn free_string(s: *mut c_char);
}

unsafe fn take_ffi_string(ptr: *mut c_char) -> Option<String> {
    if ptr.is_null() {
        None
    } else {
        let s = unsafe { CStr::from_ptr(ptr) }
            .to_string_lossy()
            .into_owned();
        unsafe { free_string(ptr) };
        Some(s)
    }
}

/// Detect if running inside a VM
pub fn detect(high_threshold: bool) -> Result<bool, VmawareError> {
    let mut out = false;
    let mut err: *mut c_char = std::ptr::null_mut();

    let ok = unsafe { vmaware_detect(&mut out, &mut err, high_threshold) };

    if ok {
        unsafe { take_ffi_string(err) };
        Ok(out)
    } else {
        match unsafe { take_ffi_string(err) } {
            Some(e) => Err(VmawareError::Ffi(e)),
            None => Err(VmawareError::Unknown),
        }
    }
}

/// Fetch the VM type
pub fn vm_type() -> Result<VmType, VmawareError> {
    let mut out: *mut c_char = std::ptr::null_mut();
    let mut err: *mut c_char = std::ptr::null_mut();

    let ok = unsafe { vmaware_type(&mut out, &mut err) };

    if ok {
        let value = unsafe { take_ffi_string(out) };
        unsafe { take_ffi_string(err) };

        match value {
            Some(val) => Ok(VmType::from(val)),
            None => Err(VmawareError::Unknown),
        }
    } else {
        unsafe { take_ffi_string(out) };
        match unsafe { take_ffi_string(err) } {
            Some(e) => Err(VmawareError::Ffi(e)),
            None => Err(VmawareError::Unknown),
        }
    }
}

/// Check for a specific technique based on flag argument
pub fn check(flag: flags) -> Result<bool, VmawareError> {
    let mut out: bool = false;
    let mut err: *mut c_char = std::ptr::null_mut();

    let ok = unsafe { vmaware_check(flag as u8, &mut out, &mut err) };

    if ok {
        unsafe { take_ffi_string(err) };
        Ok(out)
    } else {
        match unsafe { take_ffi_string(err) } {
            Some(e) => Err(VmawareError::Ffi(e)),
            None => Err(VmawareError::Unknown),
        }
    }
}

/// Get the percentage of how likely it's a VM
pub fn percentage() -> Result<u8, VmawareError> {
    let mut out: u8 = 0;
    let mut err: *mut c_char = std::ptr::null_mut();

    let ok = unsafe { vmaware_percentage(&mut out, &mut err) };

    if ok {
        unsafe { take_ffi_string(err) };
        Ok(out)
    } else {
        match unsafe { take_ffi_string(err) } {
            Some(e) => Err(VmawareError::Ffi(e)),
            None => Err(VmawareError::Unknown),
        }
    }
}

/// Fetch the conclusion message based on the brand and percentage
pub fn conclusion() -> Result<String, VmawareError> {
    let mut out: *mut c_char = std::ptr::null_mut();
    let mut err: *mut c_char = std::ptr::null_mut();

    let ok = unsafe { vmaware_conclusion(&mut out, &mut err) };

    if ok {
        let value = unsafe { take_ffi_string(out) };
        unsafe { take_ffi_string(err) };

        match value {
            Some(val) => Ok(val),
            None => Err(VmawareError::Unknown),
        }
    } else {
        unsafe { take_ffi_string(out) };
        match unsafe { take_ffi_string(err) } {
            Some(e) => Err(VmawareError::Ffi(e)),
            None => Err(VmawareError::Unknown),
        }
    }
}

/// Fetch the total number of detected techniques
pub fn detected_count() -> Result<u8, VmawareError> {
    let mut out: u8 = 0;
    let mut err: *mut c_char = std::ptr::null_mut();

    let ok = unsafe { vmaware_detected_count(&mut out, &mut err) };

    if ok {
        unsafe { take_ffi_string(err) };
        Ok(out)
    } else {
        match unsafe { take_ffi_string(err) } {
            Some(e) => Err(VmawareError::Ffi(e)),
            None => Err(VmawareError::Unknown),
        }
    }
}

/// Fetch the VM brand(s).
pub fn brand(multiple: bool) -> Result<String, VmawareError> {
    let mut out: *mut c_char = std::ptr::null_mut();
    let mut err: *mut c_char = std::ptr::null_mut();

    let ok = unsafe { vmaware_brand(&mut out, &mut err, multiple) };

    if ok {
        let value = unsafe { take_ffi_string(out) };
        unsafe { take_ffi_string(err) };

        match value {
            Some(val) => Ok(val),
            None => Err(VmawareError::Unknown),
        }
    } else {
        unsafe { take_ffi_string(out) };
        match unsafe { take_ffi_string(err) } {
            Some(e) => Err(VmawareError::Ffi(e)),
            None => Err(VmawareError::Unknown),
        }
    }
}

#[allow(non_camel_case_types, non_snake_case, dead_code)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/flags_bindgen.rs"));
}

pub use bindings::VM_brand_enum as brands;
pub use bindings::VM_enum_flags as flags;
