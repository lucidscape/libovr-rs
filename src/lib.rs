#[link(name = "LibOVR")]
extern {}

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
pub mod ffi;

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
pub mod ffi_gl;

use std::mem;
use std::ptr;

use ffi::*;

use std::ffi::CStr;
use std::borrow::Cow;

#[derive(Debug)]
pub struct OvrError {
    error:  ovrErrorType
}

impl From<ovrErrorType> for OvrError {
    fn from(e: ovrErrorType) -> OvrError {
        OvrError {
            error:  e
        }
    }
}

impl From<i32> for OvrError {
    fn from(e: i32) -> OvrError {
        unsafe {
            OvrError {
                error:  mem::transmute(e)
            }
        }
    }
}


pub const EYES: [ovrEyeType; 2] = [
    Enum_ovrEyeType_::ovrEye_Left,
    Enum_ovrEyeType_::ovrEye_Right
];

impl std::fmt::Display for OvrError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for OvrError {
    fn description(&self) -> &str {
        match *self {
            _ => "Unknown error"
        }
    }
}

/// Mirror texture description
pub struct MirrorTextureDesc {
    desc:   ovrMirrorTextureDesc
}

impl MirrorTextureDesc {
    pub fn new(format: ovrTextureFormat, width: usize, height: usize, flags: u32) -> Self {
        let mut desc    = ovrMirrorTextureDesc::default();
        desc.Format     = format;
        desc.Width      = width as i32;
        desc.Height     = height as i32;
        desc.MiscFlags  = flags;
        MirrorTextureDesc {
            desc: desc
        }
    }
}

pub trait HmdDesc {
    fn product_name(&self) -> Cow<str>;
    fn manufacturer(&self) -> Cow<str>;
}

impl HmdDesc for ovrHmdDesc {
    fn product_name(&self) -> Cow<str> {
        unsafe {
            CStr::from_ptr(&self.ProductName as *const i8).to_string_lossy()
        }
    }

    fn manufacturer(&self) -> Cow<str> {
        unsafe {
            CStr::from_ptr(&self.Manufacturer as *const i8).to_string_lossy()
        }
    }
}

pub enum GraphicsApi {
    OpenGL,
    D3D
}

pub struct GlMirrorTexture {
    session:    ovrSession,
    texture:    ovrMirrorTexture,
}

impl GlMirrorTexture {
    /// Get the OpenGL texture handle for this texture.
    pub fn get_texture_gl(&self) -> u32 {
        unsafe {
            let mut tex_id = 0;
            ffi_gl::ovr_GetMirrorTextureBufferGL(
                self.session as ffi_gl::ovrSession,
                self.texture as ffi_gl::ovrMirrorTexture,
                &mut tex_id);

            tex_id
        }
    }
}

pub struct TextureSwapChainDesc {
    desc:   ovrTextureSwapChainDesc
}

impl TextureSwapChainDesc {
    pub fn new(
        tex_type:       ovrTextureType,
        format:         ovrTextureFormat,
        array_size:     usize,
        width:          usize,
        height:         usize,
        mip_levels:     usize,
        sample_count:   usize,
        static_image:   bool,
        misc_flags:     u32,
        bind_flags:     u32
    ) -> Self {
        TextureSwapChainDesc {
            desc: ovrTextureSwapChainDesc {
                Type:          tex_type,
                Format:        format,
                ArraySize:     array_size as i32,
                Width:         width as i32,
                Height:        height as i32,
                MipLevels:     mip_levels as i32,
                SampleCount:   sample_count as i32,
                StaticImage:   static_image as ovrBool,
                MiscFlags:     misc_flags,
                BindFlags:     bind_flags
            }
        }
    }

    pub fn width(&self) -> usize {
        self.desc.Width as usize
    }

    pub fn height(&self) -> usize {
        self.desc.Height as usize
    }

}

pub struct GlTextureSwapChain {
    session:    ovrSession,
    chain:      ovrTextureSwapChain,
}

impl GlTextureSwapChain {
    pub fn len(&self) -> usize {
        unsafe {
            let mut length = 0;
            ovr_GetTextureSwapChainLength(self.session, self.chain, &mut length);
            length as usize
        }
    }

