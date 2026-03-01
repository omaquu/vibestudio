use std::sync::atomic::{AtomicU64, Ordering};

// ─── ID Generation ────────────────────────────────────────────────────────────
static ID_COUNTER: AtomicU64 = AtomicU64::new(1);

pub fn gen_id() -> String {
    format!("layer_{}", ID_COUNTER.fetch_add(1, Ordering::Relaxed))
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProjectAsset {
    pub id: String,
    pub name: String,
    pub media_url: String,
    pub asset_type: String, // "image", "audio", "video"
}

// ─── Layer Type ───────────────────────────────────────────────────────────────
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum LayerType {
    Composition,
    Workstream, // Represents a top-level group of compositions
    SpectrumCircle,
    SpectrumMountain,
    Particles,
    ParticleRings,
    Starfield,
    Tunnel,
    Kaleidoscope,
    Laser,
    Glitch,
    Text,
    Image,
    Video,
    Audio,
    Waveform,
    // New Effects
    ChromaticAberration,
    ColorCorrection,
    FilmGrain,
    VhsEffect,
    GlitchPost,
    Sharpening,
    CameraShake,
}

impl LayerType {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Composition => "Composition",
            Self::Workstream => "Workstream",
            Self::SpectrumCircle => "Spectrum Circle",
            Self::SpectrumMountain => "Spectrum Mountain",
            Self::Particles => "Particles",
            Self::ParticleRings => "Particle Rings",
            Self::Starfield => "Starfield",
            Self::Tunnel => "Tunnel",
            Self::Kaleidoscope => "Kaleidoscope",
            Self::Laser => "Laser",
            Self::Glitch => "Glitch",
            Self::Text => "Text",
            Self::Image => "Image",
            Self::Video => "Video",
            Self::Audio => "Audio",
            Self::Waveform => "Waveform",
            Self::ChromaticAberration => "Chromatic Aberration",
            Self::ColorCorrection => "Color Correction",
            Self::FilmGrain => "Film Grain",
            Self::VhsEffect => "VHS Effect",
            Self::GlitchPost => "Glitch Post",
            Self::Sharpening => "Sharpening",
            Self::CameraShake => "Camera Shake",
        }
    }

    pub fn color_hex(&self) -> &'static str {
        match self {
            Self::Composition => "#fbbf24",
            Self::Workstream => "#3b82f6", // blue-500
            Self::SpectrumCircle => "#ec4899",
            Self::SpectrumMountain => "#8b5cf6",
            Self::Particles => "#eab308",
            Self::ParticleRings => "#06b6d4",
            Self::Starfield => "#ffffff",
            Self::Tunnel => "#a78bfa",
            Self::Kaleidoscope => "#ec4899",
            Self::Laser => "#ef4444",
            Self::Glitch => "#4ade80",
            Self::Text => "#22c55e",
            Self::Image => "#60a5fa",
            Self::Video => "#a855f7",
            Self::Audio => "#f97316",
            Self::Waveform => "#f97316",
            Self::ChromaticAberration => "#f43f5e",
            Self::ColorCorrection => "#0ea5e9",
            Self::FilmGrain => "#a8a29e",
            Self::VhsEffect => "#10b981",
            Self::GlitchPost => "#8b5cf6",
            Self::Sharpening => "#eab308",
            Self::CameraShake => "#ff6b6b",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Self::Composition => "📁",
            Self::Workstream => "🌊",
            Self::Text => "T",
            Self::Image => "🖼",
            Self::Video => "🎬",
            Self::Audio => "🎵",
            Self::Waveform => "📊",
            Self::ChromaticAberration | Self::ColorCorrection | Self::FilmGrain | Self::VhsEffect | Self::GlitchPost | Self::Sharpening => "🪄",
            Self::CameraShake => "📳",
            _ => "✨",
        }
    }

    /// All types that can be added via the "Add Layer" modal
    pub fn addable_types() -> &'static [LayerType] {
        &[
            LayerType::Image,
            LayerType::Video,
            LayerType::Audio,
            LayerType::SpectrumCircle,
            LayerType::SpectrumMountain,
            LayerType::Waveform,
            LayerType::Particles,
            LayerType::ParticleRings,
            LayerType::Starfield,
            LayerType::Tunnel,
            LayerType::Kaleidoscope,
            LayerType::Laser,
            LayerType::Glitch,
            LayerType::Text,
            LayerType::ChromaticAberration,
            LayerType::ColorCorrection,
            LayerType::FilmGrain,
            LayerType::VhsEffect,
            LayerType::GlitchPost,
            LayerType::Sharpening,
            LayerType::CameraShake,
        ]
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Composition => "Group items into an isolated timeline.",
            Self::Workstream => "Top-level grouping for related compositions and assets.",
            Self::SpectrumCircle => "Circular audio visualizer reacting to frequencies.",
            Self::SpectrumMountain => "Rolling landscape that bounces to music.",
            Self::Particles => "Floating particles emitting from the center.",
            Self::ParticleRings => "Concentric rings of pulsing glowing dots.",
            Self::Starfield => "Infinite zooming stars.",
            Self::Tunnel => "Neon geometric tunnel.",
            Self::Kaleidoscope => "Mirrored kaleidoscopic fractal imagery.",
            Self::Laser => "Sweeping laser beams.",
            Self::Glitch => "Cyberpunk digital distortion layer.",
            Self::Text => "Basic text overlay on the screen.",
            Self::Image => "External image layer via URL or file.",
            Self::Video => "External video playback layer.",
            Self::Audio => "Background audio track.",
            Self::Waveform => "2D scrolling audio waveform.",
            Self::ChromaticAberration => "RGB edge splitting artifact effect.",
            Self::ColorCorrection => "Color grading adjustments.",
            Self::FilmGrain => "Cinematic noise grain overlay.",
            Self::VhsEffect => "Retro CRT monitor scanlines effect.",
            Self::GlitchPost => "Full-screen digital tearing.",
            Self::Sharpening => "Contrast and detail enhancer.",
            Self::CameraShake => "Trembling camera effect reactive to audio.",
        }
    }
}

