//! Soma app - Stochastic Exotic Scale Sequencer
//! 
//! This is a port of the DistingNT Soma Stochastic Exotic Scale Sequencer written by @thorinside.
//! Information from the original Soma repo README is reproduced below.
//! See: https://github.com/thorinside/soma/blob/main/soma.lua
//! 
//! Also based on the Faderpunk Turing app.
//!
//! Ported by Richard Smith https://github.com/rjsmith
//! 
//! # Description
//! A stochastic sequencer app for the ATOV Faderpunk that does the Turing Machine thing with exotic scales.
//!
//! Named after the mysterious drink from "Brave New World" - creates patterns that feel both random and intentional, somewhere between chaos and order.
//! 
//! Soma generates musical patterns that mutate based on probability controls - like a Turing Machine. The twist is it weights "spicy" notes higher - the ones that make each scale sound different from major / ionian.
//! 
//! ## Pattern Evolution 
//! - **Note Pattern**: Sequence that mutates or locks
//! - **Gate Pattern**: Gates that evolve independently  
//! - **Probability**: Controls mutation rate
//!   - **100%** = Constant change
//!   - **50%** = Gradual evolution
//!   - **0%** = Locked
//!
//! ## The Spicy Notes Thing
//! Compares each scale to major and gives 3x weight to notes NOT in the major scale. These characteristic notes define each scale's flavor:
//!
//! - **Phrygian**: ♭2, ♭6, ♭7 get emphasized
//! - **Lydian**: That #4 shows up more
//! - **Hungarian Minor**: ♭2, #4, ♭6 come through
//!
//! Patterns naturally emphasize what makes each scale unique.
//! 
//! ## Hardware Mapping
//! 
//! | Control | Function | + Shift | + Fn
//! |---------|----------|---------|------|
//! | Jack 1  | V/o Pitch CV out | N/A     | N/A  |
//! | Fader 1 | Note mutation % (0=locked, full=chaos) | Octave spread % (0=none, full=3 octaves) | N/A|
//! | LED 1 Top | V/o output level | Octave change chance in red | N/A
//! | LED 1 Bottom | Flash at tempo | N/A | N/A
//! | Fn 1    | Mute both outputs | Press button x times sets length (max 64 steps) | N/A |
//! | Jack 2  | Gate output | N/A     | N/A  |
//! | Fader 2 | Gate mutation % (0=locked, full=chaos) | Speed (clock divide)  | N/A |
//! | LED 2 Top | Gate output indicator | N/A | N/A 
//! | LED 2 Bottom | N/A | N/A | N/A
//! | Fn 2    | Only send note or change CV on gate | N/A | N/A |
//! 
//! ## Usage Tips
//!
//! ### Finding Sweet Spots
//! Start with high probability (70-90%) to generate interesting patterns, then gradually reduce to lock in patterns you like.
//!
//! ### Scale Morphing
//! - Lock a pattern at 0%
//! - Switch to a different scale in the Configurator 
//! - Stop then restart the Faderpunk clock (Soma only checks for global scale updates when the clock is stopped)
//! - Slowly increase probability to hear it morph
//! - Lock it again when it sounds good
//!
//! ### Rhythmic Combos
//! - Low note %, high gate % = stable melody, evolving rhythm
//! - High note %, low gate % = evolving melody, stable rhythm
//!
//! ### Octave Dynamics
//! Small amounts (10-30%) add variation without losing the melodic line.
//! 
//! ### Scenes
//! Store different app settings in different Scenes, then performatively switch between them
//!
//! ## Experiments to Try
//!
//! ### The Conversation
//! Run two Somas at different clock divisions (one at 1/4, one at 1/3). Set them to complementary scales (like Dorian and Lydian). Use low probability (~15%) so they slowly diverge from similar starting points.
//!
//! ### Ghost in the Machine  
//! Set note probability to 1-2% - just enough that you occasionally hear a "mistake" that becomes part of the pattern. Like a musician occasionally hitting a wrong note that sounds right.
//!
//! ### The Degrading Loop
//! Start with a locked pattern you like. Add just 3-5% probability and let it run for 10 minutes. It's like a tape loop slowly deteriorating, occasionally glitching into something new.
//!
//! ### Call and Response
//! Use Faderpunk global Reset input rhythmically (not just at pattern start). Feed it a euclidean rhythm. The pattern keeps getting pulled back to step 1, creating phrases that mutate but keep returning home.
//!
//! ### Scale Automation
//! Keep probability at ~40% but sequence through scales via encoder. Each scale change is like a harmonic filter being swept - the pattern reshapes itself to the new harmonic space.
//!
//! ### The Octave Scatter
//! Gate probability at 0%, note probability at 0%, but octave at 100%. Same notes, same rhythm, but huge registral leaps. Run through a resonant filter that tracks pitch for wild timbral changes.
//!
//! ### Binary Beats
//! Set gate probability high (80%) but note probability at 0%. You get evolving rhythms with a repeating melodic motif - techno generators.
//!
//! ## Technical Bits
//!
//! The weighted probability thing ensures each scale sounds like itself. The Turing Machine behavior creates organic evolution. It's that balance between random and intentional that makes it musical.
//!
use embassy_futures::{
    join::join5, select::{select, select3, Either}
};
use embassy_sync::{blocking_mutex::raw::NoopRawMutex, signal::Signal};
use heapless::Vec;
use libfp::{
    APP_MAX_PARAMS, AppIcon, Brightness, ClockDivision, Color, Config, MidiCc, MidiChannel, MidiMode, MidiNote, MidiOut, Param, Range, Value, ext::FromValue, latch::LatchLayer, quantizer::Pitch, soma_lib::{MAX_SEQUENCE_LENGTH, SomaGenerator}
};
use serde::{Deserialize, Serialize};

