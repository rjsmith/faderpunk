//! Adder app
//! 
//! Precision CV adder / quantizer of one or two other output CV jacks belonging to other apps
//! 
//! This app is able to sum the output voltages of up to two other specified output jacks belonging to other apps in the same layout.
//! It simulates the behaviour of an Eurorack precision adder module.
//! 
//! The app adds the one or two sampled output jacks together, optionally quantises the sum, then re-scales the output signal to the required Adder output range.
//! 
//! The output CV range can be configured, and the signal optionally quantized by the global quantizer.
//! 
//! If the summed CV from the two sampled output jacks exceed the equivalent of 10V, the summed CV will be hardclipped to a max 
//! 
//! ## Hardware Mapping
//! 
//! | Control | Function | + Shift | + Fn
//! |---------|----------|---------|------|
//! | Jack 1  | CV out   | N/A     | N/A  |
//! | Fader 1 | N/A      | N/A     | N/A. | 
//! | LED 1 Top | CV output level | CV output level | N/A
//! | LED 1 Bottom | N/A | N/A | N/A
//! | Fn 1    | Mute 1st added channel | Mute 2nd added channel | N/A |
//! 
//! ## Usage Tips
//! 
//! ### Steevio Sequencing
//! Replicate the magic sequencing techniques of the Welsh modular musician, Steevio!:
//! 1. Set up a "Sequencer" 8-channel app with two note patterns with different lengths and tempo (e.g. 5 and 7 steps).
//! 2. Place an "Adder" app somewhere else on the layout, configuring its "1st" and "2nd" Jack channels to the first two "CV Output" jacks of the Sequencer.
//! 3. Set the Adder's output range to 0-10V (to match the fixed 0-10V output range of the Sequencer app).
//! 
//! ### Combine drum triggers
//! Use the Adder to sum the gate or trigger signals from two other app channels (e.g. "Euclid" and "Random Trigger"), replicatng a typical Eurorack "or" logic processor.
//! It's a great way of creating more rhythms from a smaller number of trigger sources.
//! 
//!


use embassy_futures::{
    join::join3, select::{select, select3}
};
use embassy_sync::{blocking_mutex::raw::NoopRawMutex, signal::Signal};
use heapless::Vec;
use libfp::{
    APP_MAX_PARAMS, AppIcon, Brightness, Color, Config, GLOBAL_CHANNELS,     latch::LatchLayer,
Param, Range, Value, ext::FromValue};

use serde::{Deserialize, Serialize};

use crate::app::{App, AppParams, AppStorage, Led, ManagedStorage, ParamStore };

// TODO: Remove from final code
use defmt::info;

pub const CHANNELS: usize = 1;
pub const PARAMS: usize = 8;

// App configuration visible to the configurator
pub static CONFIG: Config<PARAMS> = Config::new(
    "CV Adder",
    "Adds two CV outputs from other channels, with octave offset and optional global quantization",
    Color::Yellow,
    AppIcon::KnobRound,
)
.add_param(Param::bool { name: "Enable Channel A" })
.add_param(Param::i32 { name: "Channel A Jack", min: 1, max: GLOBAL_CHANNELS as i32 })
.add_param(Param::bool { name: "Enable Channel B" })
.add_param(Param::i32 { name: "Channel B Jack", min: 1, max: GLOBAL_CHANNELS as i32 })
.add_param(Param::Color {
    name: "Color",
    variants: &[
        Color::Blue,
        Color::Green,
        Color::Rose,
        Color::Orange,
        Color::Cyan,
        Color::Pink,
        Color::Violet,
        Color::Yellow,
    ],
})
.add_param(Param::Range {
    name: "Range",
    variants: &[Range::_0_10V, Range::_0_5V, Range::_Neg5_5V],
}).add_param(Param::bool { name: "Quantize output" })
.add_param(Param::Enum {
    name: "Mode",
    variants: &["A+B", "A-B", "Max", "Min", "Average"]});

pub struct Params {
    // Will be added if = true
    channel_a_enabled: bool,
    // Output jack number 1-16 to be sampled
    channel_a_jack: i32,
    // Will be added if = true
    channel_b_enabled: bool,
    // Output jack number 1-16 to be sampled
    channel_b_jack: i32,
    // LED colour
    color: Color,
    // Output CV range
    range: Range,
    // Quantize output = true
    quantize: bool,
    // Channel combination mode
    out_mode: usize,
}

