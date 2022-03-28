// apu-worklet.js
class APUWorklet extends AudioWorkletProcessor {

  constructor(options){
    super();
    console.log(options);
  }

  process (inputs, outputs, parameters) {
    const output = outputs[0];
    const sampleSize = 128;
    const sampleRate = 44100;
    const time = sampleSize / sampleRate;
    console.log(inputs, parameters);
    inputs.lentsys.render_audio(time);
    output.forEach(channel => {
      for (let i = 0; i < channel; i++) {
        channel[i] = new Float32Array(128).fill(0)//new Float32Array(inputs.wasmMemoryBuffer, inputs.lentsys.get_audio_data(), 128);;
      }
    })
    return true
  }
}

registerProcessor('apu-worklet', APUWorklet)