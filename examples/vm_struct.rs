use vmaware::VmBuilder;

fn main() -> Result<(), vmaware::VmawareError> {
    let vm = VmBuilder::new()
        .high_threshold(true)
        .multiple_brands(true)
        .detect()?;

    println!("is vm: {}", vm.is_vm);
    println!("brand: {}", vm.brand);
    println!("type: {}", vm.vm_type);
    println!("conclusion: {}", vm.conclusion);
    println!("percentage: {}%", vm.percentage);
    println!("detected techniques: {}/{}", vm.detected_count, vm.technique_count);

    Ok(())
}
