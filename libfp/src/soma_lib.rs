//!
//! Shared (host / target) library module for Soma sequencer app
//! 
//! This is a port of the DistingNT Soma Stochastic Exotic Scale Sequencer written by @thorinside
//! See: https://github.com/thorinside/soma/blob/main/soma.lua
//!
//! Ported by Richard Smith https://github.com/rjsmith
//!
//! Code in this library module can be unit tested on a host computer but deployed onto a Faderpunk runtime target.

use core::{panic};
use serde::{Deserialize, Serialize};

use crate::{Key, Note};

// Maximum sequencer length in # notes
pub const MAX_SEQUENCE_LENGTH: usize = 64;
const MAX_2_POW_12:u16 = 4095;
const HALF_MAX_2_POW_12:u16 = MAX_2_POW_12 / 2;

/// Sequencer note generator logic for the Soma sequencer app.
///
/// Soma generates patterns that mutate based on probability controls - like a Turing Machine.
/// The twist is it weights "spicy" notes higher - the ones that make each scale sound different from major (Ionian).
#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct SomaGenerator {
    // List of scale Notes in the generated Soma sequence, up to MAX_SEQUENCE_LENGTH in length.
    #[serde(with = "serde_arrays")] // using because sequence length > 32
    note_pattern: [Note; MAX_SEQUENCE_LENGTH],
    // Gate states (true/false) for each pattern step
    #[serde(with = "serde_arrays")] // using because sequence length > 32
    gate_pattern: [bool; MAX_SEQUENCE_LENGTH],
    // Computed fractional probabilities of each note in a scale, sum to MAX_2_POW_12 (2^12-1)
    scale_probabilities: [u16; 12], 
    // Generated sequence pattern length, in #notes
    pattern_length: usize,
    // Note weights for the current scale - higher weights for "spicy" notes
    scale_weights: [u8; 12],
    // Current step of sequenced pattern
    current_step: usize,
    // Current scale
    current_scale: Key,
}

// Public methods
impl SomaGenerator {
    /// Create a new SomaGenerator with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the current step index
    pub fn get_current_step(&self) -> usize {
        self.current_step
    }

    /// Moves next sequence step back to the start of the pattern
    pub fn reset_current_step(&mut self) {
            self.current_step = 0;
    }

    /// Returns the current scale
    pub fn get_current_scale(&self) -> Key {
        self.current_scale
    }

    /// Computes scale note probabilities based on the selected key and scale weights
    /// 
    /// The next call to generate_next_step() will use these probabilities to mutate notes in the sequence.
    /// Updates internal probabilities and weights tables.
    /// 
    /// ## Arguments
    /// * `scale` - the scale to compute probabilities for, comparing against the major scale.
    /// 
    /// NB: Also changes the internal current scale to this scale
    pub fn compute_scale_probabilities(&mut self, scale: Key) {
        self.current_scale = scale;

        // Clear weights table
        self.scale_weights = [1; 12];

        // Compare scale notes to major scale notes to assign weights and sum total weight
        let mut total_weight: u8 = 0;
        let scale_mask: u16 = scale.as_u16_key();
        let major_mask = Key::Ionian.as_u16_key();
        for n in 0..12 {
            let scale_pos_has_note = (scale_mask >> (11 - n)) & 1;
            let major_pos_has_note = (major_mask >> (11 - n)) & 1;
            if (major_pos_has_note == scale_pos_has_note) && (scale_pos_has_note == 1) {
                // Note is in both major and selected scale - normal weight
                self.scale_weights[n] = 1;
            } else if scale_pos_has_note == 0 {
                // Note is not in scope, never pick this one
                self.scale_weights[n] = 0;
            } else {
                // Note is "spicy" - in selected scale but not major - higher weight
                self.scale_weights[n] = 3;
            } 
            total_weight += self.scale_weights[n];
        }

        // Update probabilities array with new weights
        for n in 0..12 {
            self.scale_probabilities[n] = self.rescale_fractional_probability_to_0_4095(self.scale_weights[n] as f32 / total_weight as f32);
        }

    }



    /// Generates next scale Note in the sequence
    /// 
    /// Advances the internal current step counter, mutates the note and gate at that step
    /// ## Arguments
    /// * `flipGate` - whether to flip the gate at the current step
    /// * `flipNote` - whether to flip the note at the current step
    /// * `random_probability` - random number 0 to MAX_2_POW_12 used to pick new note based on scale probabilities
    /// 
    /// ## Returns
    /// Tuple of (Note, gate state) for the generated step
    pub fn generate_next_step(&mut self, flip_gate: bool, flip_note: bool, random_probability: u16) -> (Note, bool) {
        let safe_random_probability = random_probability.clamp(0, MAX_2_POW_12);

        // Advance to next step
        self.current_step = (self.current_step % self.pattern_length) + 1;

        // Mutate note at current step based on probability
        if flip_note {   
            self.note_pattern[self.current_step] = self.weighted_pick_note_from_current_scale(safe_random_probability); 
        }     

        // Mutate gate at current step based on probability
        if flip_gate {
            self.gate_pattern[self.current_step] = !self.gate_pattern[self.current_step];
        }

         // Return generated note and gate
        (self.note_pattern[self.current_step], self.gate_pattern[self.current_step])
    }    