    pub fn current_index(&self) -> usize {
        unsafe {
            let mut index = 0;
            ovr_GetTextureSwapChainCurrentIndex(self.session, self.chain, &mut index);
            index as usize
        }
    }

    /// Get the OpenGL texture handle for the given texture index.
    pub fn get_texture_gl(&self, index: usize) -> u32 {
        unsafe {
            let mut tex_id = 0;
            ffi_gl::ovr_GetTextureSwapChainBufferGL(
                self.session as ffi_gl::ovrSession,
                self.chain as ffi_gl::ovrTextureSwapChain,
                index as i32,
                &mut tex_id);

            tex_id
        }
    }

    pub fn raw(&self) -> ovrTextureSwapChain {
        self.chain
    }

    pub fn desc(&self) -> TextureSwapChainDesc {
        unsafe {
            let mut desc = mem::zeroed::<ovrTextureSwapChainDesc>();
            ovr_GetTextureSwapChainDesc(self.session, self.chain, &mut desc);
            TextureSwapChainDesc {
                desc:   desc
            }
        }
    }

    pub fn commit(&self) {
        unsafe {
            ovr_CommitTextureSwapChain(self.session, self.chain);
        }
    }
}

/// Session is the main interaction point for the api.
pub struct Session {
    session:    ovrSession
}

impl Session {
    pub fn get_hmd_desc(&self) -> ovrHmdDesc {
        unsafe {
            ovr_GetHmdDesc(self.session)
        }
    }

    pub fn get_fov_texture_size(
        &self,
        eye:        ovrEyeType,
        fov_port:   ovrFovPort,
        pixels_per_display_pixel: f32
    ) -> (usize, usize) {
        unsafe {
            let size = ovr_GetFovTextureSize(self.session, eye, fov_port, pixels_per_display_pixel);
            (size.w as usize, size.h as usize)
        }
    }

    pub fn get_render_desc(&self, eye: ovrEyeType, fov: ovrFovPort) -> ovrEyeRenderDesc {
        unsafe {
            ovr_GetRenderDesc(self.session, eye, fov)
        }
    }

    pub fn get_float(&self, property: *const i8, default_value: f32) -> f32 {
        unsafe {
            ovr_GetFloat(self.session, property, default_value)
        }
    }

    pub fn get_predicted_display_time(&self, frame_index: i64) -> f64 {
        unsafe {
            ovr_GetPredictedDisplayTime(self.session, frame_index)
        }
    }

    pub fn get_tracking_state(&self, abs_time: f64, latency_marker: bool) -> ovrTrackingState {
        unsafe {
            ovr_GetTrackingState(self.session, abs_time, latency_marker as i8)
        }
    }

    /// Returns (eye poses, sensor sample time)
    pub fn get_eye_poses(
        &self,
        frame_index:        u64,
        latency_marker:     bool,
        hmd_to_eye_offset: [ovrVector3f; 2]
    ) -> ([ovrPosef; 2], f64) {
        unsafe {
            let mut hmd_to_eye_offset = hmd_to_eye_offset;
            let mut eye_poses = mem::zeroed::<[ovrPosef; 2]>();
            let mut sensor_sample_time = 0.0;

            ovr_GetEyePoses(
                self.session,
                frame_index as i64,
                latency_marker as i8,
                mem::transmute(&mut hmd_to_eye_offset),
                mem::transmute(&mut eye_poses),
                &mut sensor_sample_time
            );

            (eye_poses, sensor_sample_time)
        }
    }

    pub fn create_texture_swap_chain_gl(
        &self,
        desc:   TextureSwapChainDesc
    ) -> Result<GlTextureSwapChain, OvrError> {
        unsafe {
            let mut texture_chain = mem::zeroed::<ffi_gl::ovrTextureSwapChain>();
            let result =
                ffi_gl::ovr_CreateTextureSwapChainGL(
                    self.session as ffi_gl::ovrSession,
                    mem::transmute(&desc.desc),
                    &mut texture_chain);

            if result >= 0 {
                Ok(GlTextureSwapChain {
                    session:    self.session,
                    chain:      texture_chain as ovrTextureSwapChain,
                })
            } else {
                Err(result.into())
            }
        }
    }

