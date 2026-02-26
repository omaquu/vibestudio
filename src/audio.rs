use wasm_bindgen::prelude::*;
use web_sys::{HtmlAudioElement, File, Url};

pub struct AudioEngine {
    audio_el: HtmlAudioElement,
    object_url: Option<String>,
}

impl AudioEngine {
    pub fn new() -> Result<Self, JsValue> {
        let audio_el = HtmlAudioElement::new()?;
        audio_el.set_cross_origin(Some("anonymous"));
        audio_el.set_id("vibe-master-audio");
        audio_el.set_attribute("style", "display: none;")?;

        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(body) = document.body() {
                    let _ = body.append_child(&audio_el);
                }
            }
        }

        Ok(Self {
            audio_el,
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
        self.audio_el.set_volume(volume);
    }

    pub fn current_time(&self) -> f64 {
        self.audio_el.current_time()
    }

    pub fn duration(&self) -> f64 {
        self.audio_el.duration()
    }


}

impl Drop for AudioEngine {
    fn drop(&mut self) {
        if let Some(url) = &self.object_url {
            let _ = Url::revoke_object_url(url);
        }
        self.audio_el.remove();
    }
}
