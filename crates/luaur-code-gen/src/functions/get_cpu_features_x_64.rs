pub fn get_cpu_features_x_64() -> u32 {
    let mut result: u32 = 0;

    let mut cpuinfo: [i32; 4] = [0, 0, 0, 0];

    if crate::macros::codegen_target_x_64::CODEGEN_TARGET_X64 {
        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        {
            #[cfg(target_arch = "x86_64")]
            {
                #[cfg(target_os = "windows")]
                {
                    unsafe {
                        let mut cpuid_res = [0; 4];
                        core::arch::x86_64::__cpuid(
                            1,
                            &mut cpuid_res[0],
                            &mut cpuid_res[1],
                            &mut cpuid_res[2],
                            &mut cpuid_res[3],
                        );
                        cpuinfo = cpuid_res;
                    }
                }

                #[cfg(not(target_os = "windows"))]
                {
                    unsafe {
                        core::arch::asm!(
                            "cpuid",
                            inlateout("eax") 1 => cpuinfo[0],
                            out("ebx") cpuinfo[1],
                            out("ecx") cpuinfo[2],
                            out("edx") cpuinfo[3],
                            options(nomem, nostack, preserves_flags)
                        );
                    }
                }
            }

            #[cfg(target_arch = "x86")]
            {
                #[cfg(target_os = "windows")]
                {
                    unsafe {
                        let mut cpuid_res = [0; 4];
                        core::arch::x86::__cpuid(
                            1,
                            &mut cpuid_res[0],
                            &mut cpuid_res[1],
                            &mut cpuid_res[2],
                            &mut cpuid_res[3],
                        );
                        cpuinfo = cpuid_res;
                    }
                }

                #[cfg(not(target_os = "windows"))]
                {
                    unsafe {
                        core::arch::asm!(
                            "cpuid",
                            inlateout("eax") 1 => cpuinfo[0],
                            out("ebx") cpuinfo[1],
                            out("ecx") cpuinfo[2],
                            out("edx") cpuinfo[3],
                            options(nomem, nostack, preserves_flags)
                        );
                    }
                }
            }
        }
    }

    let feature_fma3 = crate::enums::features_x_64::FeaturesX64::Feature_FMA3 as u32;
    let feature_avx = crate::enums::features_x_64::FeaturesX64::Feature_AVX as u32;

    if (cpuinfo[2] & 0x00001000) != 0 {
        result |= feature_fma3;
    }

    if (cpuinfo[2] & 0x10000000) != 0 {
        result |= feature_avx;
    }

    result
}
