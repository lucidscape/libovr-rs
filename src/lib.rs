#[link(name = "LibOVR")]
extern {}

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
pub mod libovr_ffi;

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
pub mod libovr_gl_ffi;

use std::mem;
use std::ptr;

use libovr_ffi::*;

use std::ffi::CStr;
use std::borrow::Cow;

#[repr(i32)]
#[derive(Debug)]
pub enum OvrError {
    /* General errors */
    /// Failure to allocate memory.
    MemoryAllocationFailure    = -1000,
    /// Failure to create a socket.
    SocketCreationFailure      = -1001,
    /// Invalid ovrSession parameter provided.
    InvalidSession             = -1002,
    /// The operation timed out.
    Timeout                    = -1003,
    /// The system or component has not been initialized.
    NotInitialized             = -1004,
    /// Invalid parameter provided. See error info or log for details.
    InvalidParameter           = -1005,
    /// Generic service error. See error info or log for details.
    ServiceError               = -1006,
    /// The given HMD doesn't exist.
    NoHmd                      = -1007,

    /* Audio error range, reserved for Audio errors. */
    /// First Audio error.
    AudioReservedBegin         = -2000,
    /// Failure to find the specified audio device.
    AudioDeviceNotFound        = -2001,
    /// Generic COM error.
    AudioComError              = -2002,
    /// Last Audio error.
    AudioReservedEnd           = -2999,

    /* Initialization errors. */
    /// Generic initialization error.
    Initialize                 = -3000,
    /// Couldn't load LibOVRRT.
    LibLoad                    = -3001,
    /// LibOVRRT version incompatibility.
    LibVersion                 = -3002,
    /// Couldn't connect to the OVR Service.
    ServiceConnection          = -3003,
    /// OVR Service version incompatibility.
    ServiceVersion             = -3004,
    /// The operating system version is incompatible.
    IncompatibleOS             = -3005,
    /// Unable to initialize the HMD display.
    DisplayInit                = -3006,
    /// Unable to start the server. Is it already running?
    ServerStart                = -3007,
    /// Attempting to re-initialize with a different version.
    Reinitialization           = -3008,
    /// Chosen rendering adapters between client and service do not match
    MismatchedAdapters         = -3009,
    /// Calling application has leaked resources
    LeakingResources           = -3010,
    /// Client version too old to connect to service
    ClientVersion              = -3011,
    /// The operating system is out of date.
    OutOfDateOS                = -3012,
    /// The graphics driver is out of date.
    OutOfDateGfxDriver         = -3013,
    /// The graphics hardware is not supported
    IncompatibleGPU            = -3014,
    /// No valid VR display system found.
    NoValidVRDisplaySystem     = -3015,

    /* Hardware errors */
    /// Headset has no bundle adjustment data.
    InvalidBundleAdjustment    = -4000,
    /// The USB hub cannot handle the camera frame bandwidth.
    USBBandwidth               = -4001,
    /// The USB camera is not enumerating at the correct device speed.
    USBEnumeratedSpeed         = -4002,
    /// Unable to communicate with the image sensor.
    ImageSensorCommError       = -4003,
    /// We use this to report various tracker issues that don't fit in an easily classifiable bucket.
    GeneralTrackerFailure      = -4004,
    /// A more than acceptable number of frames are coming back truncated.
    ExcessiveFrameTruncation   = -4005,
    /// A more than acceptable number of frames have been skipped.
    ExcessiveFrameSkipping     = -4006,
    /// The tracker is not receiving the sync signal (cable disconnected?)
    SyncDisconnected           = -4007,
    /// Failed to read memory from the tracker
    TrackerMemoryReadFailure   = -4008,
    /// Failed to write memory from the tracker
    TrackerMemoryWriteFailure  = -4009,
    /// Timed out waiting for a camera frame
    TrackerFrameTimeout        = -4010,
    /// Truncated frame returned from tracker
    TrackerTruncatedFrame      = -4011,
    /// The HMD Firmware is out of date and is unacceptable.
    HMDFirmwareMismatch        = -4100,
    /// The Tracker Firmware is out of date and is unacceptable.
    TrackerFirmwareMismatch    = -4101,
    /// A bootloader HMD is detected by the service
    BootloaderDeviceDetected   = -4102,
    /// The tracker calibration is missing or incorrect
    TrackerCalibrationError    = -4103,
    /// The controller firmware is out of date and is unacceptable
    ControllerFirmwareMismatch = -4104,

