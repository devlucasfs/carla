-- Carla Target Definitions
--
-- The values defined in this file do not necessarily guarantee compatibility
-- with the corresponding operating systems, architectures, ABIs, GPUs, or
-- other hardware targets.
--
-- This document was initially generated with the assistance of artificial
-- intelligence and is intended to serve as a preliminary reference for
-- target identification and classification.
--
-- All definitions are subject to review, validation, modification, and
-- expansion as the Carla compiler and its backends evolve.
--
-- Any features, target-specific implementations, compatibility layers,
-- syscall interfaces, ABI definitions, code generation routines, or other
-- components considered necessary will be implemented as required by the
-- project's development goals and supported targets.
--
-- The presence of a value in this file indicates an intended target
-- classification, not a guarantee of implementation, support, or binary
-- compatibility.
--
-- Contributions, corrections, and additions may be made as needed to ensure
-- technical accuracy and consistency with the project's architecture.

-- Operating systems
local Linux, Windows, Darwin, BSD, Solaris, AIX, QNX, Fuchsia, Redox, Haiku, Hurd, Minix, DOS, OS2, Plan9, Inferno, WASI, Emscripten, Zephyr, RTEMS, NuttX, VxWorks, INTEGRITY, ThreadX, FreeRTOS, Contiki, RIOT, NO_OS =
    "linux",      -- Linux kernel; distributions share the Linux syscall interface
    "windows",    -- Windows NT kernel and native system interface
    "darwin",     -- Apple XNU kernel; macOS, iOS and related platforms
    "bsd",        -- BSD family; FreeBSD, OpenBSD, NetBSD and DragonFly BSD require subtarget selection
    "solaris",    -- Solaris and illumos operating system family
    "aix",        -- IBM AIX operating system
    "qnx",        -- QNX Neutrino real-time operating system
    "fuchsia",    -- Fuchsia operating system; Zircon kernel
    "redox",      -- Redox operating system
    "haiku",      -- Haiku operating system
    "hurd",       -- GNU Hurd operating system
    "minix",      -- MINIX operating system
    "dos",        -- DOS-compatible operating systems
    "os2",        -- IBM OS/2 operating system
    "plan9",      -- Plan 9 operating system
    "inferno",    -- Inferno operating system
    "wasi",       -- WebAssembly System Interface
    "emscripten", -- Emscripten runtime and JavaScript interoperability
    "zephyr",     -- Zephyr real-time operating system
    "rtems",      -- RTEMS real-time operating system
    "nuttx",      -- Apache NuttX embedded operating system
    "vxworks",    -- VxWorks real-time operating system
    "integrity",  -- INTEGRITY real-time operating system
    "threadx",    -- ThreadX real-time operating system
    "freertos",   -- FreeRTOS real-time operating system
    "contiki",    -- Contiki embedded operating system
    "riot",       -- RIOT embedded operating system
    "no_os"       -- No operating system; bare-metal execution

-- CPU architectures
local x86, i386, i486, i586, i686, x86_64, amd64, x86_64h, x86_64_v2, x86_64_v3, x86_64_v4 =
    "x86",       -- Generic 32-bit x86 architecture
    "i386",      -- Intel 80386 instruction set
    "i486",      -- Intel 80486 instruction set
    "i586",      -- Pentium-class x86 architecture
    "i686",      -- Pentium Pro-class x86 architecture
    "x86_64",    -- 64-bit x86 architecture
    "amd64",     -- Alternative name for x86-64
    "x86_64h",   -- x86-64 with Haswell-era features
    "x86_64_v2", -- x86-64-v2 microarchitecture feature level
    "x86_64_v3", -- x86-64-v3 microarchitecture feature level
    "x86_64_v4"  -- x86-64-v4 microarchitecture feature level

local arm, armv5, armv6, armv7, armv7a, armv7m, armv7em, armv8, armv8a, armv8m, armv9, armv9a, aarch64, aarch64_be, arm64, arm64e =
    "arm",        -- Generic ARM architecture
    "armv5",      -- ARM version 5
    "armv6",      -- ARM version 6
    "armv7",      -- ARM version 7
    "armv7a",     -- ARMv7-A application profile
    "armv7m",     -- ARMv7-M microcontroller profile
    "armv7em",    -- ARMv7E-M enhanced microcontroller profile
    "armv8",      -- ARM version 8
    "armv8a",     -- ARMv8-A application profile
    "armv8m",     -- ARMv8-M microcontroller profile
    "armv9",      -- ARM version 9
    "armv9a",     -- ARMv9-A application profile
    "aarch64",    -- 64-bit ARM architecture
    "aarch64_be", -- Big-endian 64-bit ARM architecture
    "arm64",      -- Alternative name for AArch64
    "arm64e"      -- Apple ARM64 variant with pointer authentication

