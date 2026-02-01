// Copyright (C) 2012 Emilie Gillet.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.
//
// Based on the original Grids by Emilie Gillet.

/*
* Terminology:
*
* "Part":        "Voices" of generated sequence (e.g. BD/SD/HH for Drum mode)
* "Beat":        (e.g., quarter notes based on sequence_step).
* "Pulses":      aka. 24 ppqn clock ticks
* "Step":        Sequence step 0 - 31
*/
use resources::K_NUM_PARTS;

const K_PULSES_PER_STEP:u8 = 3; // 24 ppqn ; 8 steps per quarter note
const K_PULSE_DURATION:u8 = 8; // 8 ticks of the main 24 ppqn clock

#[derive(Debug, Clone, Copy)]
struct DrumsSettings {
    x: u8,
    y: u8,
    randomness: u8, // Chaos amount for drum patterns
}

#[derive(Debug, Clone, Copy)]
struct EuclideanSettings {
    // Length for each part is managed by PatternGenerator::current_euclidean_length_ directly,
    // not stored in this settings struct, to simplify updates from parameters.
    chaos_amount: u8,
}

#[derive(Debug, Clone, Copy)]
enum PatternModeSettings {
    Drums(DrumsSettings),
    Euclidean(EuclideanSettings)
}

#[derive(Debug, Clone, Copy)]
struct PatternGeneratorSettings {
    options: PatternModeSettings,
    density: [u8; K_NUM_PARTS],
}

#[derive(Debug, Clone, Copy)]
enum OutputMode {
    OutputModeEuclidean,
    OutputModeDrums,
}

// Core Grids pattern generator state and logic
#[derive(Debug, Clone, Copy)]
struct PatternGenerator {

    // Internal state variables
    output_buffer: [u8; K_STEPS_PER_PATTERN >> 3],          // Stores the full pattern bitmask (not directly used for real-time state_)
    pulse_counter: [u8; K_NUM_PARTS],                       // Tracks individual part pulse counts (if ever needed)
    pulse_duration: [u8; K_NUM_PARTS],                      // Individual pulse durations (if ever needed, currently global via state_)
    current_euclidean_length: [u8; K_NUM_PARTS],            // Active length for each Euclidean part
    fill: [u8; K_NUM_PARTS],                                // Calculated number of active steps for Euclidean parts, based on density
    step_counter: [u8; K_NUM_PARTS],                        // Generic step counter per part, used for Euclidean perturbation in original code
    part_perturbation: [u8; K_NUM_PARTS],                   // Randomness value applied per part in Drum mode
    euclidean_step: [u8; K_NUM_PARTS],                      // Current step for each Euclidean generator (0 to length-1)

    state: u8,                                              // Holds the current trigger/accent state for the current tick for all parts + accent.
    step: u8,                                               // Current step in the main 32-step sequence (0-31), synonymous with sequence_step_

    // Clock and timing related
    internal_clock_ticks: u16,               // Counts sub-ticks for original Grids clocking mode.
    beat_counter: u8,                        // Counts beats (e.g., quarter notes based on sequence_step_).
    sequence_step: u8,                       // Current step in the sequence (0-31), drives pattern evaluation.
    swing_applied: bool,                     // Tracks if swing has been applied in the current sub-step (more relevant to original complex swing).
    first_beat: bool,                        // True if current step is the first beat of the pattern.
    beat: bool,                              // True if current step is on a beat (typically quarter note).

    // State variables
    pulse: u8,
    pulse_duration_counter: u16,

}

// Public API
impl PatternGenerator {

    // Initializes with default settings
    pub fn init() {}


}

// Private / internal functions
impl PatternGenerator {

}






