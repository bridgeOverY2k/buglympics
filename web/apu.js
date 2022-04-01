// apu-worklet.js
class APUWorklet extends AudioWorkletProcessor {

  constructor(options){
    super(options);
    this.port.onmessage = (e) => {
      //console.log(e.data.audioDataPtr);
      this.samples = e.data.samples;
      console.log(this.samples);
    }
  }

  process (inputs, outputs, parameters) {
    const output = outputs[0];
    const sampleSize = 128;
    //console.log(sampleSize);
    output[0] = this.samples.slice(0, sampleSize);
    //console.log(output[0]);
    return true
  }
}

registerProcessor('apu-worklet', APUWorklet)