impl Default for Params {
    fn default() -> Self {
        Self {
            channel_a_enabled: false,
            channel_a_jack: 1,
            channel_b_enabled: false,
            channel_b_jack: 1,
            color: Color::Yellow,
            range: Range::_0_10V,
            quantize: false,
            out_mode: 0,
        }
    }
}

impl AppParams for Params {
    fn from_values(values: &[Value]) -> Option<Self> {
        if values.len() < PARAMS {
            return None;
        }
        Some(Self {
            channel_a_enabled: bool::from_value(values[0]),
            channel_a_jack: i32::from_value(values[1]),
            channel_b_enabled: bool::from_value(values[2]),
            channel_b_jack: i32::from_value(values[3]),
            color: Color::from_value(values[4]),
            range: Range::from_value(values[5]),
            quantize: bool::from_value(values[6]),
            out_mode: usize::from_value(values[7]),
        })
    }

    fn to_values(&self) -> Vec<Value, APP_MAX_PARAMS> {
        let mut vec = Vec::new();
        vec.push(self.channel_a_enabled.into()).unwrap();
        vec.push(self.channel_a_jack.into()).unwrap();
        vec.push(self.channel_b_enabled.into()).unwrap();
        vec.push(self.channel_b_jack.into()).unwrap();
        vec.push(self.color.into()).unwrap();
        vec.push(self.range.into()).unwrap();
        vec.push(self.quantize.into()).unwrap();
        vec.push(self.out_mode.into()).unwrap();
        vec
    }
}

#[derive(Serialize, Deserialize)]
pub struct Storage {
    channel_a_mute_saved: bool,
    channel_b_mute_saved: bool,
}

impl Default for Storage {
    fn default() -> Self {
        Self {
            channel_a_mute_saved: false,
            channel_b_mute_saved: false,
        }
    }
}

impl AppStorage for Storage {}

// Wrapper task - required for all apps
#[embassy_executor::task(pool_size = 16/CHANNELS)]
pub async fn wrapper(app: App<CHANNELS>, exit_signal: &'static Signal<NoopRawMutex, bool>) {
    let param_store = ParamStore::<Params>::new(app.app_id, app.layout_id);
    let storage = ManagedStorage::<Storage>::new(app.app_id, app.layout_id);

    param_store.load().await;
    storage.load().await;

      let app_loop = async {
        loop {
            select3(
                run(&app, &param_store, &storage),
                param_store.param_handler(),
                storage.saver_task(),
            )
            .await;
        }
    };

    select(app_loop, app.exit_handler(exit_signal)).await;
}