local riscv32, riscv32e, riscv64, riscv128 =
    "riscv32",  -- 32-bit RISC-V architecture
    "riscv32e", -- Embedded RISC-V variant with 16 integer registers
    "riscv64",  -- 64-bit RISC-V architecture
    "riscv128"  -- 128-bit RISC-V architecture

local mips, mipsel, mips64, mips64el, mips32r2, mips32r6, mips64r2, mips64r6 =
    "mips",     -- Generic MIPS architecture
    "mipsel",   -- Little-endian MIPS architecture
    "mips64",   -- 64-bit MIPS architecture
    "mips64el", -- Little-endian 64-bit MIPS architecture
    "mips32r2", -- MIPS32 Release 2
    "mips32r6", -- MIPS32 Release 6
    "mips64r2", -- MIPS64 Release 2
    "mips64r6"  -- MIPS64 Release 6

local powerpc, powerpc32, powerpc64, powerpc64le, ppc, ppc64, ppc64le =
    "powerpc",     -- Generic PowerPC architecture
    "powerpc32",   -- 32-bit PowerPC architecture
    "powerpc64",   -- 64-bit PowerPC architecture
    "powerpc64le", -- Little-endian 64-bit PowerPC architecture
    "ppc",         -- PowerPC alias
    "ppc64",       -- 64-bit PowerPC alias
    "ppc64le"      -- Little-endian 64-bit PowerPC alias

local s390, s390x, sparc, sparc64, sparcv9, alpha, ia64, hppa, hppa64, m68k, m68000, m68020, sh, sh2, sh3, sh4, sh4a =
    "s390",   -- IBM System/390 architecture
    "s390x",  -- IBM z/Architecture
    "sparc",  -- SPARC architecture
    "sparc64",-- 64-bit SPARC architecture
    "sparcv9",-- SPARC Version 9
    "alpha",  -- DEC Alpha architecture
    "ia64",   -- Intel Itanium architecture
    "hppa",   -- HP PA-RISC architecture
    "hppa64", -- 64-bit PA-RISC architecture
    "m68k",   -- Motorola 68000 processor family
    "m68000", -- Motorola 68000 processor
    "m68020", -- Motorola 68020 processor
    "sh",     -- SuperH architecture
    "sh2",    -- SuperH-2 architecture
    "sh3",    -- SuperH-3 architecture
    "sh4",    -- SuperH-4 architecture
    "sh4a"    -- SuperH-4A architecture

local loongarch32, loongarch64, e2k, vax, pdp11, csky, openrisc, microblaze, nios2, arc, arcv2, blackfin, tricore, hexagon, kalimba =
    "loongarch32", -- 32-bit LoongArch architecture
    "loongarch64", -- 64-bit LoongArch architecture
    "e2k",         -- Elbrus/E2K processor architecture
    "vax",         -- DEC VAX architecture
    "pdp11",       -- DEC PDP-11 architecture
    "csky",        -- C-SKY processor architecture
    "openrisc",    -- OpenRISC instruction set architecture
    "microblaze",  -- AMD/Xilinx MicroBlaze soft processor
    "nios2",       -- Intel/Altera Nios II soft processor
    "arc",         -- Synopsys ARC processor architecture
    "arcv2",       -- Synopsys ARCv2 architecture
    "blackfin",    -- Analog Devices Blackfin DSP architecture
    "tricore",     -- Infineon TriCore microcontroller architecture
    "hexagon",     -- Qualcomm Hexagon DSP architecture
    "kalimba"      -- Qualcomm Kalimba DSP architecture