    /**
     * Initializes and generates a new pattern of given length and scale
     * 
     * ## Arguments
     * * `length` - length of the pattern to generate, in #notes
     * * `scale` - scale to use for the pattern
     * * `note_probability` - array of random integers 0 to 4095 used to pick new notes for each step
     * * `gate_probability` - array of random integers 0 to 4095 used to pick new gates for each step
     */
    pub fn initialize_patterns(&mut self, length: usize, scale: Key, note_probability: [u16; MAX_SEQUENCE_LENGTH], gate_probability: [u16; MAX_SEQUENCE_LENGTH]) {
        self.current_scale = scale;
        self.pattern_length = length;   
        self.current_step = 0;
        self.compute_scale_probabilities(scale);
        // Clear unused pattern slots
        for n in length .. MAX_SEQUENCE_LENGTH {
            self.note_pattern[n] = Note::C;
            self.gate_pattern[n] = false;
        }
        // Generate starting pattern
        for n in 0 .. length {
            // TODO : Randomise!
            self.note_pattern[n] = self.weighted_pick_note_from_current_scale(note_probability[n]); 
            self.gate_pattern[n] = gate_probability[n].clamp(0, MAX_2_POW_12) > HALF_MAX_2_POW_12;   
        }
        
    }

}

// Private methods
impl SomaGenerator {
    /// Rescales a fractional probability (0.0 to 1.0) to a u16 value between 0 and 4095
    fn rescale_fractional_probability_to_0_4095(&self, prob: f32) -> u16 {
        ((prob * MAX_2_POW_12 as f32) as u16).clamp(0, MAX_2_POW_12)
    }

    ///
    /// Returns a Note picked from the current scale based on pre-computed weighted probabilities
    /// 
    /// ## Arguments
    /// * `random_probability` - random number 0 to MAX_2_POW_12 used to pick new note based on scale probabilities (e.g. from a random number generator)
    /// 
    /// ## Returns              
    ///* Note picked from the current scale
    ///
    fn weighted_pick_note_from_current_scale(&self, random_probability: u16) -> Note {
        let safe_random_probability = random_probability.clamp(0, MAX_2_POW_12);

        let scale_mask: u16 = self.current_scale.as_u16_key();

        let mut accum: u16 = 0;
        for n in self.scale_probabilities.iter().enumerate() {
            accum += n.1;
            if safe_random_probability <= accum { 
                // Found our note
                let note_index = n.0 as u8;
                // Map note index to actual Note in scale
                let mut scale_note_count = 0;
                for i in 0..12 {
                    let scale_pos_has_note = (scale_mask >> (11 - i)) & 1;
                    if scale_pos_has_note == 1 {
                        if scale_note_count == note_index {
                            return Note::from(i as u8);
                        } else {
                            scale_note_count += 1;
                        }
                    }
                }
            }
        }
    
        // Fallback, return highest note in scale
        for i in (0..12).rev() {
            let scale_pos_has_note = (scale_mask >> (11 - i)) & 1;
            if scale_pos_has_note == 1 {
                return Note::from(i as u8);
            }
        }

        // Fallback - should never reach here as long as scale has at least one note!
        panic!("weighted_pick_note_from_current_scale: failed to pick note from scale");
    }

  
}

impl Default for SomaGenerator {
    fn default() -> Self {
        Self {
            pattern_length: 8,
            note_pattern: [Note::C; MAX_SEQUENCE_LENGTH],
            gate_pattern: [false; MAX_SEQUENCE_LENGTH],
            scale_probabilities: [0; 12],
            scale_weights: [1; 12],
            current_step: 0,
            current_scale: Key::Ionian,
        }
    }
}



