use std::marker::PhantomData;

use esp_idf_hal::gpio::*;
use esp_idf_hal::peripheral::Peripheral;
use esp_idf_sys::{camera::camera_grab_mode_t, *};

use camera::{camera_model_t, camera_pid_t, camera_sccb_addr_t, framesize_t, pixformat_t};

// Camera sensor pids
pub const OV9650_PID: camera_pid_t = 150;
pub const OV7725_PID: camera_pid_t = 119;
pub const OV2640_PID: camera_pid_t = 38;
pub const OV3660_PID: camera_pid_t = 13920;
pub const OV5640_PID: camera_pid_t = 22080;
pub const OV7670_PID: camera_pid_t = 118;
pub const NT99141_PID: camera_pid_t = 5136;
pub const GC2145_PID: camera_pid_t = 8517;
pub const GC032A_PID: camera_pid_t = 9002;
pub const GC0308_PID: camera_pid_t = 155;
pub const BF3005_PID: camera_pid_t = 48;
pub const BF20A6_PID: camera_pid_t = 8358;
pub const SC101IOT_PID: camera_pid_t = 55882;
pub const SC030IOT_PID: camera_pid_t = 39494;
pub const SC031GS_PID: camera_pid_t = 49;

// Camera sensor models
pub const CAMERA_OV7725: camera_model_t = 0;
pub const CAMERA_OV2640: camera_model_t = 1;
pub const CAMERA_OV3660: camera_model_t = 2;
pub const CAMERA_OV5640: camera_model_t = 3;
pub const CAMERA_OV7670: camera_model_t = 4;
pub const CAMERA_NT99141: camera_model_t = 5;
pub const CAMERA_GC2145: camera_model_t = 6;
pub const CAMERA_GC032A: camera_model_t = 7;
pub const CAMERA_GC0308: camera_model_t = 8;
pub const CAMERA_BF3005: camera_model_t = 9;
pub const CAMERA_BF20A6: camera_model_t = 10;
pub const CAMERA_SC101IOT: camera_model_t = 11;
pub const CAMERA_SC030IOT: camera_model_t = 12;
pub const CAMERA_SC031GS: camera_model_t = 13;
pub const CAMERA_MODEL_MAX: camera_model_t = 14;
pub const CAMERA_NONE: camera_model_t = 15;

// Sensor sccb addrs
pub const OV2640_SCCB_ADDR: camera_sccb_addr_t = 48;
pub const OV5640_SCCB_ADDR: camera_sccb_addr_t = 60;
pub const OV3660_SCCB_ADDR: camera_sccb_addr_t = 60;
pub const OV7725_SCCB_ADDR: camera_sccb_addr_t = 33;
pub const OV7670_SCCB_ADDR: camera_sccb_addr_t = 33;
pub const NT99141_SCCB_ADDR: camera_sccb_addr_t = 42;
pub const GC2145_SCCB_ADDR: camera_sccb_addr_t = 60;
pub const GC032A_SCCB_ADDR: camera_sccb_addr_t = 33;
pub const GC0308_SCCB_ADDR: camera_sccb_addr_t = 33;
pub const BF3005_SCCB_ADDR: camera_sccb_addr_t = 110;
pub const BF20A6_SCCB_ADDR: camera_sccb_addr_t = 110;
pub const SC101IOT_SCCB_ADDR: camera_sccb_addr_t = 104;
pub const SC030IOT_SCCB_ADDR: camera_sccb_addr_t = 104;
pub const SC031GS_SCCB_ADDR: camera_sccb_addr_t = 48;

// Pixel output formats sizes for the camera
pub const PIXFORMAT_RGB565: pixformat_t = 0;
pub const PIXFORMAT_YUV422: pixformat_t = 1;
pub const PIXFORMAT_YUV420: pixformat_t = 2;
pub const PIXFORMAT_GRAYSCALE: pixformat_t = 3;
pub const PIXFORMAT_JPEG: pixformat_t = 4;
pub const PIXFORMAT_RGB888: pixformat_t = 5;
pub const PIXFORMAT_RAW: pixformat_t = 6;
pub const PIXFORMAT_RGB444: pixformat_t = 7;
pub const PIXFORMAT_RGB555: pixformat_t = 8;