local xtensa, xtensa_lx, xtensa_lx6, xtensa_lx7, avr, avr32, msp430, pic, pic16, pic18, pic24, dspic, rl78, rx, v850, h8, m32r, m32c, tic5x, tic6x =
    "xtensa",     -- Tensilica Xtensa architecture
    "xtensa_lx",  -- Tensilica Xtensa LX family
    "xtensa_lx6", -- Xtensa LX6 used in the original ESP32
    "xtensa_lx7", -- Xtensa LX7 used in ESP32-S2 and ESP32-S3
    "avr",        -- 8-bit AVR microcontroller architecture
    "avr32",      -- 32-bit AVR architecture
    "msp430",     -- Texas Instruments MSP430 microcontroller
    "pic",        -- Microchip PIC microcontroller family
    "pic16",      -- Microchip PIC16 family
    "pic18",      -- Microchip PIC18 family
    "pic24",      -- Microchip PIC24 family
    "dspic",      -- Microchip dsPIC digital signal controller
    "rl78",       -- Renesas RL78 microcontroller
    "rx",         -- Renesas RX microcontroller
    "v850",       -- Renesas V850 processor
    "h8",         -- Hitachi/Renesas H8 processor family
    "m32r",       -- Mitsubishi M32R architecture
    "m32c",       -- Renesas M32C microcontroller
    "tic5x",      -- Texas Instruments TMS320C5x DSP
    "tic6x"       -- Texas Instruments TMS320C6x DSP

-- GPU vendors
local gpu_nvidia, gpu_amd, gpu_intel, gpu_apple, gpu_qualcomm, gpu_arm, gpu_imagination, gpu_samsung, gpu_broadcom, gpu_microsoft, gpu_google =
    "gpu_nvidia",      -- NVIDIA GPU vendor
    "gpu_amd",         -- AMD GPU vendor
    "gpu_intel",       -- Intel GPU vendor
    "gpu_apple",       -- Apple GPU vendor
    "gpu_qualcomm",    -- Qualcomm GPU vendor
    "gpu_arm",         -- ARM GPU vendor
    "gpu_imagination", -- Imagination Technologies GPU vendor
    "gpu_samsung",     -- Samsung GPU vendor
    "gpu_broadcom",    -- Broadcom GPU vendor
    "gpu_microsoft",   -- Microsoft GPU-related target
    "gpu_google"       -- Google GPU-related target

-- GPU families
local gpu_mali, gpu_adreno, gpu_powervr, gpu_xclipse, gpu_immortalis, gpu_tegra, gpu_geforce, gpu_quadro, gpu_tesla, gpu_rtx, gpu_radeon, gpu_radeon_pro, gpu_instinct, gpu_arc, gpu_iris, gpu_uhd, gpu_xe, gpu_m1, gpu_m2, gpu_m3, gpu_m4, gpu_m5, gpu_custom =
    "gpu_mali",       -- ARM Mali GPU family
    "gpu_adreno",     -- Qualcomm Adreno GPU family
    "gpu_powervr",    -- Imagination PowerVR GPU family
    "gpu_xclipse",    -- Samsung Xclipse GPU family
    "gpu_immortalis", -- ARM Immortalis GPU family
    "gpu_tegra",      -- NVIDIA Tegra platform GPUs
    "gpu_geforce",    -- NVIDIA GeForce consumer GPUs
    "gpu_quadro",     -- NVIDIA Quadro workstation GPUs
    "gpu_tesla",      -- NVIDIA Tesla compute GPUs
    "gpu_rtx",        -- NVIDIA RTX GPU family
    "gpu_radeon",     -- AMD Radeon GPUs
    "gpu_radeon_pro", -- AMD Radeon Pro workstation GPUs
    "gpu_instinct",   -- AMD Instinct compute accelerators
    "gpu_arc",        -- Intel Arc discrete GPUs
    "gpu_iris",       -- Intel Iris integrated GPUs
    "gpu_uhd",        -- Intel UHD integrated GPUs
    "gpu_xe",         -- Intel Xe graphics architecture
    "gpu_m1",         -- Apple M1 integrated GPU
    "gpu_m2",         -- Apple M2 integrated GPU
    "gpu_m3",         -- Apple M3 integrated GPU
    "gpu_m4",         -- Apple M4 integrated GPU
    "gpu_m5",         -- Apple M5 integrated GPU
    "gpu_custom"      -- Custom or unknown GPU