//
// ************** UNIT TESTS ****************************
//
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_soma_generator() {
        // SETUP
        let s = SomaGenerator::default();

        // EXECUTE

        // ASSERT
        assert_eq!(s.pattern_length, 8);
        assert_eq!(s.note_pattern, [Note::C; MAX_SEQUENCE_LENGTH]);
        assert_eq!(s.gate_pattern, [false; MAX_SEQUENCE_LENGTH]);
        assert_eq!(s.scale_probabilities, [0; 12]);
        assert_eq!(s.scale_weights, [1; 12]);
        assert_eq!(s.current_step, 0);
        assert_eq!(s.current_scale, Key::Ionian);
    }

    #[test]
    fn test_generate_first_step_no_flip_default() {
        // SETUP
        let mut s = SomaGenerator::default();

        // EXECUTE
        let (note, gate) = s.generate_next_step(false, false, 0);

        // ASSERT
        assert_eq!(note, Note::C);
        assert_eq!(gate, false);
        assert_eq!(s.current_step, 1);
    }

    #[test]
    fn test_generate_first_step_flip_default() {
        // SETUP
        let mut s = SomaGenerator::default();
        s.compute_scale_probabilities(Key::Ionian);

        // EXECUTE - will pick first note in scale, a C
        let (note, gate) = s.generate_next_step(true, true, 0);

        // ASSERT
        assert_eq!(note, Note::C); 
        assert_eq!(gate, true);
        assert_eq!(s.current_step, 1);

         // EXECUTE - will pick last note in scale, a B
        let (note2, _gate2) = s.generate_next_step(true, true, MAX_2_POW_12);

        // ASSERT
        assert_eq!(note2, Note::B); 
        assert_eq!(s.current_step, 2);

        // EXECUTE - will pick middle note in scale, a F
        let (note3, _gate3) = s.generate_next_step(true, true, HALF_MAX_2_POW_12);

        // ASSERT
        assert_eq!(note3, Note::A); 
        assert_eq!(s.current_step, 3);

    }

    #[test]
    fn test_compute_scale_probabilities_major_scale() {
        // SETUP
        let mut s = SomaGenerator::default();   

        // EXECUTE
        s.compute_scale_probabilities(Key::Ionian);

        // ASSERT
        let expected_probabilities: [u16; 12] = [
            s.rescale_fractional_probability_to_0_4095(1.0 / 7.0),
            0,
            s.rescale_fractional_probability_to_0_4095(1.0 / 7.0),
            0,
            s.rescale_fractional_probability_to_0_4095(1.0 / 7.0),
            s.rescale_fractional_probability_to_0_4095(1.0 / 7.0),
            0,
            s.rescale_fractional_probability_to_0_4095(1.0 / 7.0),
            0,
            s.rescale_fractional_probability_to_0_4095(1.0 / 7.0),
            0,
            s.rescale_fractional_probability_to_0_4095(1.0 / 7.0),
        ];
        assert_eq!(s.scale_probabilities, expected_probabilities);
        // Check probabilities sum to MAX_2_POW_12, with tolerance for floating point errors
        assert_eq!(s.scale_probabilities.iter().sum::<u16>(), MAX_2_POW_12);

    }

       #[test]
    fn test_compute_scale_probabilities_phyrgian_scale() {
        // SETUP
        let mut s = SomaGenerator::default();   

        // EXECUTE
        s.compute_scale_probabilities(Key::Phrygian);

        // ASSERT
        let expected_probabilities: [u16; 12] = [
            s.rescale_fractional_probability_to_0_4095(1.0 / 15.0),
            s.rescale_fractional_probability_to_0_4095(3.0 / 15.0), // Spicy!
            0,
            s.rescale_fractional_probability_to_0_4095(3.0 / 15.0), // Spicy!
            0,
            s.rescale_fractional_probability_to_0_4095(1.0 / 15.0),
            0,
            s.rescale_fractional_probability_to_0_4095(1.0 / 15.0),
            s.rescale_fractional_probability_to_0_4095(3.0 / 15.0), // Spicy!
            0,
            s.rescale_fractional_probability_to_0_4095(3.0 / 15.0), // Spicy!
            0,
        ];
        assert_eq!(s.scale_probabilities, expected_probabilities);
        assert_eq!(s.scale_probabilities.iter().sum::<u16>(), MAX_2_POW_12);
    }

    #[test]
    fn test_generate_spicy_step() {
        // SETUP
        let mut s = SomaGenerator::default();
        s.compute_scale_probabilities(Key::Phrygian);

        // EXECUTE - will pick first note in scale, a C
        let (note, _gate) = s.generate_next_step(false, true, s.rescale_fractional_probability_to_0_4095(4.0 / 15.0));

        // ASSERT
        assert_eq!(note, Note::CSharp); 
        assert_eq!(s.current_step, 1);

    }

    #[test]
    fn test_initialize_patterns() {
        // SETUP
        let mut s = SomaGenerator::default();
        let note_probability = [s.rescale_fractional_probability_to_0_4095(0.5); MAX_SEQUENCE_LENGTH];
        let gate_probability = [s.rescale_fractional_probability_to_0_4095(0.6); MAX_SEQUENCE_LENGTH];

        // EXECUTE
        s.initialize_patterns(MAX_SEQUENCE_LENGTH, Key::Phrygian, note_probability, gate_probability);

        // ASSERT
        assert_eq!(s.pattern_length, MAX_SEQUENCE_LENGTH);
        assert_eq!(s.note_pattern, [Note::GSharp; MAX_SEQUENCE_LENGTH]);
        assert_eq!(s.gate_pattern, [true; MAX_SEQUENCE_LENGTH]);


    }


}