// Frame capture sizes for the camera
pub const FRAMESIZE_96X96: framesize_t = 0;
pub const FRAMESIZE_QQVGA: framesize_t = 1;
pub const FRAMESIZE_QCIF: framesize_t = 2;
pub const FRAMESIZE_HQVGA: framesize_t = 3;
pub const FRAMESIZE_240X240: framesize_t = 4;
pub const FRAMESIZE_QVGA: framesize_t = 5;
pub const FRAMESIZE_CIF: framesize_t = 6;
pub const FRAMESIZE_HVGA: framesize_t = 7;
pub const FRAMESIZE_VGA: framesize_t = 8;
pub const FRAMESIZE_SVGA: framesize_t = 9;
pub const FRAMESIZE_XGA: framesize_t = 10;
pub const FRAMESIZE_HD: framesize_t = 11;
pub const FRAMESIZE_SXGA: framesize_t = 12;
pub const FRAMESIZE_UXGA: framesize_t = 13;
pub const FRAMESIZE_FHD: framesize_t = 14;
pub const FRAMESIZE_P_HD: framesize_t = 15;
pub const FRAMESIZE_P_3MP: framesize_t = 16;
pub const FRAMESIZE_QXGA: framesize_t = 17;
pub const FRAMESIZE_QHD: framesize_t = 18;
pub const FRAMESIZE_WQXGA: framesize_t = 19;
pub const FRAMESIZE_P_FHD: framesize_t = 20;
pub const FRAMESIZE_QSXGA: framesize_t = 21;
pub const FRAMESIZE_INVALID: framesize_t = 22;

// Camera grab modes
pub const CAMERA_GRAB_WHEN_EMPTY: camera_grab_mode_t = 0;
pub const CAMERA_GRAB_LATEST: camera_grab_mode_t = 1;

pub struct FrameBuffer<'a> {
    fb: *mut camera::camera_fb_t,
    _p: PhantomData<&'a camera::camera_fb_t>,
}

impl<'a> FrameBuffer<'a> {
    pub fn data(&self) -> &'a [u8] {
        unsafe { std::slice::from_raw_parts((*self.fb).buf, (*self.fb).len) }
    }

    pub fn width(&self) -> usize {
        unsafe { (*self.fb).width }
    }

    pub fn height(&self) -> usize {
        unsafe { (*self.fb).height }
    }

    pub fn format(&self) -> camera::pixformat_t {
        unsafe { (*self.fb).format }
    }

    pub fn timestamp(&self) -> camera::timeval {
        unsafe { (*self.fb).timestamp }
    }
}

pub struct CameraSensor<'a> {
    sensor: *mut camera::sensor_t,
    _p: PhantomData<&'a camera::sensor_t>,
}

