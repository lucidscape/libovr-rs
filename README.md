# Rust bindings for LibOVR (Oculus Rift SDK)

Current Target: SDK 1.3

## Current Limitations
- Requires the 64-bit rust msvc-nightly compiler
- Requires MSVC2015 to link
- Static linked
- OpenGL helpers only

## Low-Level Access
- The raw foreign function interface is available through libovr::ffi
