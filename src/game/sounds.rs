use zingr::lentsys::LentSysBus;
use zingr::apu::synth::Instrument;
use zingr::apu::synth::{AmpEnvelope, WaveForm};
use zingr::apu::music::AudioSource;

pub enum SFX {
  JumpA,
  JumpB,
  Fire,
  Ski,
  Select,
  Switch,
}

pub fn prepare_effects(bus: &mut LentSysBus){
  bus.apu.synths.push(Instrument {
    volume: 0.4,
    waveform: WaveForm::NOISE,
    duty: 0.75,
    amp_envelope: AmpEnvelope{
      attack: 0.0,
      decay: 0.0,
      sustain: 0.0,
      sustain_level: 1.0,
      release: 1.0,
      ..AmpEnvelope::default()
    },
    ..Instrument::default()
  });

  bus.apu.synths.push(Instrument {
    volume: 0.4,
    waveform: WaveForm::PULSE,
    duty: 0.6,
    amp_envelope: AmpEnvelope{
      attack: 0.0,
      decay: 0.0,
      sustain: 0.0,
      sustain_level: 1.0,
      release: 1.0,
      ..AmpEnvelope::default()
    },
    ..Instrument::default()
  });

  bus.apu.synths.push(Instrument {
    volume: 0.4,
    waveform: WaveForm::TRIANGLE,
    duty: 0.5,
    amp_envelope: AmpEnvelope{
      attack: 0.0,
      decay: 0.0,
      sustain: 0.0,
      sustain_level: 1.0,
      release: 1.0,
      ..AmpEnvelope::default()
    },
    ..Instrument::default()
  });

}

pub fn play_effect(bus: &mut LentSysBus, effect: SFX, num_samples: usize){
  bus.apu.synths[1].counter = 0;
  bus.apu.synths[2].counter = 0;
  //bus.apu.synths[0].counter = 0;
  match effect {
    SFX::JumpA => {
      bus.apu.fx_queue.push((200.0, AudioSource::Instrument, 2, num_samples));
    },
    SFX::JumpB => {
      bus.apu.fx_queue.push((110.0, AudioSource::Instrument, 1, num_samples));
    },
    SFX::Ski => {
      bus.apu.fx_queue.push((20.0, AudioSource::Instrument, 0, num_samples));
    },
    SFX::Fire => {
      bus.apu.fx_queue.push((10.0, AudioSource::Instrument, 1, num_samples));
    },
    SFX::Select => {
      bus.apu.fx_queue.push((300.0, AudioSource::Instrument, 2, num_samples));
    },
    SFX::Switch => {
      bus.apu.fx_queue.push((500.0, AudioSource::Instrument, 2, num_samples));
    }
  }
}