// Main app logic
pub async fn run(app: &App<CHANNELS>,
    params: &ParamStore<Params>,
    storage: &ManagedStorage<Storage>,
) {


    // first_channel and second_channel params are converted to usize in range 0 - (GLOBAL_CHANNELS-1)
    let (channel_a_enabled, channel_a_jack, channel_b_enabled, channel_b_jack, led_color, range, quantize, out_mode) = params
    .query(|p| {
        (
            p.channel_a_enabled,
            p.channel_a_jack,
            p.channel_b_enabled,
            p.channel_b_jack,
            p.color,
            p.range,
            p.quantize,
            p.out_mode,
        )
    });

    let channel_a_safe = (channel_a_jack.clamp(1, GLOBAL_CHANNELS as i32) - 1) as usize;
    let channel_b_safe = (channel_b_jack.clamp(1, GLOBAL_CHANNELS as i32) - 1) as usize;

    let output = app.make_out_jack(0, range).await;
    let fader = app.use_faders();
    let buttons = app.use_buttons();
    let leds = app.use_leds();

    let quantizer = app.use_quantizer(range);
    let channel_a_mute_glob = app.make_global(storage.query(|s| s.channel_a_mute_saved));
    let channel_b_mute_glob = app.make_global(storage.query(|s| s.channel_b_mute_saved));
    let glob_latch_layer = app.make_global(LatchLayer::Main);

    // Set up initial state of LED button
    if channel_a_mute_glob.get() {
        leds.unset(0, Led::Button);
    } else {
        leds.set(0, Led::Button, led_color, Brightness::Mid);
    }

    let main_fut = async {
       
       loop {
            app.delay_millis(1).await;

            let channel_a_active = channel_a_enabled && !channel_a_mute_glob.get();
            let channel_b_active = channel_b_enabled && !channel_b_mute_glob.get();
            let channel_a_use =  channel_a_active && app.start_channel != channel_a_safe;
            let channel_b_use = channel_b_active && app.start_channel != channel_b_safe;
            let mut a: i32 = if channel_a_use { 
                    app.get_out_jack_value(channel_a_safe) as i32
                } else {
                    0
                };
            let mut b:i32 = if channel_b_use { 
                    app.get_out_jack_value(channel_b_safe) as i32
                } else {
                    0
                };

            let out:i32 = if out_mode == 0 {
                a + b    
            } else if out_mode == 1 {
                a - b    
            } else if out_mode == 2 {
                // Max
                if a > b { a } else { b }
            } else if out_mode == 3 {
                // If channel a or b are disabled, effectively remove them from the calculation,
                // making sure that out = zero if BOTH are disabled
                if !channel_a_use { a = 4095; } 
                if !channel_b_use { b = 4095; }
                if !channel_a_use && !channel_b_use {
                    a = 0;
                    b = 0;
                };
                if a < b { a } else { b }
            } else if out_mode == 4 {
                // If both channels active, output their average, else either a or b only
                if channel_a_use && channel_b_use {
                    (a + b) / 2
                } else if channel_a_use && !channel_b_use {
                    a
                } else {
                    b
                }
            } else { 
                0 
            };

            // Hard clip summed values
            let out_safe: u16 = (out.clamp(0, 4095)) as u16;

            // Output CV
            if quantize {
                let out_pitch = quantizer.get_quantized_note(out_safe).await;
                let out = out_pitch.as_counts(range);
                output.set_value(out);
                leds.set(0, Led::Top, led_color, Brightness::Custom((out / 16) as u8));
            } else {
                output.set_value(out_safe);
                leds.set(0, Led::Top, led_color, Brightness::Custom((out_safe / 16) as u8));
            }

            // info!("Summed out: {}, channel A enabled: {}[{}], channel B enabled: {}[{}]", out_safe, channel_a_enabled, channel_a_safe, channel_b_enabled, channel_b_safe);

       }
    };

    let btn_fut = async {
        loop {
            buttons.wait_for_down(0).await;
            if !buttons.is_shift_pressed() {
                // First channel mute
                let muted = storage.modify_and_save(|s| {
                    s.channel_a_mute_saved = !s.channel_a_mute_saved;
                    s.channel_a_mute_saved
                });
                channel_a_mute_glob.set(muted);
                if muted {
                    leds.unset(0, Led::Button);
                } else {
                    leds.set(0, Led::Button, led_color, Brightness::Mid);
                }
            } else {
                // Second channel mute
                let muted = storage.modify_and_save(|s| {
                    s.channel_b_mute_saved = !s.channel_b_mute_saved;
                    s.channel_b_mute_saved
                });
                channel_b_mute_glob.set(muted);
                if muted {
                    leds.unset(0, Led::Button);
                } else {
                    leds.set(0, Led::Button, led_color, Brightness::Mid);
                }
            }
        };       
    };

    let shift_fut = async {
        loop {
            app.delay_millis(1).await;

            glob_latch_layer.set(LatchLayer::from(buttons.is_shift_pressed()));

            // Change state of button when shift is pressed or released to show correct active state of first or second added channels
            if !buttons.is_shift_pressed() {
                let muted = storage.query(|s| s.channel_a_mute_saved);
                if muted {
                    leds.unset(0, Led::Button);
                } else {
                    leds.set(0, Led::Button, led_color, Brightness::Mid);
                }
            } else {
                let muted = storage.query(|s| s.channel_b_mute_saved);
                if muted {
                    leds.unset(0, Led::Button);
                } else {
                    leds.set(0, Led::Button, led_color, Brightness::Mid);
                }
            }
            
        };
    };

    join3(main_fut, btn_fut, shift_fut).await;

}