impl<'a> CameraSensor<'a> {
    pub fn init_status(&self) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).init_status.unwrap()(self.sensor) })
    }
    pub fn reset(&self) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).reset.unwrap()(self.sensor) })
    }
    pub fn set_pixformat(&self, format: camera::pixformat_t) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_pixformat.unwrap()(self.sensor, format) })
    }
    pub fn set_framesize(&self, framesize: camera::framesize_t) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_framesize.unwrap()(self.sensor, framesize) })
    }
    pub fn set_contrast(&self, level: i32) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_contrast.unwrap()(self.sensor, level) })
    }
    pub fn set_brightness(&self, level: i32) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_brightness.unwrap()(self.sensor, level) })
    }
    pub fn set_saturation(&self, level: i32) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_saturation.unwrap()(self.sensor, level) })
    }
    pub fn set_sharpness(&self, level: i32) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_sharpness.unwrap()(self.sensor, level) })
    }
    pub fn set_denoise(&self, level: i32) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_denoise.unwrap()(self.sensor, level) })
    }
    pub fn set_gainceiling(&self, gainceiling: camera::gainceiling_t) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_gainceiling.unwrap()(self.sensor, gainceiling) })
    }
    pub fn set_quality(&self, quality: i32) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_quality.unwrap()(self.sensor, quality) })
    }
    pub fn set_colorbar(&self, enable: bool) -> Result<(), EspError> {
        esp!(unsafe {
            (*self.sensor).set_colorbar.unwrap()(self.sensor, if enable { 1 } else { 0 })
        })
    }
    pub fn set_whitebal(&self, enable: bool) -> Result<(), EspError> {
        esp!(unsafe {
            (*self.sensor).set_whitebal.unwrap()(self.sensor, if enable { 1 } else { 0 })
        })
    }
    pub fn set_gain_ctrl(&self, enable: bool) -> Result<(), EspError> {
        esp!(unsafe {
            (*self.sensor).set_gain_ctrl.unwrap()(self.sensor, if enable { 1 } else { 0 })
        })
    }
    pub fn set_exposure_ctrl(&self, enable: bool) -> Result<(), EspError> {
        esp!(unsafe {
            (*self.sensor).set_exposure_ctrl.unwrap()(self.sensor, if enable { 1 } else { 0 })
        })
    }
    pub fn set_hmirror(&self, enable: bool) -> Result<(), EspError> {
        esp!(unsafe {
            (*self.sensor).set_hmirror.unwrap()(self.sensor, if enable { 1 } else { 0 })
        })
    }
    pub fn set_vflip(&self, enable: bool) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_vflip.unwrap()(self.sensor, if enable { 1 } else { 0 }) })
    }
    pub fn set_aec2(&self, enable: bool) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_aec2.unwrap()(self.sensor, if enable { 1 } else { 0 }) })
    }
    pub fn set_awb_gain(&self, enable: bool) -> Result<(), EspError> {
        esp!(unsafe {
            (*self.sensor).set_awb_gain.unwrap()(self.sensor, if enable { 1 } else { 0 })
        })
    }
    pub fn set_agc_gain(&self, gain: i32) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_agc_gain.unwrap()(self.sensor, gain) })
    }
    pub fn set_aec_value(&self, gain: i32) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_aec_value.unwrap()(self.sensor, gain) })
    }
    pub fn set_special_effect(&self, effect: i32) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_special_effect.unwrap()(self.sensor, effect) })
    }
    pub fn set_wb_mode(&self, mode: i32) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_wb_mode.unwrap()(self.sensor, mode) })
    }
    pub fn set_ae_level(&self, level: i32) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_ae_level.unwrap()(self.sensor, level) })
    }
    pub fn set_dcw(&self, enable: bool) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_dcw.unwrap()(self.sensor, if enable { 1 } else { 0 }) })
    }
    pub fn set_bpc(&self, enable: bool) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_bpc.unwrap()(self.sensor, if enable { 1 } else { 0 }) })
    }
    pub fn set_wpc(&self, enable: bool) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_wpc.unwrap()(self.sensor, if enable { 1 } else { 0 }) })
    }
    pub fn set_raw_gma(&self, enable: bool) -> Result<(), EspError> {
        esp!(unsafe {
            (*self.sensor).set_raw_gma.unwrap()(self.sensor, if enable { 1 } else { 0 })
        })
    }
    pub fn set_lenc(&self, enable: bool) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_lenc.unwrap()(self.sensor, if enable { 1 } else { 0 }) })
    }
    pub fn get_reg(&self, reg: i32, mask: i32) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).get_reg.unwrap()(self.sensor, reg, mask) })
    }
    pub fn set_reg(&self, reg: i32, mask: i32, value: i32) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_reg.unwrap()(self.sensor, reg, mask, value) })
    }
    pub fn set_res_raw(
        &self,
        start_x: i32,
        start_y: i32,
        end_x: i32,
        end_y: i32,
        offset_x: i32,
        offset_y: i32,
        total_x: i32,
        total_y: i32,
        output_x: i32,
        output_y: i32,
        scale: bool,
        binning: bool,
    ) -> Result<(), EspError> {
        esp!(unsafe {
            (*self.sensor).set_res_raw.unwrap()(
                self.sensor,
                start_x,
                start_y,
                end_x,
                end_y,
                offset_x,
                offset_y,
                total_x,
                total_y,
                output_x,
                output_y,
                scale,
                binning,
            )
        })
    }
    pub fn set_pll(
        &self,
        bypass: i32,
        mul: i32,
        sys: i32,
        root: i32,
        pre: i32,
        seld5: i32,
        pclken: i32,
        pclk: i32,
    ) -> Result<(), EspError> {
        esp!(unsafe {
            (*self.sensor).set_pll.unwrap()(
                self.sensor,
                bypass,
                mul,
                sys,
                root,
                pre,
                seld5,
                pclken,
                pclk,
            )
        })
    }
    pub fn set_xclk(&self, timer: i32, xclk: i32) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_xclk.unwrap()(self.sensor, timer, xclk) })
    }
}

pub struct Camera<'a> {
    _p: PhantomData<&'a ()>,
}