// ─── Post-Effect Parameters ───────────────────────────────────────────────────
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AudioBand {
    None,
    Bass,
    Mid,
    Treble,
}

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
pub struct EffectParams {
    pub strength: f32,
    pub direction: i8, // 1=toward, -1=away
    pub hue_shift: f32,
    pub saturation: f32,
    pub contrast: f32,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TextParams {
    pub text: String,
    pub font_size: f32,
    pub color: String,
    pub stroke_color: String,
    pub stroke_width: f32,
    pub shadow_color: String,
    pub shadow_blur: f32,
}

impl Default for TextParams {
    fn default() -> Self {
        Self {
            text: "TEXT".to_string(),
            font_size: 48.0,
            color: "#ffffff".to_string(),
            stroke_color: "#000000".to_string(),
            stroke_width: 3.0,
            shadow_color: "#000000".to_string(),
            shadow_blur: 12.0,
        }
    }
}

impl Default for EffectParams {
    fn default() -> Self {
        Self {
            strength: 1.0,
            direction: 1,
            hue_shift: 0.0,
            saturation: 1.0,
            contrast: 1.0,
        }
    }
}

// ─── Layer ────────────────────────────────────────────────────────────────────
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Layer {
    pub id: String,
    pub name: String,
    pub layer_type: LayerType,
    pub visible: bool,
    pub parent_id: Option<String>,
    pub start_time: f64,
    pub duration: f64,
    pub fade_in: f64,
    pub fade_out: f64,
    pub opacity: f32,
    pub scale: f32,
    pub position: [f32; 3],
    pub audio_react: AudioBand,
    pub rotation: f32,
    pub skew_x: f32,
    pub skew_y: f32,
    pub flip_x: bool,
    pub flip_y: bool,
    pub perspective: [f32; 2],
    pub warp: [f32; 4],
    pub effect_params: EffectParams,
    pub text_params: TextParams,
    pub media_url: Option<String>,
    pub custom_color: Option<String>,
}

impl Layer {
    pub fn new(layer_type: LayerType, parent_id: Option<String>) -> Self {
        let id = gen_id();
        Self {
            name: format!("{} {}", layer_type.label(), &id[id.len().saturating_sub(3)..]),
            id,
            layer_type,
            visible: true,
            parent_id,
            start_time: 0.0,
            duration: if layer_type == LayerType::Composition || layer_type == LayerType::Workstream { 30.0 } else { 5.0 },
            fade_in: 0.0,
            fade_out: 0.0,
            opacity: 1.0,
            scale: 1.0,
            position: [0.0, 0.0, 0.0],
            audio_react: AudioBand::None,
            rotation: 0.0,
            skew_x: 0.0,
            skew_y: 0.0,
            flip_x: false,
            flip_y: false,
            perspective: [0.0, 0.0],
            warp: [0.0, 0.0, 0.0, 0.0],
            effect_params: EffectParams::default(),
            text_params: TextParams::default(),
            media_url: None,
            custom_color: None,
        }
    }

