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
//! | Fader 1 | Note mutation % (0=locked, 100=chaos) | Octave spread % (0=none, 100=3 octaves) | Speed (clock divide) |
//! | LED 1 Top | V/o output level | Octave change chance in red | N/A
//! | LED 1 Bottom | Flash at tempo | N/A | N/A
//! | Fn 1    | Mute both outputs | Press button x times sets length (max 64 steps) | N/A |
//! | Jack 2  | Gate output | N/A     | N/A  |
//! | Fader 2 | Gate mutation % (0=locked, 100=chaos) | N/A | N/A |
//! | LED 2 Top | Gate output indicator | N/A | N/A 
//! | LED 2 Bottom | N/A | N/A | N/A
//! | Fn 2    | N/A | N/A | N/A |
//! 
//! ## Usage Tips
//!
//! ### Finding Sweet Spots
//! Start with high probability (70-90%) to generate interesting patterns, then gradually reduce to lock in patterns you like.
//!
//! ### Scale Morphing
//! - Lock a pattern at 0%
//! - Switch to a different scale  
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
    join::join5, select::{select, select3}
};
use embassy_sync::{blocking_mutex::raw::NoopRawMutex, signal::Signal};
use heapless::Vec;
use libfp::{
    ext::FromValue, 
    soma_lib::MAX_SEQUENCE_LENGTH, soma_lib::SomaGenerator,
    quantizer::Pitch,
    latch::LatchLayer, AppIcon, Brightness, ClockDivision, Color, Config, Curve,
    MidiCc, MidiChannel, MidiMode, MidiNote, MidiOut, Param, Range, Value, APP_MAX_PARAMS,
};
use serde::{Deserialize, Serialize};

use crate::app::{App, AppParams, AppStorage, ClockEvent, Led, ManagedStorage, ParamStore, SceneEvent};

// TODO: Remove from final code
use defmt::info;

pub const CHANNELS: usize = 2;
pub const PARAMS: usize = 8;

// Clock division resolution, 12 = quarter notes at 24 PPQN
const CLOCK_RESOLUTION: [i32; 8] = [24, 16, 12, 8, 6, 4, 3, 2];
const DEFAULT_CLOCK_DIVIDER: u16 = 12; // Quarter notes

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
    midi_out: MidiOut,
    midi_note: MidiNote,
    gatel: i32,
    color: Color,
    range: Range,
}

