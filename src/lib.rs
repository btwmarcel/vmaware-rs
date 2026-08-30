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
            "Hypervisor (Type 1)" => VmType::Hv1,
            "Hypervisor (Type 2)" => VmType::Hv2,
            "Hypervisor (unknown type)" => VmType::HvUnknown,
            "Hosted hypervisor / accelerator (type 2)" => VmType::HostedAccelerator,
            "Emulator" => VmType::Emulator,
            "Emulator/Hypervisor (type 2)" => VmType::EmulatorHv2,
            "Paravirtualised/Hypervisor (type 2)" => VmType::Paravirtualised,
            "Sandbox" => VmType::Sandbox,
            "Container" => VmType::Container,
            "Compatibility layer" => VmType::CompatibilityLayer,
            "VM encryptor" => VmType::VmEncryptor,
            "Trusted Domain" => VmType::TrustedDomain,
            "Partitioning Hypervisor" => VmType::PartitioningHv,
            "Cloud VM service" => VmType::CloudVmService,
            "Host machine" => VmType::HyperVRoot,
            "Unknown" => VmType::Unknown,
            "Invalid" => VmType::Invalid,
            other => VmType::Other(other.to_string()),
        }
    }
}

impl std::fmt::Display for VmType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VmType::Hv1 => write!(f, "Hypervisor (Type 1)"),
            VmType::Hv2 => write!(f, "Hypervisor (Type 2)"),
            VmType::HvUnknown => write!(f, "Hypervisor (unknown type)"),
            VmType::HostedAccelerator => write!(f, "Hosted hypervisor / accelerator (Type 2)"),
            VmType::Emulator => write!(f, "Emulator"),
            VmType::EmulatorHv2 => write!(f, "Emulator/Hypervisor (Type 2)"),
            VmType::Paravirtualised => write!(f, "Paravirtualised/Hypervisor (Type 2)"),
            VmType::Sandbox => write!(f, "Sandbox"),
            VmType::Container => write!(f, "Container"),
            VmType::CompatibilityLayer => write!(f, "Compatibility layer"),
            VmType::VmEncryptor => write!(f, "VM encryptor"),
            VmType::TrustedDomain => write!(f, "Trusted Domain"),
            VmType::PartitioningHv => write!(f, "Partitioning Hypervisor"),
            VmType::CloudVmService => write!(f, "Cloud VM service"),
            VmType::HyperVRoot => write!(f, "Host machine"),
            VmType::Unknown => write!(f, "Unknown"),
            VmType::Invalid => write!(f, "Invalid"),
            VmType::Other(s) => write!(f, "{s}"),
        }
    }
}

unsafe extern "C" {
    fn vmaware_detect(out: *mut bool, err: *mut *mut c_char) -> bool;
    fn vmaware_check(flag: u8, out: *mut bool, err: *mut *mut c_char) -> bool;
    fn vmaware_type(out: *mut *mut c_char, err: *mut *mut c_char) -> bool;
    fn vmaware_percentage(out: *mut u8, err: *mut *mut c_char) -> bool;
    fn vmaware_conclusion(out: *mut *mut c_char, err: *mut *mut c_char) -> bool;
    fn vmaware_detected_count(out: *mut u8, err: *mut *mut c_char) -> bool;
    fn vmaware_brand(out: *mut *mut c_char, err: *mut *mut c_char) -> bool;
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
pub fn detect() -> Result<bool, VmawareError> {
    let mut out = false;
    let mut err: *mut c_char = std::ptr::null_mut();

    let ok = unsafe { vmaware_detect(&mut out, &mut err) };

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

/// Fetch the VM brand
pub fn brand() -> Result<String, VmawareError> {
    let mut out: *mut c_char = std::ptr::null_mut();
    let mut err: *mut c_char = std::ptr::null_mut();

    let ok = unsafe { vmaware_brand(&mut out, &mut err) };

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