    pub fn new_composition(name: &str, start_time: f64, duration: f64) -> Self {
        Self {
            id: gen_id(),
            name: name.to_string(),
            layer_type: LayerType::Composition,
            visible: true,
            parent_id: None,
            start_time,
            duration,
            fade_in: 0.0,
            fade_out: 0.0,
            opacity: 1.0,
            scale: 1.0,
            position: [0.0, 0.0, 0.0],
            audio_react: AudioBand::None,
            rotation: 0.0,
            skew_x: 0.0,
            skew_y: 0.0,
            flip_x: false,
            flip_y: false,
            perspective: [0.0, 0.0],
            warp: [0.0, 0.0, 0.0, 0.0],
            effect_params: EffectParams::default(),
            text_params: TextParams::default(),
            media_url: None,
            custom_color: None,
        }
    }

    pub fn new_workstream(name: &str) -> Self {
        let mut ws = Self::new(LayerType::Workstream, None);
        ws.name = name.to_string();
        ws
    }
}

// ─── Drag State ───────────────────────────────────────────────────────────────
#[derive(Clone, Debug, Default)]
pub struct DragState {
    pub source_id: Option<String>,
    pub hover_target_id: Option<String>,
    pub is_canvas_drag: bool,
    pub last_pos: Option<(f64, f64)>,
}

// ─── Clip Drag (Timeline Move / Trim) ─────────────────────────────────────────
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClipDragMode {
    Move,
    TrimLeft,
    TrimRight,
    FadeIn,
    FadeOut,
}

#[derive(Clone, Debug, Default)]
pub struct ClipDragState {
    pub mode: Option<ClipDragMode>,
    pub layer_id: Option<String>,
    pub start_pointer_x: f64,
    pub original_start_time: f64,
    pub original_duration: f64,
    pub original_fade_in: f64,
    pub original_fade_out: f64,
}

// ─── App State ────────────────────────────────────────────────────────────────
#[derive(Clone, Debug)]
pub struct AppState {
    pub layers: Vec<Layer>,
    pub selected_id: Option<String>,
    pub open_comps: Vec<String>,     // IDs of open compositions
    pub master_open: bool,
    pub current_time: f64,
    pub is_playing: bool,
    pub timeline_zoom: f32,
    pub timeline_scroll_x: f64,
    pub timeline_scroll_zoom: bool,
    pub is_scrubbing: bool,
    pub is_panning_timeline: bool,
    pub last_pan_x: f64,
    pub drag: DragState,
    pub clip_drag: ClipDragState,
    pub show_add_modal: bool,
    pub is_cut_mode: bool,
    pub add_parent_id: Option<String>,
    // Audio state
    pub audio_loaded: bool,
    pub audio_file_name: Option<String>,
    // Global effects
    pub global_bloom: f64,
    pub master_volume: f64,
    pub global_chromatic: f64,
    pub global_film_grain: f64,
    pub global_vhs: f64,
    pub global_color_hue: f64,
    pub global_color_saturation: f64,
    pub global_sharpening: f64,
    pub global_vignette: f64,
    // Terminal state
    pub terminal_logs: Vec<String>,
    pub show_terminal: bool,
    // Playback
    pub loop_playback: bool,
    // Timeline Snapping
    pub snap_to_grid: bool,
    // Settings panel
    pub show_settings: bool,
    pub ui_scale: f64,
    // Project properties
    pub project_name: String,
    pub project_width: u32,
    pub project_height: u32,
    pub project_assets: Vec<ProjectAsset>,
    // UI Panel Sizes
    pub left_panel_width: f64,
    pub right_panel_width: f64,
    pub bottom_panel_height: f64,
    pub resizing_panel: Option<String>,
    pub next_comp_index: usize,
}

impl Default for AppState {
    fn default() -> Self {
        let workstream = Layer::new_workstream("Workstream 1");
        let ws_id = workstream.id.clone();

        Self {
            layers: vec![workstream],
            selected_id: None,
            open_comps: vec![ws_id],
            master_open: true,
            current_time: 0.0,
            is_playing: false,
            timeline_zoom: 3.0,
            timeline_scroll_x: 0.0,
            timeline_scroll_zoom: true,
            is_scrubbing: false,
            is_panning_timeline: false,
            last_pan_x: 0.0,
            drag: DragState::default(),
            clip_drag: ClipDragState::default(),
            show_add_modal: false,
            is_cut_mode: false,
            add_parent_id: None,
            audio_loaded: false,
            audio_file_name: None,
            global_bloom: 0.5,
            master_volume: 1.0,
            global_chromatic: 0.0,
            global_film_grain: 0.0,
            global_vhs: 0.0,
            global_color_hue: 0.0,
            global_color_saturation: 1.0,
            global_sharpening: 0.0,
            global_vignette: 0.0,
            terminal_logs: vec!["VibeVisualizer Terminal v2.0 — type `help` for commands.".to_string()],
            show_terminal: true,
            loop_playback: false,
            snap_to_grid: true,
            show_settings: false,
            ui_scale: 1.0,
            project_name: "My Project".to_string(),
            project_width: 1920,
            project_height: 1080,
            project_assets: vec![],
            left_panel_width: 260.0,
            right_panel_width: 280.0,
            bottom_panel_height: 300.0,
            resizing_panel: None,
            next_comp_index: 1,
        }
    }
}

impl AppState {
    // ── Global Setting Mutators ─────────────────────────────────────────────
    pub fn log_terminal(&mut self, msg: &str) {
        self.terminal_logs.push(msg.to_string());
    }