use crate::app::{App, AppParams, AppStorage, ClockEvent, Led, ManagedStorage, ParamStore, SceneEvent};

// TODO: Remove from final code
use defmt::info;

pub const CHANNELS: usize = 2;
pub const PARAMS: usize = 8;

// Clock division resolution, 24 = quarter notes at 24 PPQN
const CLOCK_RESOLUTION: [u16; 8] = [24 /* quarter note */, 16 /* dotted eighth */, 12 /* eighth */, 8, 6 /* sixteenth note */, 4, 3, 2];

// Maximum octave range for randomization
const MAX_OCTAVE_RANGE: f32 = 3.;

// App configuration visible to the configurator
pub static CONFIG: Config<PARAMS> = Config::new(
    "Soma",
    "Stochastic exotic scale sequencer, synced to internal clock",
    Color::Blue,
    AppIcon::Random,
)
.add_param(Param::MidiMode)
.add_param(Param::MidiChannel {
    name: "MIDI channel",
})
.add_param(Param::MidiCc { name: "CC number" })
.add_param(Param::MidiNote { name: "Base Note" })
.add_param(Param::i32 {
    name: "GATE %",
    min: 1,
    max: 100,
})
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
})
.add_param(Param::MidiOut);
pub struct Params {
    midi_mode: MidiMode,
    midi_channel: MidiChannel,
    midi_cc: MidiCc,
    midi_note: MidiNote,
    gatel: i32,
    color: Color,
    range: Range,
    midi_out: MidiOut,
}

impl Default for Params {
    fn default() -> Self {
        Self {
            midi_mode: MidiMode::default(),
            midi_channel: MidiChannel::default(),
            midi_cc: MidiCc::from(40),
            midi_note: MidiNote::from(36),
            gatel: 50,
            color: Color::Blue,
            range: Range::_0_5V,
            midi_out: MidiOut::default(),
        }
    }
}