    pub fn create_mirror_texture_gl(
        &self,
        desc: MirrorTextureDesc
    ) -> Result<GlMirrorTexture, OvrError> {
        unsafe {
            let mut texture = mem::zeroed::<ffi_gl::ovrMirrorTexture>();
            let result =
                ffi_gl::ovr_CreateMirrorTextureGL(
                    self.session as ffi_gl::ovrSession,
                    mem::transmute(&desc.desc),
                    &mut texture);

            if result > 0 {
                Ok(GlMirrorTexture {
                    session: self.session,
                    texture: texture as ovrMirrorTexture
                })
            } else {
                Err(result.into())
            }
        }
    }

    pub fn submit_frame(
        &self,
        frame_index:        i64,
        view_scale_desc:    Option<&ovrViewScaleDesc>,
        layer_header:       *const *const ovrLayerHeader,
        layer_count:        usize
    ) -> Result<(), OvrError> {
        unsafe {
            let view_scale_desc =
                match view_scale_desc {
                    Some(desc) => desc as *const ovrViewScaleDesc,
                    None => ptr::null()
                };

            let result =
                ovr_SubmitFrame(
                    self.session,
                    frame_index,
                    view_scale_desc,
                    layer_header,
                    layer_count as u32);

            if result >= 0 {
                Ok(())
            } else {
                Err(result.into())
            }
        }
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        unsafe {
            ovr_Destroy(self.session);
        }
    }
}

/// Initialize the runtime.
pub fn initialize() -> Result<(), OvrError> {
    unsafe {
        let mut params = ovrInitParams {
                Flags:                  0,
                RequestedMinorVersion:  0,
                LogCallback:            None,
                UserData:               0,
                ConnectionTimeoutMS:    1000,
                pad0:                   mem::uninitialized()
        };
        let result = ovr_Initialize(&mut params);
        if result >= 0 {
            Ok(())
        } else {
            Err(result.into())
        }
    }
}

/// Shut down the runtime.
pub fn shutdown() {
    unsafe {
        ovr_Shutdown();
    }
}

/// Try and create a session.
pub fn create() -> Result<Session, OvrError> {
    unsafe {
        let mut session = mem::uninitialized();
        let mut luid = mem::uninitialized();
        let result = ovr_Create(&mut session, &mut luid);
        if result >= 0 {
            Ok(Session {
                session: session
            })
        } else {
            Err(result.into())
        }
    }
}

pub fn get_time_in_seconds() -> f64 {
    unsafe {
        ovr_GetTimeInSeconds()
    }
}

// TODO: Move to a HeadPose struct
pub fn calc_eye_poses(head_pose: ovrPosef, view_offset: [ovrVector3f; 2]) -> [ovrPosef; 2] {
    unsafe {
        let mut view_offset = view_offset;
        let mut eye_poses: [ovrPosef; 2] = mem::uninitialized();
        ovr_CalcEyePoses(head_pose, view_offset.as_mut_ptr(), eye_poses.as_mut_ptr());
        eye_poses
    }
}

pub struct DetectResult {
    result: ovrDetectResult
}

impl DetectResult {
    pub fn is_oculus_service_running(&self) -> bool {
        self.result.IsOculusServiceRunning != 0
    }

    pub fn is_hmd_connected(&self) -> bool {
        self.result.IsOculusHMDConnected != 0
    }
}

/// Detect the presence of the runtime / HMD connection.
pub fn detect(timeout_ms: i32) -> DetectResult {
    unsafe {
        DetectResult {
            result: ovr_Detect(timeout_ms)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let detect_result = detect(1000);
        assert!(detect_result.is_oculus_service_running());
        assert!(detect_result.is_hmd_connected());

        initialize().expect("init ok");

        let session = create().expect("create hmd");

        let _desc = session.get_hmd_desc();
        // println!("--> {} {}", desc.product_name(), desc.manufacturer());
        // assert!(false);

        // let swap_set = session.create_swap_texture_set_gl(0, 512, 512);

        shutdown();
    }
}