impl<'a> Camera<'a> {
    pub fn new(
        pin_pwdn: impl Peripheral<P = impl InputPin + OutputPin> + 'a,
        pin_reset: impl Peripheral<P = impl InputPin + OutputPin> + 'a,
        pin_xclk: impl Peripheral<P = impl InputPin + OutputPin> + 'a,
        pin_d0: impl Peripheral<P = impl InputPin + OutputPin> + 'a,
        pin_d1: impl Peripheral<P = impl InputPin + OutputPin> + 'a,
        pin_d2: impl Peripheral<P = impl InputPin + OutputPin> + 'a,
        pin_d3: impl Peripheral<P = impl InputPin + OutputPin> + 'a,
        pin_d4: impl Peripheral<P = impl InputPin + OutputPin> + 'a,
        pin_d5: impl Peripheral<P = impl InputPin + OutputPin> + 'a,
        pin_d6: impl Peripheral<P = impl InputPin + OutputPin> + 'a,
        pin_d7: impl Peripheral<P = impl InputPin + OutputPin> + 'a,
        pin_vsync: impl Peripheral<P = impl InputPin + OutputPin> + 'a,
        pin_href: impl Peripheral<P = impl InputPin + OutputPin> + 'a,
        pin_pclk: impl Peripheral<P = impl InputPin + OutputPin> + 'a,

        pin_sccb_sda: impl Peripheral<P = impl InputPin + OutputPin> + 'a,
        pin_sccb_scl: impl Peripheral<P = impl InputPin + OutputPin> + 'a,

        frame_size: framesize_t,

        xclk_freq_hz: i32,

        grab_mode: camera::camera_grab_mode_t,

        pixel_format: pixformat_t,

        jpeg_quality: i32, // 0-63
        fb_count: usize,
    ) -> Result<Self, esp_idf_sys::EspError> {
        esp_idf_hal::into_ref!(
            pin_pwdn,
            pin_reset,
            pin_xclk,
            pin_d0,
            pin_d1,
            pin_d2,
            pin_d3,
            pin_d4,
            pin_d5,
            pin_d6,
            pin_d7,
            pin_vsync,
            pin_href,
            pin_pclk,
            pin_sccb_sda,
            pin_sccb_scl
        );
        let config = camera::camera_config_t {
            pin_pwdn: pin_pwdn.pin(),
            pin_reset: pin_reset.pin(),
            pin_xclk: pin_xclk.pin(),

            __bindgen_anon_1: camera::camera_config_t__bindgen_ty_1 {
                pin_sccb_sda: pin_sccb_sda.pin(),
            },
            __bindgen_anon_2: camera::camera_config_t__bindgen_ty_2 {
                pin_sccb_scl: pin_sccb_scl.pin(),
            },

            pin_d0: pin_d0.pin(),
            pin_d1: pin_d1.pin(),
            pin_d2: pin_d2.pin(),
            pin_d3: pin_d3.pin(),
            pin_d4: pin_d4.pin(),
            pin_d5: pin_d5.pin(),
            pin_d6: pin_d6.pin(),
            pin_d7: pin_d7.pin(),
            pin_vsync: pin_vsync.pin(),
            pin_href: pin_href.pin(),
            pin_pclk: pin_pclk.pin(),

            xclk_freq_hz: xclk_freq_hz,
            ledc_timer: esp_idf_sys::ledc_timer_t_LEDC_TIMER_0,
            ledc_channel: esp_idf_sys::ledc_channel_t_LEDC_CHANNEL_0,

            pixel_format: pixel_format,
            frame_size: frame_size,

            jpeg_quality: jpeg_quality,
            fb_count: fb_count,
            grab_mode: grab_mode,

            ..Default::default()
        };

        esp_idf_sys::esp!(unsafe { camera::esp_camera_init(&config) })?;
        Ok(Self { _p: PhantomData })
    }

    pub fn get_framebuffer(&self) -> Option<FrameBuffer> {
        let fb = unsafe { camera::esp_camera_fb_get() };
        return if fb.is_null() {
            None
        } else {
            Some(FrameBuffer {
                fb,
                _p: PhantomData,
            })
        };
    }

    pub fn sensor(&self) -> CameraSensor<'a> {
        CameraSensor {
            sensor: unsafe { camera::esp_camera_sensor_get() },
            _p: PhantomData,
        }
    }
}

impl<'a> Drop for Camera<'a> {
    fn drop(&mut self) {
        esp!(unsafe { camera::esp_camera_deinit() }).expect("error during esp_camera_deinit")
    }
}