impl AppParams for Params {
    fn from_values(values: &[Value]) -> Option<Self> {
        if values.len() < PARAMS {
            return None;
        }
        Some(Self {
            midi_mode: MidiMode::from_value(values[0]),
            midi_channel: MidiChannel::from_value(values[1]),
            midi_cc: MidiCc::from_value(values[2]),
            midi_note: MidiNote::from_value(values[3]),
            gatel: i32::from_value(values[4]),
            color: Color::from_value(values[5]),
            range: Range::from_value(values[6]),
            midi_out: MidiOut::from_value(values[7]),
        })
    }

    fn to_values(&self) -> Vec<Value, APP_MAX_PARAMS> {
        let mut vec = Vec::new();
        vec.push(self.midi_mode.into()).unwrap();
        vec.push(self.midi_channel.into()).unwrap();
        vec.push(self.midi_cc.into()).unwrap();
        vec.push(self.midi_note.into()).unwrap();
        vec.push(self.gatel.into()).unwrap();
        vec.push(self.color.into()).unwrap();
        vec.push(self.range.into()).unwrap();
        vec.push(self.midi_out.into()).unwrap();
        vec
    }
}

// Set up app scene storage - persistent data
#[derive(Serialize, Deserialize)]
pub struct Storage {
    note_flip_prob_saved: u16,
    gate_flip_prob_saved: u16,
    octave_spread_prob_saved: u16,
    clock_resolution_saved: u16,
    length_saved: u16,
    key_saved: u8,
    muted_saved: bool,
    note_only_on_gate_saved: bool,
}