    pub fn set_global_bloom(&mut self, bloom: f64) {
        self.global_bloom = bloom;
        self.log_terminal(&format!("> [SYSTEM] Saved preset: Bloom strength = {:.2}", bloom));
    }

    // ── Queries ────────────────────────────────────────────────────────────

    pub fn root_workstreams(&self) -> Vec<&Layer> {
        self.layers.iter()
            .filter(|l| l.layer_type == LayerType::Workstream && l.parent_id.is_none())
            .collect()
    }

    pub fn all_compositions(&self) -> Vec<&Layer> {
        self.layers.iter()
            .filter(|l| l.layer_type == LayerType::Composition)
            .collect()
    }

    pub fn root_compositions(&self) -> Vec<&Layer> {
        self.layers.iter()
            .filter(|l| l.layer_type == LayerType::Composition && l.parent_id.is_none())
            .collect()
    }

    pub fn children_of(&self, parent_id: &str) -> Vec<&Layer> {
        self.layers.iter()
            .filter(|l| l.parent_id.as_deref() == Some(parent_id))
            .collect()
    }

    pub fn unbound_layers(&self) -> Vec<&Layer> {
        self.layers.iter()
            .filter(|l| l.layer_type != LayerType::Composition && l.layer_type != LayerType::Workstream)
            .filter(|l| {
                if l.parent_id.is_none() { return true; }
                if let Some(pid) = &l.parent_id {
                    if let Some(p) = self.layers.iter().find(|parent| parent.id == *pid) {
                        return p.layer_type == LayerType::Workstream;
                    }
                }
                false
            })
            .collect()
    }

