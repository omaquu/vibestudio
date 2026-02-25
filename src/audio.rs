use wasm_bindgen::prelude::*;
use web_sys::{AudioContext, AnalyserNode, GainNode, HtmlAudioElement, File, Url, MediaElementAudioSourceNode};

pub struct AudioEngine {
    audio_el: HtmlAudioElement,
    pub context: AudioContext,
    pub analyser: AnalyserNode,
    pub gain: GainNode,
    pub source: Option<MediaElementAudioSourceNode>,
    object_url: Option<String>,
}

impl AudioEngine {
    pub fn new() -> Result<Self, JsValue> {
        let audio_el = HtmlAudioElement::new()?;
        audio_el.set_cross_origin(Some("anonymous"));
        
        let context = AudioContext::new()?;
        let analyser = context.create_analyser()?;
        analyser.set_fft_size(256);
        
        let gain = context.create_gain()?;
        gain.connect_with_audio_node(&context.destination())?;
        analyser.connect_with_audio_node(&gain)?;
        
        let source = context.create_media_element_source(&audio_el)?;
        source.connect_with_audio_node(&analyser)?;

        Ok(Self {
            audio_el,
            context,
            analyser,
            gain,
            source: Some(source),
            object_url: None,
        })
    }

    pub fn load_file(&mut self, file: &File) -> Result<(), JsValue> {
        if let Some(url) = &self.object_url {
            let _ = Url::revoke_object_url(url);
        }
        
        let url = Url::create_object_url_with_blob(file)?;
        self.audio_el.set_src(&url);
        self.object_url = Some(url);
        
        Ok(())
    }

    pub fn load_url(&mut self, url: &str) {
        if let Some(old_url) = &self.object_url {
            let _ = Url::revoke_object_url(old_url);
            self.object_url = None;
        }
        self.audio_el.set_src(url);
    }

    pub fn play(&self) -> Result<(), JsValue> {
        let _ = self.context.resume();
        let _ = self.audio_el.play()?;
        Ok(())
    }

    pub fn pause(&self) -> Result<(), JsValue> {
        self.audio_el.pause()
    }

    pub fn seek(&self, time: f64) {
        self.audio_el.set_current_time(time);
    }

    pub fn set_volume(&self, volume: f64) {
        self.gain.gain().set_value(volume as f32);
    }

    pub fn current_time(&self) -> f64 {
        self.audio_el.current_time()
    }

    pub fn duration(&self) -> f64 {
        self.audio_el.duration()
    }

    pub fn get_frequency_data(&self) -> Vec<u8> {
        let mut data = vec![0; self.analyser.frequency_bin_count() as usize];
        self.analyser.get_byte_frequency_data(&mut data);
        data
    }
}

impl Drop for AudioEngine {
    fn drop(&mut self) {
        if let Some(url) = &self.object_url {
            let _ = Url::revoke_object_url(url);
        }
        let _ = self.context.close();
    }
}
