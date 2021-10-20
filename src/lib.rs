#![allow(incomplete_features)]
#![feature(generic_associated_types)]

use baseplug::{Plugin, ProcessContext, WindowOpenResult};
use baseview::{Size, WindowOpenOptions, WindowScalePolicy};
use dsp::fft::{ForwardFFT, InverseFFT};
use dsp::num_complex::Complex;
use dsp::signal::Signal;
use dsp::spectrum::Spectrum;
use dsp::window;
use iced_baseview::WindowHandle;
use raw_window_handle::HasRawWindowHandle;
use serde::{Deserialize, Serialize};

baseplug::model! {
    #[derive(Debug, Serialize, Deserialize)]
    struct ZaDClipperModel {
        #[model(min = -1.0, max = 1.0)]
        #[parameter(name = "dc_left")]
        dc_left: f32,

        #[model(min = -1.0, max = 1.0)]
        #[parameter(name = "dc_right")]
        dc_right: f32,
        
        #[model(min = 0.0, max = 1.0)]
        #[parameter(name = "dc_amount")]
        dc_amount: f32,
        
        #[model(min = 0.0, max = 24.0)]
        #[parameter(name = "drive", unit = "Decibels", gradient = "Power(0.15)")]
        drive: f32,

        #[model(min = 0.0, max = 100.0)]
        #[parameter(name = "out_gain", label = "%")]
        out_gain: f32,
    }
}

impl Default for ZaDClipperModel {
    fn default() -> Self {
        Self {
            // "gain" is converted from dB to coefficient in the parameter handling code,
            // so in the model here it's a coeff.
            // -0dB == 1.0
            dc_left: 0.0,
            dc_right: 0.0,
            dc_amount: 1.0,
            drive: 1.0,
            out_gain: 1.0,
        }
    }
}

struct ZaDClipper {}

impl Plugin for ZaDClipper {
    const NAME: &'static str = "ZaDClipper";
    const PRODUCT: &'static str = "ZaDCliper";
    const VENDOR: &'static str = "Zavtrone";

    const INPUT_CHANNELS: usize = 2;
    const OUTPUT_CHANNELS: usize = 2;

    type Model = ZaDClipperModel;

    #[inline]
    fn new(_sample_rate: f32, _model: &ZaDClipperModel) -> Self {
        Self {}
    }

    #[inline]
    fn process(&mut self, model: &ZaDClipperModelProcess, ctx: &mut ProcessContext<Self>) {
        let bufsize = ctx.nframes;
        
        let input = ctx.inputs[0].buffers;
        let output = &mut ctx.outputs[0].buffers;

        for i in 0..bufsize {
            output[0][i] = (input[0][i] * model.drive[i] + model.dc_left[i] * model.dc_amount[i]).min(1.0).max(-1.0) * model.out_gain[i] * 0.01 * (1.0 / model.drive[i].sqrt());
            output[1][i] = (input[1][i] * model.drive[i] + model.dc_right[i] * model.dc_amount[i]).min(1.0).max(-1.0) * model.out_gain[i] * 0.01 * (1.0 / model.drive[i].sqrt());
        }
    }
}

baseplug::vst2!(ZaDClipper, b"zDcP");