    pub fn descendants_of(&self, parent_id: &str) -> Vec<&Layer> {
        let mut result = Vec::new();
        for layer in &self.layers {
            if layer.parent_id.as_deref() == Some(parent_id) {
                result.push(layer);
                let grandchildren = self.descendants_of(&layer.id);
                result.extend(grandchildren);
            }
        }
        result
    }

    pub fn is_descendant_of(&self, potential_child: &str, potential_parent: &str) -> bool {
        let mut current = potential_child;
        loop {
            if let Some(layer) = self.layers.iter().find(|l| l.id == current) {
                match &layer.parent_id {
                    Some(pid) if pid == potential_parent => return true,
                    Some(pid) => current = pid,
                    None => return false,
                }
            } else {
                return false;
            }
        }
    }

    pub fn timeline_duration(&self) -> f64 {
        let mut max_end = 0.0_f64;
        for l in &self.layers {
            let end = l.start_time + l.duration;
            if end > max_end { max_end = end; }
        }
        max_end.max(5.0) // Provide a minimum 5 second timeline if empty
    }

    pub fn is_comp_open(&self, id: &str) -> bool {
        self.open_comps.contains(&id.to_string())
    }

    // ── Mutations ─────────────────────────────────────────────────────────

    pub fn add_layer(&mut self, mut layer: Layer) {
        if let Some(pid) = &layer.parent_id {
            if let Some(parent) = self.layers.iter().find(|l| l.id == *pid).cloned() {
                if parent.layer_type == LayerType::Composition {
                    // Default child to match parent's start and duration
                    layer.start_time = parent.start_time;
                    layer.duration = parent.duration;
                } else if parent.layer_type == LayerType::Workstream {
                    // Stagger: place new layer after the last child in this workstream
                    let max_end = self.layers.iter()
                        .filter(|l| l.parent_id.as_deref() == Some(&parent.id))
                        .map(|l| l.start_time + l.duration)
                        .fold(parent.start_time, f64::max);
                    layer.start_time = max_end;
                    if layer.start_time + layer.duration > parent.start_time + parent.duration {
                        layer.duration = (parent.start_time + parent.duration - layer.start_time).max(0.5);
                    }
                } else {
                    layer.start_time = parent.start_time;
                    if layer.start_time + layer.duration > parent.start_time + parent.duration {
                        layer.duration = (parent.duration).min(layer.duration);
                    }
                }
            }
        }
        // Apply snap if enabled
        if self.snap_to_grid {
            layer.start_time = (layer.start_time * 10.0).round() / 10.0;
        }
        let id = layer.id.clone();
        self.layers.push(layer);
        self.selected_id = Some(id);
    }

    pub fn remove_layer(&mut self, id: &str) {
        // Orphan children first (set parent to None)
        let children_ids: Vec<String> = self.layers.iter()
            .filter(|l| l.parent_id.as_deref() == Some(id))
            .map(|l| l.id.clone())
            .collect();
        for cid in children_ids {
            if let Some(child) = self.layers.iter_mut().find(|l| l.id == cid) {
                child.parent_id = None;
            }
        }
        self.layers.retain(|l| l.id != id);
        if self.selected_id.as_deref() == Some(id) {
            self.selected_id = None;
        }
    }

    pub fn toggle_visibility(&mut self, id: &str) {
        if let Some(layer) = self.layers.iter_mut().find(|l| l.id == id) {
            layer.visible = !layer.visible;
        }
    }

    pub fn rename_layer(&mut self, id: &str, new_name: String) {
        if let Some(layer) = self.layers.iter_mut().find(|l| l.id == id) {
            layer.name = new_name;
        }
    }