impl Default for Params {
    fn default() -> Self {
        Self {
            midi_mode: MidiMode::default(),
            midi_channel: MidiChannel::default(),
            midi_cc: MidiCc::from(40),
            midi_note: MidiNote::from(36),
            midi_out: MidiOut::default(),
            gatel: 50,
            color: Color::Blue,
            range: Range::_0_5V,
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
    octave_prob_saved: u16,
    length_saved: u16,
}

impl Default for Storage {
    fn default() -> Self {
        Self {
            note_flip_prob_saved: 0,
            gate_flip_prob_saved: 0,
            octave_prob_saved: 0,
            length_saved: 8,
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
    let octave_spread_range_glob = app.make_global(0);
    let recall_flag = app.make_global(false); // What is this for?
    let div_glob = app.make_global(DEFAULT_CLOCK_DIVIDER);
    let midi_note = app.make_global(MidiNote::default());
    let glob_latch_layer = app.make_global(LatchLayer::Main);
    let length_glob = app.make_global(8);

    // Switch off LEDs on both channels initially
    leds.set(0, Led::Button, led_color, Brightness::Off);
    leds.set(1, Led::Button, led_color, Brightness::Off);

    let pitch_output = app.make_out_jack(0, range).await;
    let gate_output = app.make_gate_jack(1, 4095).await;

    //  let (octave_prob, length) =
    //     storage.query(|s| (s.octave_saved, s.length_saved));

    let length =8;
    // octave_prob_glob.set(octave_prob);
    // length_glob.set(length);

    // Set up Soma Generator
    let mut soma = SomaGenerator::default();
    let mut note_probabilities = [0; libfp::soma_lib::MAX_SEQUENCE_LENGTH];
    let mut gate_probabilities = [0; libfp::soma_lib::MAX_SEQUENCE_LENGTH];
    for n in 0..length {
        note_probabilities[n] = die.roll();
        gate_probabilities[n] = die.roll();
    } 
    soma.initialize_patterns(8, libfp::Key::Phrygian, note_probabilities, gate_probabilities);

    // Main sequencer task, clocked by internal clock
    let clock_loop = async {
        // Clock step counter
        let mut clkn = 0;
        loop {
            let div = div_glob.get();
            let length = length_glob.get();

            match clock.wait_for_event(ClockDivision::_1).await {
                ClockEvent::Reset => {
                    clkn = 0;
                    if midi_mode == MidiMode::Note {
                        midi.send_note_off(midi_note.get()).await;
                    }
                    soma.reset_current_step();
                }

                ClockEvent::Tick => {
                   
                    // If on the right division, step the note and gate sequencers
                    if clkn % div == 0 {

                        let note_change_prob = storage.query(|s| s.note_flip_prob_saved);
                        let gate_change_prob = storage.query(|s| s.gate_flip_prob_saved); 

                        let flip_note = die.roll() < note_change_prob;
                        let flip_gate = die.roll() < gate_change_prob;
                        let note_choice_probability = die.roll().clamp(0, 4095);
                        // Step the soma generator
                        let (note, gate) = soma.generate_next_step(
                            flip_gate,
                            flip_note,
                            note_choice_probability,
                        );

                        // Apply octave variation (add between 0 and 3 octaves)
                        // `octave_spread range` is 2^12 = 4095 max, so spread of octave 0 (0) - 4095 (+3 octaves)
                        let octave_spread_range  = ((octave_spread_range_glob.get() / 4095) as f32 * MAX_OCTAVE_RANGE) as u16;
                        let mut octave_offset = 0;
                        if octave_spread_range > 0 {
                            let octave_chance = die.roll();
                            octave_offset = ((octave_chance / 4095) as f32 * octave_spread_range as f32) as i8;
                        }

                        // Calculate output pitch
                        let out_pitch = Pitch {
                            octave: octave_offset,
                            note: note.into(),
                        };

                        // Set output CV, gate and MIDI Note/CC
                        let out_pitch_in_0_10v = out_pitch.as_counts(Range::_0_10V);
                        pitch_output.set_value(out_pitch_in_0_10v);
                        leds.set(
                            0,
                            Led::Top,
                            led_color,
                            Brightness::Custom((out_pitch_in_0_10v / 160) as u8), // TODO: Verify scaling
                        );

                        match midi_mode {
                            MidiMode::Note => {
                                let note = midi_note.set(out_pitch.as_midi() + base_note);
                                midi.send_note_on(note, 4095).await;
                            }
                            MidiMode::Cc => {
                                midi.send_cc(midi_cc, out_pitch.as_counts(Range::_0_10V)).await;
                            }
                        }

                        // Output gate CV and set LED
                        if gate {
                            gate_output.set_high().await;
                            leds.set(1, Led::Top, led_color, Brightness::High);
                        } else {
                            gate_output.set_low().await; 
                            // This shoul dbe redundant becauise of the gate length time handling code below
                            leds.unset(1, Led::Top);
                        }

                        info!("Generated note at pattern step: {}, note flip: {}, note: {:?}, gate flip: {}, gate: {}, note prob: {}, out pitch 0-10V: {}", (clkn / div) % length, flip_note as u8, note as u8, flip_gate as u8, gate, note_choice_probability, out_pitch_in_0_10v);

                   }

                   // Wait for the gate time to elapse, then terminate the playing note, if any
                   if clkn % div == (div * gatel as u16 / 100).clamp(1, div - 1) {
                        // Gate  off
                        leds.unset(1, Led::Top);
                        gate_output.set_low().await;

                        if midi_mode == MidiMode::Note {
                            midi.send_note_off(midi_note.get()).await;
                        }

                    }
                   
                    clkn += 1;
                }
                ClockEvent::Stop => {
                    info!("Clock stopped");
                    if midi_mode == MidiMode::Note {
                        midi.send_note_off(midi_note.get()).await;
                    }
                }
                // Ignore other MIDI Clock events
                _ => {}
            }
        }
    };

    let faders_loop = async {
        let mut latch = [
            app.make_latch(faders.get_value_at(0)),
            app.make_latch(faders.get_value_at(1)),
        ];

        loop {
            let chan = faders.wait_for_any_change().await;
            let latch_layer = glob_latch_layer.get();
            info!("Fader {} changed on latch layer {} to value {}", chan, latch_layer as u8, faders.get_value_at(chan));
            if chan == 0 {
                let target_value = match latch_layer {
                    LatchLayer::Main => storage.query(|s| s.note_flip_prob_saved),
                    LatchLayer::Alt => 0,
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
                        _ => {}
                    }
                }       
                
            } else if chan == 1 {
                // Channel 2 fader changes
               let target_value = match latch_layer {
                    LatchLayer::Main => storage.query(|s| s.gate_flip_prob_saved),
                    LatchLayer::Alt => 0,
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
                        _ => {}
                    }
                }                      
            }
        }
    };

    // Set up pattern length recording
    let length_rec_flag = app.make_global(false);
    let length_rec = app.make_global(0);

    let fut3 = async {
        loop {
            // Increment temporary recorded pattern length whilst shift button held down
            let shift = buttons.wait_for_down(0).await;
            let mut length = length_rec.get();
            if shift && length_rec_flag.get() {
                length += 1;
                length_rec.set(length.min(libfp::soma_lib::MAX_SEQUENCE_LENGTH as u16));
            }
        }
    };

    let fut4 = async {
        let mut shift_old = false;

        loop {
            app.delay_millis(1).await;

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

    let scene_handler = async {
        loop {
            match app.wait_for_scene_event().await {
                SceneEvent::LoadSscene(scene) => {
                    storage.load_from_scene(scene).await;
                    // TODO: Work out what to do - e.g. update xxx_glob variables
                }

                SceneEvent::SaveScene(scene) => {
                    storage.save_to_scene(scene).await;
                }
            }
        }
    };
    join5(clock_loop, faders_loop, fut3, fut4, scene_handler).await;

}