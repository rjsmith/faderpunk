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

use crate::app::{App, AppParams, AppStorage, Led, ManagedStorage, ParamStore, OutJack, SceneEvent};

// TODO: Remove from final code
use defmt::info;

pub const CHANNELS: usize = 1;
pub const PARAMS: usize = 7;

// App configuration visible to the configurator
pub static CONFIG: Config<PARAMS> = Config::new(
    "CV Adder",
    "Adds two CV outputs from other channels, with octave offset and optional global quantization",
    Color::Yellow,
    AppIcon::KnobRound,
)
.add_param(Param::bool { name: "Enable 1st Channel" })
.add_param(Param::i32 { name: "1st Jack Channel", min: 1, max: GLOBAL_CHANNELS as i32 })
.add_param(Param::bool { name: "Enable 2nd Channel" })
.add_param(Param::i32 { name: "2nd Jack Channel", min: 1, max: GLOBAL_CHANNELS as i32 })
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
}).add_param(Param::bool { name: "Quantize output" });

pub struct Params {
    // Will be added if = true
    first_channel_enabled: bool,
    // Output jack number 1-16 to be sampled
    first_channel: i32,
    // Will be added if = true
    second_channel_enabled: bool,
    // Output jack number 1-16 to be sampled
    second_channel: i32,
    // LED colour
    color: Color,
    // Output CV range
    range: Range,
    // Quantize output = true
    quantize: bool,
}

impl Default for Params {
    fn default() -> Self {
        Self {
            first_channel_enabled: false,
            first_channel: 1,
            second_channel_enabled: false,
            second_channel: 1,
            color: Color::Yellow,
            range: Range::_0_10V,
            quantize: false,
        }
    }
}

impl AppParams for Params {
    fn from_values(values: &[Value]) -> Option<Self> {
        if values.len() < PARAMS {
            return None;
        }
        Some(Self {
            first_channel_enabled: bool::from_value(values[0]),
            first_channel: i32::from_value(values[1]),
            second_channel_enabled: bool::from_value(values[2]),
            second_channel: i32::from_value(values[3]),
            color: Color::from_value(values[4]),
            range: Range::from_value(values[5]),
            quantize: bool::from_value(values[6]),
        })
    }

    fn to_values(&self) -> Vec<Value, APP_MAX_PARAMS> {
        let mut vec = Vec::new();
        vec.push(self.first_channel_enabled.into()).unwrap();
        vec.push(self.first_channel.into()).unwrap();
        vec.push(self.second_channel_enabled.into()).unwrap();
        vec.push(self.second_channel.into()).unwrap();
        vec.push(self.color.into()).unwrap();
        vec.push(self.range.into()).unwrap();
        vec.push(self.quantize.into()).unwrap();
        vec
    }
}

#[derive(Serialize, Deserialize)]
pub struct Storage {
    first_channel_mute_saved: bool,
    second_channel_mute_saved: bool,
}

impl Default for Storage {
    fn default() -> Self {
        Self {
            first_channel_mute_saved: false,
            second_channel_mute_saved: false,
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
    let (first_channel_enabled, first_channel, second_channel_enabled, second_channel, led_color, range, quantize) = params
    .query(|p| {
        (
            p.first_channel_enabled,
            p.first_channel,
            p.second_channel_enabled,
            p.second_channel,
            p.color,
            p.range,
            p.quantize
        )
    });

    let first_channel_safe = (first_channel.clamp(1, GLOBAL_CHANNELS as i32) - 1) as usize;
    let second_channel_safe = (second_channel.clamp(1, GLOBAL_CHANNELS as i32) - 1) as usize;

    let output = app.make_out_jack(0, range).await;
    let fader = app.use_faders();
    let buttons = app.use_buttons();
    let leds = app.use_leds();

    let quantizer = app.use_quantizer(range);
    let first_channel_muted_glob = app.make_global(storage.query(|s| s.first_channel_mute_saved));
    let second_channel_muted_glob = app.make_global(storage.query(|s| s.second_channel_mute_saved));
    let glob_latch_layer = app.make_global(LatchLayer::Main);

    // Set up initial state of LED button
    if first_channel_muted_glob.get() {
        leds.unset(0, Led::Button);
    } else {
        leds.set(0, Led::Button, led_color, Brightness::Mid);
    }

    let main_fut = async {
       
       loop {
            app.delay_millis(1).await;

            let first_channel_active = first_channel_enabled && !first_channel_muted_glob.get();
            let second_channel_active = first_channel_enabled && !second_channel_muted_glob.get();

            // Sample and sum output channels
            let mut out = 0;
            if first_channel_active && app.start_channel != first_channel_safe {
                let first_channel_out_jack_value = app.get_out_jack_value(first_channel_safe);
                out += first_channel_out_jack_value;
            }
            if second_channel_active && app.start_channel != second_channel_safe {
                let second_channel_out_jack_value = app.get_out_jack_value(second_channel_safe);
                out += second_channel_out_jack_value
            }

            // Hard clip summed values
            let out_safe = out.clamp(0, 4095);

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

            // info!("Summed out: {}, 1st channel enabled: {}[{}], 2nd channel enabled: {}[{}]", out_safe, first_channel_enabled, first_channel_safe, second_channel_enabled, second_channel_safe);

       }
    };

    let btn_fut = async {
        loop {
            buttons.wait_for_down(0).await;
            if !buttons.is_shift_pressed() {
                // First channel mute
                let muted = storage.modify_and_save(|s| {
                    s.first_channel_mute_saved = !s.first_channel_mute_saved;
                    s.first_channel_mute_saved
                });
                first_channel_muted_glob.set(muted);
                if muted {
                    leds.unset(0, Led::Button);
                } else {
                    leds.set(0, Led::Button, led_color, Brightness::Mid);
                }
            } else {
                // Second channel mute
                let muted = storage.modify_and_save(|s| {
                    s.second_channel_mute_saved = !s.second_channel_mute_saved;
                    s.second_channel_mute_saved
                });
                second_channel_muted_glob.set(muted);
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
                let muted = storage.query(|s| s.first_channel_mute_saved);
                if muted {
                    leds.unset(0, Led::Button);
                } else {
                    leds.set(0, Led::Button, led_color, Brightness::Mid);
                }
            } else {
                let muted = storage.query(|s| s.second_channel_mute_saved);
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