    pub fn reparent(&mut self, layer_id: &str, new_parent: Option<String>) {
        if layer_id.starts_with("asset:") {
            let asset_id = layer_id.trim_start_matches("asset:");
            if let Some(asset) = self.project_assets.iter().find(|a| a.id == asset_id).cloned() {
                let layer_type = match asset.asset_type.as_str() {
                    "audio" => LayerType::Audio,
                    "video" => LayerType::Video,
                    _ => LayerType::Image,
                };
                let mut layer = Layer::new(layer_type, new_parent);
                layer.name = asset.name.clone();
                layer.media_url = Some(asset.media_url.clone());
                if layer.layer_type == LayerType::Audio {
                    layer.opacity = 1.0; // max volume by default
                }
                self.layers.push(layer);
            }
            return;
        }

        // Prevent circular references
        if let Some(ref parent) = new_parent {
            if self.is_descendant_of(parent, layer_id) || parent == layer_id {
                return;
            }
        }
        if let Some(layer) = self.layers.iter_mut().find(|l| l.id == layer_id) {
            layer.parent_id = new_parent.clone();
        }

        // Auto-expand composition if the dropped item goes past it
        if let Some(ref parent) = new_parent {
            if let Some(child) = self.layers.iter().find(|l| l.id == layer_id).cloned() {
                let child_end = child.start_time + child.duration;
                self.expand_parent_duration(parent, child_end);
            }
        }
    }

    pub fn expand_parent_duration(&mut self, parent_id: &str, child_end_time: f64) {
        if let Some(parent) = self.layers.iter().find(|l| l.id == parent_id).cloned() {
            if child_end_time > parent.start_time + parent.duration {
                let new_dur = child_end_time - parent.start_time;
                if let Some(p) = self.layers.iter_mut().find(|l| l.id == parent_id) {
                    p.duration = new_dur;
                }
                if let Some(grandparent_id) = parent.parent_id {
                    self.expand_parent_duration(&grandparent_id, child_end_time);
                }
            }
        }
    }

    pub fn split_layer(&mut self, layer_id: &str, time: f64) {
        let idx = self.layers.iter().position(|l| l.id == layer_id);
        if let Some(i) = idx {
            let mut original = self.layers[i].clone();
            if time > original.start_time && time < original.start_time + original.duration {
                let first_dur = time - original.start_time;
                let second_dur = original.duration - first_dur;
                
                let orig_name = original.name.clone();
                original.duration = first_dur;
                original.name = format!("{} (A)", orig_name);
                self.layers[i] = original.clone();
                
                let mut second_half = original.clone();
                second_half.id = gen_id();
                second_half.start_time = time;
                second_half.duration = second_dur;
                second_half.name = format!("{} (B)", orig_name);
                // parent_id is preserved from clone
                self.layers.insert(i + 1, second_half);
            }
        }
    }

    pub fn reorder_layer(&mut self, layer_id: &str, target_id: &str, before: bool) {
        if layer_id == target_id {
            return;
        }
        
        // Find current index of layer
        let layer_idx = self.layers.iter().position(|l| l.id == layer_id);
        if layer_idx.is_none() { return; }
        let layer_idx = layer_idx.unwrap();

        // Extract layer
        let layer = self.layers.remove(layer_idx);

        // Find new target index (post removal)
        let target_idx = self.layers.iter().position(|l| l.id == target_id);
        if let Some(mut tid) = target_idx {
            // Adopt target parent
            let parent_id = self.layers[tid].parent_id.clone();
            
            if !before {
                tid += 1;
            }
            let mut l = layer;
            l.parent_id = parent_id;
            self.layers.insert(tid, l);
        } else {
            // Target missing, just push back
            self.layers.push(layer);
        }
    }

    pub fn update_layer_timing(&mut self, id: &str, start_time: f64, duration: f64) {
        if let Some(layer) = self.layers.iter_mut().find(|l| l.id == id) {
            layer.start_time = start_time;
            layer.duration = duration;
        }
    }