    /* Synchronization errors */
    /// Requested async work not yet complete.
    Incomplete                 = -5000,
    /// Requested async work was abandoned and result is incomplete.
    Abandoned                  = -5001,

    /* Rendering errors */
    /// In the event of a system-wide graphics reset or cable unplug this is returned to the app
    DisplayLost                = -6000,

    /* Fatal errors */
    /// A runtime exception occurred. The application is required to shutdown LibOVR and re-initialize it before this error state will be cleared.
    RuntimeException           = -7000,
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

pub struct Texture {
    texture:    *mut libovr_gl_ffi::ovrGLTexture
}

impl Texture {
    pub fn dimensions(&self) -> (usize, usize) {
        unsafe {
            let size = (*(*self.texture).OGL()).Header.TextureSize;
            (size.w as usize, size.h as usize)
        }
    }

    pub fn gl_handle(&self) -> u32 {
        unsafe {
            (*(*self.texture).OGL()).TexId
        }
    }
}

pub struct GlSwapTextureSet {
    set:    *mut ovrSwapTextureSet,
}

impl GlSwapTextureSet {
    pub fn texture_count(&self) -> usize {
        unsafe {
            (*self.set).TextureCount as usize
        }
    }

    pub fn current_index(&self) -> usize {
        unsafe {
            (*self.set).CurrentIndex as usize
        }
    }

    /// Advance the index of the current texture.
    pub fn next_index(&mut self) -> usize {
        unsafe {
            (*self.set).CurrentIndex = ((*self.set).CurrentIndex + 1) % ((*self.set).TextureCount);
            (*self.set).CurrentIndex as usize
        }
    }

    /// Get the texture the current index.
    pub fn current_texture(&self) -> Texture {
        self.get_texture(self.current_index())
    }

    /// Get the texture for the given index.
    pub fn get_texture(&self, index: usize) -> Texture {
        unsafe {
            assert!(index < self.texture_count());
            let textures = (*self.set).Textures as *mut libovr_gl_ffi::ovrGLTexture;
            Texture {
                texture: textures.offset(index as isize)
            }
        }
    }

    pub fn raw(&self) -> *mut ovrSwapTextureSet {
        self.set
    }
}

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

    pub fn create_swap_texture_set_gl(
        &self,
        format: libovr_gl_ffi::GLuint,
        width:  i32,
        height: i32
    ) -> Result<GlSwapTextureSet, OvrError> {
        unsafe {
            let mut texture_set = mem::uninitialized();
            let result = libovr_gl_ffi::ovr_CreateSwapTextureSetGL(
                self.session as libovr_gl_ffi::ovrHmd,
                format,
                width,
                height,
                &mut texture_set);

            if result >= 0 {
                Ok(GlSwapTextureSet {
                    set:    texture_set as *mut ovrSwapTextureSet,
                })
            } else {
                Err(mem::transmute(result))
            }
        }
    }

    pub fn create_mirror_texture_gl(
        &self,
        format: libovr_gl_ffi::GLuint,
        width:  i32,
        height: i32
    ) -> Texture {
        unsafe {
            let mut texture: &mut libovr_gl_ffi::ovrGLTexture = mem::uninitialized();
            libovr_gl_ffi::ovr_CreateMirrorTextureGL(
                self.session as libovr_gl_ffi::ovrHmd,
                format,
                width,
                height,
                mem::transmute(&mut texture));

            Texture {
                texture: texture as *mut libovr_gl_ffi::ovrGLTexture
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

            let result = ovr_SubmitFrame(self.session, frame_index, view_scale_desc, layer_header,
                                         layer_count as u32);
            if result >= 0 {
                Ok(())
            } else {
                Err(mem::transmute(result))
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
            Err(mem::transmute(result))
        }
    }
}

pub fn shutdown() {
    unsafe {
        ovr_Shutdown();
    }
}

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
            Err(mem::transmute(result))
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
