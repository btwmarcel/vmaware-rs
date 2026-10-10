use crate::{VmType, VmawareError, brand, conclusion, detect, detected_count, percentage, technique_count, vm_type};

/// A detected virtual machine
pub struct Vm {
    pub is_vm: bool,
    pub percentage: u8,
    pub detected_count: u8,
    pub technique_count: u16,
    pub brand: String,
    pub vm_type: VmType,
    pub conclusion: String,
}

/// `Vm` builder
pub struct VmBuilder {
    high_threshold: bool,
    multi_brand: bool,
}

impl VmBuilder {
    /// Create a new `VmBuilder`
    pub fn new() -> Self {
        Self { high_threshold: false, multi_brand: false }
    }

    /// Whether VMAware should have a higher threshold for determining `is_vm`
    pub fn high_threshold(mut self, value: bool) -> Self {
        self.high_threshold = value;
        self
    }

    /// Whether VMAware should detect multiple brands
    pub fn multiple_brands(mut self, value: bool) -> Self {
        self.multi_brand = value;
        self
    }

    /// Run detection and return the results
    pub fn detect(&self) -> Result<Vm, VmawareError> {
        let is_vm = detect(self.high_threshold)?;
        let percentage = percentage()?;
        let detected_count = detected_count()?;
        let technique_count = technique_count();
        let brand = brand(self.multi_brand)?;
        let vm_type = vm_type()?;
        let conclusion = conclusion()?;

        Ok(Vm {
            is_vm,
            percentage,
            detected_count,
            technique_count,
            brand,
            vm_type,
            conclusion
        })
    }
}