    pub fn seek_to(&mut self, time: f64) {
        self.current_time = time.max(0.0).min(self.timeline_duration());
    }

    // ── Clip Drag Helpers ──────────────────────────────────────────────────

    pub fn begin_clip_drag(&mut self, layer_id: &str, mode: ClipDragMode, pointer_x: f64) {
        if let Some(layer) = self.layers.iter().find(|l| l.id == layer_id) {
            self.clip_drag = ClipDragState {
                mode: Some(mode),
                layer_id: Some(layer_id.to_string()),
                start_pointer_x: pointer_x,
                original_start_time: layer.start_time,
                original_duration: layer.duration,
                original_fade_in: layer.fade_in,
                original_fade_out: layer.fade_out,
            };
            self.selected_id = Some(layer_id.to_string());
        }
    }

    pub fn update_clip_drag(&mut self, pointer_x: f64, pixels_per_second: f64) {
        let cd = self.clip_drag.clone();
        if let (Some(mode), Some(ref lid)) = (cd.mode, &cd.layer_id) {
            let delta_px = pointer_x - cd.start_pointer_x;
            let delta_secs = delta_px / pixels_per_second;
            
            let mut is_comp = false;
            let mut parent_bounds = None;
            
            // Collect sibling edge times for edge-snapping
            let mut sibling_edges: Vec<f64> = Vec::new();
            if self.snap_to_grid {
                let dragged_parent = self.layers.iter().find(|l| l.id == *lid).and_then(|l| l.parent_id.clone());
                for l in &self.layers {
                    if l.id == *lid { continue; }
                    if l.parent_id == dragged_parent || l.layer_type == LayerType::Composition {
                        sibling_edges.push(l.start_time);
                        sibling_edges.push(l.start_time + l.duration);
                    }
                }
            }
            
            // Snap helper: grid snap + edge snap to siblings
            let snap_threshold = 5.0 / pixels_per_second; // 5px in seconds
            let snap = |t: f64| -> f64 {
                if !self.snap_to_grid { return t; }
                // First try edge-snap to siblings
                for &edge in &sibling_edges {
                    if (t - edge).abs() < snap_threshold {
                        return edge;
                    }
                }
                // Fall back to grid snap (0.1s)
                (t * 10.0).round() / 10.0
            };
            
            if let Some(layer) = self.layers.iter().find(|l| l.id == *lid) {
                if let Some(pid) = &layer.parent_id {
                    if let Some(parent) = self.layers.iter().find(|l| l.id == *pid) {
                        if parent.layer_type != LayerType::Workstream {
                            parent_bounds = Some((parent.start_time, parent.start_time + parent.duration));
                        }
                    }
                }
            }
            
            if let Some(layer) = self.layers.iter_mut().find(|l| l.id == *lid) {
                is_comp = layer.layer_type == LayerType::Composition && layer.parent_id.is_none();
                
                match mode {
                    ClipDragMode::Move => {
                        let mut new_start = (cd.original_start_time + delta_secs).max(0.0);
                        if let Some((p_start, p_end)) = parent_bounds {
                            new_start = new_start.clamp(p_start, (p_end - layer.duration).max(p_start));
                        }
                        layer.start_time = snap(new_start);
                    }
                    ClipDragMode::TrimLeft => {
                        let mut new_start = (cd.original_start_time + delta_secs).max(0.0);
                        if let Some((p_start, _)) = parent_bounds {
                            new_start = new_start.max(p_start);
                        }
                        let end = cd.original_start_time + cd.original_duration;
                        if new_start < end - 0.1 {
                            let snapped_start = snap(new_start);
                            if snapped_start < end - 0.1 {
                                layer.start_time = snapped_start;
                                layer.duration = end - snapped_start;
                            }
                        }
                    }
                    ClipDragMode::TrimRight => {
                        let mut new_dur = (cd.original_duration + delta_secs).max(0.1);
                        if let Some((_, p_end)) = parent_bounds {
                            if layer.start_time + new_dur > p_end {
                                new_dur = (p_end - layer.start_time).max(0.1);
                            }
                        }
                        layer.duration = snap(new_dur).max(0.1);
                    }
                    ClipDragMode::FadeIn => {
                        layer.fade_in = snap((cd.original_fade_in + delta_secs).max(0.0).min(layer.duration - layer.fade_out));
                    }
                    ClipDragMode::FadeOut => {
                        layer.fade_out = snap((cd.original_fade_out - delta_secs).max(0.0).min(layer.duration - layer.fade_in));
                    }
                }
            }
            if is_comp {
                // Compositions are now absolutely positioned; sequence packing is disabled
                // self.enforce_composition_sequence();
            }
        }
    }

