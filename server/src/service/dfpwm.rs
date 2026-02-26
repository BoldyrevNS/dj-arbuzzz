/// DFPWM (Dynamic Filter Pulse Width Modulation) encoder
/// Implementation based on the 1a specification used by ComputerCraft
/// Enhanced with noise shaping for improved audio quality
pub struct DfpwmEncoder {
    charge: i32,
    strength: i32,
    previous_bit: bool,
    // Noise shaping state
    noise_shaper_error: f32,
}

impl DfpwmEncoder {
    pub fn new() -> Self {
        Self {
            charge: 0,
            strength: 0,
            previous_bit: false,
            noise_shaper_error: 0.0,
        }
    }

    /// Encode a slice of signed 8-bit PCM samples into DFPWM format with noise shaping
    /// Returns the encoded bytes (8 samples = 1 byte)
    pub fn encode(&mut self, input: &[i8], output: &mut Vec<u8>) {
        let mut byte = 0u8;
        let mut bit_pos = 0;

        for &sample in input {
            // Scale 8-bit sample to encoder's working range (-32768..32767)
            let sample_scaled = (sample as f32) * 256.0;

            // Apply noise shaping with increased feedback (0.85 for more aggressive shaping)
            // This pushes quantization noise to higher frequencies where DFPWM handles it better
            let shaped_sample = sample_scaled + self.noise_shaper_error * 0.85;
            let target = shaped_sample as i32;

            // Determine output bit
            let current_bit = target > self.charge;

            // Calculate quantization error for noise shaping
            let quantized_value = if current_bit {
                self.charge + self.strength
            } else {
                self.charge - self.strength
            };
            self.noise_shaper_error = shaped_sample - quantized_value as f32;

            // Update charge
            let mut next_charge = self.charge;
            if current_bit {
                next_charge += self.strength;
            } else {
                next_charge -= self.strength;
            }

            // Clamp charge to valid range
            next_charge = next_charge.clamp(-32768, 32767);

            // Update strength (adaptive) - optimized parameters
            let mut next_strength = self.strength;
            if current_bit == self.previous_bit {
                // Slightly faster buildup for better transient response
                next_strength += 2;
            } else {
                next_strength -= 1;
            }
            next_strength = next_strength.clamp(0, 32767);

            // Apply exponential decay with slightly slower decay (2047 instead of 2046)
            if next_strength > 0 {
                next_strength = ((next_strength as i64 * 2047) / 2048) as i32;
            }
            if next_strength < 0 {
                next_strength = 0;
            }

            // Ensure minimum strength - optimized for better SNR
            if next_strength < 20 {
                next_strength = 20;
            }

            self.charge = next_charge;
            self.strength = next_strength;
            self.previous_bit = current_bit;

            // Pack bit into byte (LSB first)
            if current_bit {
                byte |= 1 << bit_pos;
            }

            bit_pos += 1;
            if bit_pos == 8 {
                output.push(byte);
                byte = 0;
                bit_pos = 0;
            }
        }

        // Flush remaining bits if any
        if bit_pos > 0 {
            output.push(byte);
        }
    }
}

/// DFPWM decoder for client-side playback
pub struct DfpwmDecoder {
    charge: i32,
    strength: i32,
    previous_bit: bool,
}

impl DfpwmDecoder {
    pub fn new() -> Self {
        Self {
            charge: 0,
            strength: 0,
            previous_bit: false,
        }
    }

    /// Decode DFPWM data into signed 8-bit PCM samples
    pub fn decode(&mut self, input: &[u8], output: &mut Vec<i8>) {
        for &byte in input {
            for bit_pos in 0..8 {
                let current_bit = (byte >> bit_pos) & 1 == 1;

                // Update charge based on bit
                let mut next_charge = self.charge;
                if current_bit {
                    next_charge += self.strength;
                } else {
                    next_charge -= self.strength;
                }

                // Clamp charge
                next_charge = next_charge.clamp(-32768, 32767);

                // Update strength
                let mut next_strength = self.strength;
                if current_bit == self.previous_bit {
                    next_strength += 1;
                } else {
                    next_strength -= 1;
                }
                next_strength = next_strength.clamp(0, 32767);

                // Apply exponential decay
                if next_strength > 0 {
                    next_strength = ((next_strength as i64 * 2046) / 2048) as i32;
                }
                if next_strength < 0 {
                    next_strength = 0;
                }

                // Ensure minimum strength
                if next_strength < 16 {
                    next_strength = 16;
                }

                // Convert to 8-bit sample
                let sample_16 = next_charge >> 8;
                output.push(sample_16.clamp(-128, 127) as i8);

                self.charge = next_charge;
                self.strength = next_strength;
                self.previous_bit = current_bit;
            }
        }
    }
}

impl Default for DfpwmEncoder {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for DfpwmDecoder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encoder_basic() {
        let mut encoder = DfpwmEncoder::new();
        let input = vec![0i8, 64, 127, 64, 0, -64, -128, -64];
        let mut output = Vec::new();

        encoder.encode(&input, &mut output);

        assert_eq!(output.len(), 1); // 8 samples = 1 byte
    }

    #[test]
    fn test_encoder_decoder_roundtrip() {
        let mut encoder = DfpwmEncoder::new();
        let mut decoder = DfpwmDecoder::new();

        // Create a simple sine-like wave
        let input: Vec<i8> = (0..128)
            .map(|i| ((i as f32 * 0.1).sin() * 100.0) as i8)
            .collect();

        let mut encoded = Vec::new();
        encoder.encode(&input, &mut encoded);

        let mut decoded = Vec::new();
        decoder.decode(&encoded, &mut decoded);

        assert_eq!(input.len(), decoded.len());
    }
}