impl Default for Storage {
    fn default() -> Self {
        Self {
            note_flip_prob_saved: 0,
            gate_flip_prob_saved: 0,
            octave_spread_prob_saved: 0,
            clock_resolution_saved: 2048,
            length_saved: 8,
            key_saved: 0, // Chromatic
            muted_saved: false,
            note_only_on_gate_saved: false,
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
    info!("Soma started!");
    
    // Get app parameters
    let (midi_out, midi_mode, midi_cc, led_color, midi_chan, base_note, gatel, range) = params
    .query(|p| {
        (
            p.midi_out,
            p.midi_mode,
            p.midi_cc,
            p.color,
            p.midi_channel,
            p.midi_note,
            p.gatel,
            p.range,
        )
    });

    let buttons = app.use_buttons();
    let faders = app.use_faders();
    let leds = app.use_leds();
    let mut clock = app.use_clock();
    let die = app.use_die();
    let midi = app.use_midi_output(midi_out, midi_chan);
    let div_glob = app.make_global(CLOCK_RESOLUTION[0]);
    let midi_note_glob = app.make_global(MidiNote::default());
    let midi_note_on_glob = app.make_global(false); // Tracks if midi note is sounding (true) or not
    let glob_latch_layer = app.make_global(LatchLayer::Main);
    let length_glob = app.make_global(8);
    let muted_glob = app.make_global(storage.query(|s| s.muted_saved));
    let note_only_on_gate_glob = app.make_global(storage.query(|s| s.note_only_on_gate_saved));

    let quantizer = app.use_quantizer(range);

    // Get global config quantized key
    let (global_key, _) = quantizer.get_scale().await;

    if muted_glob.get() {
        leds.unset(0, Led::Button);
        leds.unset(1, Led::Button);
    } else {
        leds.set(0, Led::Button, led_color, Brightness::Mid);
        if note_only_on_gate_glob.get() {
            leds.set(1, Led::Button, led_color, Brightness::Mid);
        } else {
            leds.set(1, Led::Button, led_color, Brightness::Low);
        }
    }

    let pitch_output = app.make_out_jack(0, range).await;
    let gate_output = app.make_gate_jack(1, 4095).await;

    let (clock_res, length) =
        storage.query(|s| (s.clock_resolution_saved, s.length_saved));

    div_glob.set(CLOCK_RESOLUTION[clock_res as usize / 512]);

    // Set up Soma Generator
    let mut soma = SomaGenerator::default();
    let mut note_probabilities = [0; MAX_SEQUENCE_LENGTH];
    let mut gate_probabilities = [0; MAX_SEQUENCE_LENGTH];
    for n in 0..length as usize {
        note_probabilities[n] = die.roll();
        gate_probabilities[n] = die.roll();
    } 
    soma.initialize_patterns(8, global_key, note_probabilities, gate_probabilities);
   
    // Main sequencer task, clocked by internal clock
    let clock_fut = async {
        // Clock step counter
        let mut clkn: u16 = 0;
        loop {
            let div = div_glob.get();
            let length = length_glob.get();

            match clock.wait_for_event(ClockDivision::_1).await {
                ClockEvent::Reset => {
                    info!("Soma: Reset received!");
                    clkn = 0;
                    if midi_mode == MidiMode::Note && midi_note_on_glob.get() {
                        midi.send_note_off(midi_note_glob.get()).await;
                        midi_note_on_glob.set(false);
                    }
                    soma.reset_current_step();
                }

                ClockEvent::Tick => {
                   
                    // If on the right division, step the note and gate sequencers
                    if clkn.is_multiple_of(div) {

                        // Compute step mutation based on fader probabilities
                        let note_change_prob = storage.query(|s| s.note_flip_prob_saved);
                        let gate_change_prob = storage.query(|s| s.gate_flip_prob_saved); 
                        let flip_note = die.roll() < note_change_prob;
                        let flip_gate = die.roll() < gate_change_prob;
                        let note_choice_probability = die.roll(); // Random number between 0 and 4095;

                        // Step the soma generator
                        let (note, gate) = soma.generate_next_step(
                            flip_gate,
                            flip_note,
                            note_choice_probability,
                        );

                        // Apply octave variation (add between 0 and 3 octaves)
                        // `octave_spread range` is 2^12 = 4095 max, so spread of octave 0 (0) - 4095 (+3 octaves)
                        let octave_spread_range  = ((storage.query(|s| s.octave_spread_prob_saved) as f32 / 4095.0) * MAX_OCTAVE_RANGE) as u16;
                        let mut octave_offset = 0;
                        if octave_spread_range > 0 {
                            let octave_chance = die.roll(); // 0 - 4095 random number
                            // NB: The "+ 0.5" here is to ensure proper rounding when converting to i8
                            octave_offset = ((((octave_chance as f32 / 4095.0) * octave_spread_range as f32) + 0.5) as i8).clamp(0, MAX_OCTAVE_RANGE as i8);
                        }

                        // Calculate output pitch
                        let out_pitch = Pitch {
                            octave: octave_offset,
                            note,
                        };

                        // Set output CV, gate and MIDI Note/CC
                        let muted = muted_glob.get();
                        let note_only_on_gate = note_only_on_gate_glob.get();

                        let out_pitch_in_0_10v = if muted {
                            0
                        } else if !note_only_on_gate || gate {
                            out_pitch.as_counts(Range::_0_10V)
                        } else {
                            0
                        };

                        pitch_output.set_value(out_pitch_in_0_10v);
                        leds.set(
                            0,
                            Led::Top,
                            led_color,
                            Brightness::Custom((out_pitch_in_0_10v / 160) as u8), // TODO: Verify scaling
                        );

                        if !muted {
                            // Only send Midi if either not in midi-on-trigger mode, or midi-on-trigger and gate
                            if !note_only_on_gate || gate {
                                match midi_mode {
                                    MidiMode::Note => {
                                        let note = midi_note_glob.set(out_pitch.as_midi() + base_note);
                                        midi_note_on_glob.set(true);
                                        midi.send_note_on(note, 4095).await;
                                    }
                                    MidiMode::Cc => {
                                        midi.send_cc(midi_cc, out_pitch.as_counts(Range::_0_10V)).await;
                                    }
                                }
                            }
                          
                            // Output gate CV and set LED
                            if gate {
                                gate_output.set_high().await;
                                leds.set(1, Led::Top, led_color, Brightness::High);
                            } else {
                                gate_output.set_low().await; 
                                leds.unset(1, Led::Top);
                            }

                            info!("Generated note at pattern step: {}, note flip: {}, note: {:?}, gate flip: {}, gate: {}, note prob: {}, out pitch 0-10V: {}", (clkn / div) % length, flip_note as u8, note as u8, flip_gate as u8, gate, note_choice_probability, out_pitch_in_0_10v);

                        } else {
                            // Muted, so gate  off
                            leds.unset(1, Led::Top);
                            gate_output.set_low().await;

                            if midi_mode == MidiMode::Note && midi_note_on_glob.get() {
                                midi.send_note_off(midi_note_glob.get()).await;
                                midi_note_on_glob.set(false);
                            }
                            
                            info!("Muted note at pattern step: {}", (clkn / div) % length);

                        }

                        // If just processed last step of `length` pattern
                        if (clkn / div).is_multiple_of(length) {
                            // Check for global scale change
                            let (global_key, _) = quantizer.get_scale().await;
                            
                            if global_key as u8 != storage.query(|s| s.key_saved) {
                                info!("Global key changed to {:?}, updating soma generator", global_key as u8);
                                storage.modify_and_save(|s| s.key_saved = global_key as u8);
                                soma.compute_scale_probabilities(global_key);
                            }
                        }
                            

                    }

                    // Wait for the gate time to elapse, then terminate the playing note, if any
                    if clkn % div == (div * gatel as u16 / 100).clamp(1, div - 1) {
                        // Gate  off
                        leds.unset(1, Led::Top);
                        gate_output.set_low().await;

                        if midi_mode == MidiMode::Note && midi_note_on_glob.get() {
                            midi.send_note_off(midi_note_glob.get()).await;
                            midi_note_on_glob.set(false);
                        }

                    }
                   
                    clkn += 1;
                }
                ClockEvent::Stop => {
                    info!("Clock stopped");
                    // Gate  off
                    leds.unset(1, Led::Top);
                    gate_output.set_low().await;

                    if midi_mode == MidiMode::Note && midi_note_on_glob.get() {
                        midi.send_note_off(midi_note_glob.get()).await;
                        midi_note_on_glob.set(false);
                    }
                }
                // Ignore other MIDI Clock events
                _ => {}
            }
        }
    };

    let faders_fut = async {
        let mut latch = [
            app.make_latch(faders.get_value_at(0)),
            app.make_latch(faders.get_value_at(1)),
        ];

        loop {
            let chan = faders.wait_for_any_change().await;
            let latch_layer = glob_latch_layer.get();
            if chan == 0 {
                let target_value = match latch_layer {
                    LatchLayer::Main => storage.query(|s| s.note_flip_prob_saved),
                    LatchLayer::Alt => storage.query(|s| s.octave_spread_prob_saved),
                    LatchLayer::Third => 0,
                };
                if let Some(new_value) =
                    latch[chan].update(faders.get_value_at(chan), latch_layer, target_value)
                {
                    match latch_layer {
                        LatchLayer::Main => {
                            // Note change probability changed
                            storage.modify_and_save(|s| { s.note_flip_prob_saved = new_value});   
                        }
                        LatchLayer::Alt => {
                            // Octave spread probability changed
                            info!("Octave spread prob changed to {}", new_value);
                            storage.modify_and_save(|s| { s.octave_spread_prob_saved = new_value});   
                        }   
                        _ => {}
                    }
                }       
                
            } else if chan == 1 {
                // Channel 2 fader changes
               let target_value = match latch_layer {
                    LatchLayer::Main => storage.query(|s| s.gate_flip_prob_saved),
                    LatchLayer::Alt => storage.query(|s| s.clock_resolution_saved),
                    LatchLayer::Third => 0,
                };
                if let Some(new_value) =
                    latch[chan].update(faders.get_value_at(chan), latch_layer, target_value)
                {
                    match latch_layer {
                        LatchLayer::Main => {
                            // Gate change probability changed
                            storage.modify_and_save(|s| { s.gate_flip_prob_saved = new_value});   
                        }
                        LatchLayer::Alt => {
                            // The higher the clock resolution fader, the faster the sequencer will step though the pattern
                            div_glob.set(CLOCK_RESOLUTION[new_value as usize / 512]);
                            storage.modify_and_save(|s| s.clock_resolution_saved = new_value);
                            // We are changing clock division here, so switch off midi note
                            if midi_mode == MidiMode::Note && midi_note_on_glob.get() {
                                midi.send_note_off(midi_note_glob.get()).await;
                                midi_note_on_glob.set(false);
                            }
                        }
                        _ => {}
                    }
                }                      
            }
        }
    };

    // Set up pattern length recording
    let length_rec_flag = app.make_global(false);
    let length_rec = app.make_global(0);

    let button_fut = async {
        loop {
            match select(buttons.wait_for_down(0), buttons.wait_for_down(1)).await {
                Either::First(_) => {
                    let mut length = length_rec.get();
                    if buttons.is_shift_pressed() && length_rec_flag.get() {
                        // Increment temporary recorded pattern length whilst shift button held down
                        length += 1;
                        length_rec.set(length.min(MAX_SEQUENCE_LENGTH as u16));
                    } else if !buttons.is_shift_pressed() {
                        // Toggle app muted state
                        let muted = storage.modify_and_save(|s| {
                            s.muted_saved = !s.muted_saved;
                            s.muted_saved
                        });
                        muted_glob.set(muted);

                        if muted {
                            leds.unset(0, Led::Button);
                            leds.unset(1, Led::Button);
                        } else {
                            leds.set(0, Led::Button, led_color, Brightness::Mid);
                            if note_only_on_gate_glob.get() {
                                leds.set(1, Led::Button, led_color, Brightness::Mid)
                            } else {
                                leds.set(1, Led::Button, led_color, Brightness::Low)
                            }
                        }
                    }
                }
                Either::Second(_) => {
                    let note_only_on_gate = storage.modify_and_save(|s| {
                        s.note_only_on_gate_saved = !s.note_only_on_gate_saved;
                        s.note_only_on_gate_saved
                    });
                    note_only_on_gate_glob.set(note_only_on_gate);
                    if note_only_on_gate {
                        leds.set(1, Led::Button, led_color, Brightness::Mid);
                    } else {
                        leds.set(1, Led::Button, led_color, Brightness::Low);
                    }
                }
            };
        }
    };

    let shift_fut = async {
        let mut shift_old = false;

        loop {
            app.delay_millis(1).await;

            glob_latch_layer.set(LatchLayer::from(buttons.is_shift_pressed()));

            // Handle pattern length recording - start and stop recording
            if buttons.is_shift_pressed() && !shift_old {
                shift_old = true;
                length_rec_flag.set(true);
                length_rec.set(0);
            }
            if !buttons.is_shift_pressed() && shift_old {
                shift_old = false;
                length_rec_flag.set(false);
                let length = length_rec.get();
                if length > 1 {
                    length_glob.set(length);
                    storage.modify_and_save(|s| s.length_saved = length);
                }
            }
        }
    };

    let scene_fut = async {
        loop {
            match app.wait_for_scene_event().await {
                SceneEvent::LoadScene(scene) => {
                    storage.load_from_scene(scene).await;
                    let muted = storage.query(|s| s.muted_saved);
                    muted_glob.set(muted);
                    let note_only_on_gate = storage.query(|s| s.note_only_on_gate_saved);
                    note_only_on_gate_glob.set(note_only_on_gate);
                    if muted {
                        leds.unset(0, Led::Button);
                        leds.unset(1, Led::Button);
                    } else {
                        leds.set(0, Led::Button, led_color, Brightness::Mid);
                        if note_only_on_gate {
                            leds.set(1, Led::Button, led_color, Brightness::Mid);
                        } else {
                            leds.set(1, Led::Button, led_color, Brightness::Low);
                        }
                    }
                }

                SceneEvent::SaveScene(scene) => {
                    storage.save_to_scene(scene).await;
                }
            }
        }
    };

    join5(clock_fut, faders_fut, button_fut, shift_fut, scene_fut).await;

}