    pub fn enforce_composition_sequence(&mut self) {
        let workstreams = self.root_workstreams().into_iter().map(|w| w.id.clone()).collect::<Vec<_>>();
        
        let mut groups_to_sequence = vec![None]; // root
        for ws_id in workstreams {
            groups_to_sequence.push(Some(ws_id));
        }

        for pid in groups_to_sequence {
            let mut comp_timings: Vec<(String, f64)> = self.layers.iter()
                .filter(|l| l.layer_type == LayerType::Composition && l.parent_id == pid)
                .map(|l| (l.id.clone(), l.start_time))
                .collect();

            // Sort by current start_time purely to determine sequence order
            comp_timings.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

            // Pack them together starting from 0.0
            let mut current_time = 0.0;
            for (id, _) in comp_timings {
                if let Some(layer) = self.layers.iter_mut().find(|l| l.id == id) {
                    layer.start_time = current_time;
                    current_time += layer.duration;
                }
            }
        }
    }

    pub fn end_clip_drag(&mut self) {
        self.clip_drag = ClipDragState::default();
    }

    pub fn toggle_comp(&mut self, id: &str) {
        if let Some(pos) = self.open_comps.iter().position(|x| x == id) {
            self.open_comps.remove(pos);
        } else {
            self.open_comps.push(id.to_string());
        }
    }

    pub fn add_composition(&mut self, target_ws_id: Option<&String>) {
        let target_ws = if let Some(id) = target_ws_id {
            Some(id.clone())
        } else {
            let workstreams = self.root_workstreams();
            workstreams.first().map(|w| w.id.clone())
        };

        let comps = self.all_compositions();
        let mut max_end = 0.0_f64;
        for c in &comps {
            if c.parent_id == target_ws {
                let end = c.start_time + c.duration;
                if end > max_end { max_end = end; }
            }
        }
        self.next_comp_index += 1;
        let count = self.next_comp_index;
        let mut comp = Layer::new_composition(&format!("Composition {}", count), max_end, 30.0);
        comp.parent_id = target_ws;
        let comp_id = comp.id.clone();
        self.add_layer(comp);
        self.open_comps.push(comp_id);
    }

    pub fn add_workstream(&mut self) {
        self.add_workstream_with_duration(30.0);
    }

    pub fn add_workstream_with_duration(&mut self, duration: f64) {
        let ws_count = self.root_workstreams().len() + 1;
        let mut ws = Layer::new_workstream(&format!("Workstream {}", ws_count));
        ws.duration = duration;
        let ws_id = ws.id.clone();
        self.layers.push(ws);
        self.open_comps.push(ws_id);
    }

    /// Returns the next active time if `from` falls in a gap between workstreams.
    /// If `from` is inside a workstream, returns `from` unchanged.
    pub fn next_active_time(&self, from: f64) -> f64 {
        let workstreams = self.root_workstreams();
        // Check if we're inside any workstream
        for ws in &workstreams {
            if from >= ws.start_time && from <= ws.start_time + ws.duration {
                return from; // inside a workstream, no skip
            }
        }
        // We're in a gap — find the nearest workstream start after `from`
        let mut nearest = f64::MAX;
        for ws in &workstreams {
            if ws.start_time > from && ws.start_time < nearest {
                nearest = ws.start_time;
            }
        }
        if nearest < f64::MAX { nearest } else { from }
    }
}