-- GPU architectures
local gpu_nvidia_sm, gpu_nvidia_ptx, gpu_amd_gcn, gpu_amd_rdna, gpu_amd_cdna, gpu_amd_r600, gpu_intel_gen, gpu_intel_xe =
    "gpu_nvidia_sm", -- NVIDIA Streaming Multiprocessor architecture
    "gpu_nvidia_ptx",-- NVIDIA PTX virtual instruction set
    "gpu_amd_gcn",   -- AMD Graphics Core Next architecture
    "gpu_amd_rdna",  -- AMD RDNA graphics architecture
    "gpu_amd_cdna",  -- AMD CDNA compute architecture
    "gpu_amd_r600",  -- AMD R600 architecture family
    "gpu_intel_gen", -- Intel Gen graphics architecture
    "gpu_intel_xe"   -- Intel Xe graphics architecture

-- GPU APIs and programming platforms
local gpu_cuda, gpu_rocm, gpu_opencl, gpu_vulkan, gpu_metal, gpu_directx, gpu_direct3d, gpu_opengl, gpu_opengles, gpu_webgpu, gpu_webgl, gpu_spirv, gpu_ptx, gpu_amdgcn, gpu_oneapi, gpu_level_zero =
    "gpu_cuda",      -- NVIDIA CUDA compute platform
    "gpu_rocm",      -- AMD ROCm compute platform
    "gpu_opencl",    -- OpenCL cross-vendor compute API
    "gpu_vulkan",    -- Vulkan graphics and compute API
    "gpu_metal",     -- Apple Metal graphics and compute API
    "gpu_directx",   -- Microsoft DirectX API family
    "gpu_direct3d",  -- Microsoft Direct3D graphics API
    "gpu_opengl",    -- OpenGL graphics API
    "gpu_opengles",  -- OpenGL ES embedded graphics API
    "gpu_webgpu",    -- WebGPU graphics and compute API
    "gpu_webgl",     -- WebGL browser graphics API
    "gpu_spirv",     -- SPIR-V intermediate representation
    "gpu_ptx",       -- NVIDIA Parallel Thread Execution ISA
    "gpu_amdgcn",    -- AMD GPU machine instruction architecture
    "gpu_oneapi",    -- Intel oneAPI programming model
    "gpu_level_zero" -- Intel Level Zero low-level GPU API

-- ABI and data models
local abi_system_v, abi_windows, abi_msvc, abi_gnu, abi_eabi, abi_eabihf, abi_aapcs, abi_darwin, abi_musl, abi_itanium, abi_wasm, abi_wasi, abi_android, abi_x32, abi_ilp32, abi_lp64, abi_llp64 =
    "system_v", -- System V ABI family
    "windows",  -- Windows platform ABI
    "msvc",     -- Microsoft Visual C++ ABI conventions
    "gnu",      -- GNU toolchain environment
    "eabi",     -- Embedded Application Binary Interface
    "eabihf",   -- Embedded ABI with hardware floating-point support
    "aapcs",    -- ARM Architecture Procedure Call Standard
    "darwin",   -- Apple Darwin ABI conventions
    "musl",     -- musl libc runtime environment
    "itanium",  -- Itanium C++ ABI
    "wasm",     -- WebAssembly ABI
    "wasi",     -- WebAssembly System Interface ABI
    "android",  -- Android ABI conventions
    "x32",      -- x86-64 instruction set with 32-bit pointers
    "ilp32",    -- 32-bit int, long and pointer data model
    "lp64",     -- 64-bit long and pointer data model
    "llp64"     -- 64-bit pointers with 32-bit long data model

-- Endianness
local endian_little, endian_big, endian_mixed =
    "little", -- Little-endian byte ordering
    "big",    -- Big-endian byte ordering
    "mixed"   -- Mixed or architecture-specific byte ordering

-- Hardware categories
local cpu, gpu, mcu, dsp, fpga, npu, tpu, accelerator, coprocessor =
    "cpu",         -- Central processing unit
    "gpu",         -- Graphics processing unit
    "mcu",         -- Microcontroller unit
    "dsp",         -- Digital signal processor
    "fpga",        -- Field-programmable gate array
    "npu",         -- Neural processing unit
    "tpu",         -- Tensor processing unit
    "accelerator", -- Generic hardware accelerator
    "coprocessor"  -- Auxiliary processor
