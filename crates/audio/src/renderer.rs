use crate::AudioData;

pub enum StereoMode {
    Stereo,
    MonoL,
    MonoR,
}

// Renders a combination of audio tracks into a single stereo buffer
pub fn render_audio(
    tracks: &[(&AudioData, f32, StereoMode)],
    offset: usize,
    samples: usize,
    out_offset: usize,
    out: &mut [f32],
) -> usize {
    if tracks
        .iter()
        .any(|t| (t.0.samples().len() / t.0.channels() as usize) < offset)
    {
        return 0;
    }

    let samples = samples.min(
        tracks
            .iter()
            .map(|t| (t.0.samples().len() / t.0.channels() as usize) - offset)
            .min()
            .unwrap_or(usize::MAX),
    );

    for i in 0..samples {
        let mut left = 0.0;
        let mut right = 0.0;

        for track in tracks {
            let gain = gain_for_db(track.1);

            if gain == f32::NEG_INFINITY {
                continue;
            }

            if track.0.channels() == 1 {
                left += track.0.samples()[offset + i] * 0.707 * gain;
                right += track.0.samples()[offset + i] * 0.707 * gain;
            } else if track.0.channels() == 2 {
                let base_idx = offset * 2 + i * 2;
                let l_sample = track.0.samples()[base_idx];
                let r_sample = track.0.samples()[base_idx + 1];

                match track.2 {
                    StereoMode::Stereo => {
                        left += l_sample * gain;
                        right += r_sample * gain;
                    }
                    StereoMode::MonoL => {
                        left += l_sample * gain;
                        right += l_sample * gain;
                    }
                    StereoMode::MonoR => {
                        left += r_sample * gain;
                        right += r_sample * gain;
                    }
                }
            }
        }

        out[out_offset + i * 2] = left;
        out[out_offset + i * 2 + 1] = right;
    }

    samples
}

fn gain_for_db(db: f32) -> f32 {
    match db {
        // Fully mute when at minimum
        v if v <= -30.0 => f32::NEG_INFINITY,
        v => db_to_linear(v),
    }
}
fn db_to_linear(db: f32) -> f32 {
    10.0_f32.powf(db / 20.0)
}
