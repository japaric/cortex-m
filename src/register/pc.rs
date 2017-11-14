//! Program counter

/// Reads the CPU register
#[inline(always)]
pub fn read() -> u32 {
    let r;

    #[cfg(target_arch = "arm")]
    unsafe {
        asm!("mov $0,R15"
             : "=r"(r)
             :
             :
             : "volatile");
    }

    #[cfg(not(target_arch = "arm"))]
    { r = 0; }

    r
}

/// Writes `bits` to the CPU register
#[inline(always)]
pub unsafe fn write(_bits: u32) {
    #[cfg(target_arch = "arm")]
    asm!("mov R15,$0"
         :
         : "r"(_bits)
         :
         : "volatile");
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_should_compile() {
        // Make sure that ARM-specific inline assembly is only included on ARM.
        super::read();
        unsafe { super::write(5); }
    }
}
