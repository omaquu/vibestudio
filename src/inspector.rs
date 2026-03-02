use dioxus::prelude::*;
use crate::model::*;

// ─── Reusable UI Components ──────────────────────────────────────────────────

#[component]
fn Slider(label: String, min: f64, max: f64, step: f64, value: f64, on_change: EventHandler<f64>) -> Element {
    let display_val = if step < 0.01 {
        format!("{:.3}", value)
    } else if step < 0.1 {
        format!("{:.2}", value)
    } else {
        format!("{:.1}", value)
    };

    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 2px;",
            div { style: "display: flex; justify-content: space-between; align-items: center;",
                label { style: "font-size: 10px; color: rgba(255,255,255,0.4);", "{label}" }
                span { style: "font-size: 10px; font-family: monospace; color: rgba(255,255,255,0.5); width: 36px; text-align: right;", "{display_val}" }
            }
            input {
                r#type: "range",
                min: "{min}",
                max: "{max}",
                step: "{step}",
                value: "{value}",
                style: "width: 100%; height: 4px; appearance: none; background: rgba(255,255,255,0.1); border-radius: 2px; outline: none; cursor: pointer;",
                oninput: move |evt| {
                    if let Ok(v) = evt.value().parse::<f64>() {
                        on_change.call(v);
                    }
                }
            }
        }
    }
}

#[component]
fn Toggle(label: String, checked: bool, on_change: EventHandler<bool>) -> Element {
    let bg_color = if checked { "var(--accent-color)" } else { "rgba(255,255,255,0.1)" };
    let transform_val = if checked { "translateX(14px)" } else { "translateX(0)" };

    rsx! {
        div {
            style: "display: flex; justify-content: space-between; align-items: center; cursor: pointer;",
            onclick: move |_| on_change.call(!checked),
            span { style: "font-size: 10px; color: rgba(255,255,255,0.4);", "{label}" }
            div {
                style: "width: 28px; height: 14px; border-radius: 7px; background: {bg_color}; display: flex; align-items: center; padding: 0 2px; transition: background 0.2s;",
                div {
                    style: "width: 10px; height: 10px; border-radius: 50%; background: #fff; transition: transform 0.2s; transform: {transform_val};"
                }
            }
        }
    }
}

#[component]
fn Section(title: String, default_open: Option<bool>, children: Element) -> Element {
    let mut open = use_signal(|| default_open.unwrap_or(true));

    rsx! {
        div { style: "border-bottom: 1px solid rgba(255,255,255,0.05);",
            div {
                style: "display: flex; align-items: center; gap: 5px; padding: 8px 0; cursor: pointer; user-select: none;",
                onclick: move |_| open.set(!open()),
                if *open.read() {
                    span { style: "font-size: 11px; color: rgba(255,255,255,0.6);", "▾" }
                } else {
                    span { style: "font-size: 11px; color: rgba(255,255,255,0.6);", "▸" }
                }
                span { style: "font-size: 10px; font-weight: 600; text-transform: uppercase; letter-spacing: 0.05em; color: rgba(255,255,255,0.6);", "{title}" }
            }
            if *open.read() {
                div { style: "padding-bottom: 10px; display: flex; flex-direction: column; gap: 6px;",
                    {children}
                }
            }
        }
    }
}

// ─── Main Inspector Component ─────────────────────────────────────────────────

#[component]
pub fn Inspector() -> Element {
    let mut state = use_context::<Signal<AppState>>();
    let audio_ctx = use_context::<std::rc::Rc<std::cell::RefCell<Option<crate::audio::AudioEngine>>>>();
    let s = state.read();

    let selected = s.selected_id.as_ref()
        .and_then(|id| s.layers.iter().find(|l| l.id == *id))
        .cloned();

    let is_global = s.selected_id.as_deref() == Some("__global_effects__");
    let is_project = s.selected_id.as_deref() == Some("__project__");
    
    // Check if selected is a composition
    let is_composition = selected.as_ref().map(|l| l.layer_type == LayerType::Composition).unwrap_or(false);
    let comp_children: Vec<Layer> = if is_composition {
        let sel_id = selected.as_ref().unwrap().id.clone();
        s.layers.iter().filter(|l| l.parent_id.as_deref() == Some(&sel_id)).cloned().collect()
    } else {
        vec![]
    };

    rsx! {
        div { 
            style: "width: {state.read().right_panel_width}px; height: 100%; padding: 10px; display: flex; flex-direction: column; gap: 0; z-index: 10; overflow-y: auto; overflow-x: hidden; background: #0e0e16; border-left: 1px solid rgba(255,255,255,0.05); flex-shrink: 0;",
            if is_project {
                div {
                    style: "display: flex; flex-direction: column; gap: 10px;",
                    div {
                        style: "font-size: 11px; font-weight: 600; color: rgba(255,255,255,0.8); text-transform: uppercase; letter-spacing: 0.05em; margin-bottom: 4px;",
                        "Project Settings"
                    }
                    Section {
                        title: "General".to_string(),
                        default_open: true,
                        div { style: "display: flex; flex-direction: column; gap: 6px;",
                            div { style: "display: flex; flex-direction: column; gap: 2px;",
                                label { style: "font-size: 9px; color: rgba(255,255,255,0.3);", "Project Name" }
                                input {
                                    r#type: "text",
                                    class: "glass-input",
                                    style: "font-size: 10px; padding: 4px 6px;",
                                    value: "{state.read().project_name}",
                                    oninput: move |evt| {
                                        state.write().project_name = evt.value().clone();
                                    }
                                }
                            }
                        }
                    }
                    Section {
                        title: "Resolution".to_string(),
                        default_open: true,
                        div { style: "display: flex; flex-direction: column; gap: 6px;",
                            div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 6px;",
                                div { style: "display: flex; flex-direction: column; gap: 2px;",
                                    label { style: "font-size: 9px; color: rgba(255,255,255,0.3);", "Width" }
                                    input {
                                        r#type: "number",
                                        class: "glass-input",
                                        style: "font-size: 10px; padding: 4px 6px;",
                                        value: "{state.read().project_width}",
                                        oninput: move |evt| {
                                            if let Ok(v) = evt.value().parse::<u32>() {
                                                state.write().project_width = v.max(1);
                                            }
                                        }
                                    }
                                }
                                div { style: "display: flex; flex-direction: column; gap: 2px;",
                                    label { style: "font-size: 9px; color: rgba(255,255,255,0.3);", "Height" }
                                    input {
                                        r#type: "number",
                                        class: "glass-input",
                                        style: "font-size: 10px; padding: 4px 6px;",
                                        value: "{state.read().project_height}",
                                        oninput: move |evt| {
                                            if let Ok(v) = evt.value().parse::<u32>() {
                                                state.write().project_height = v.max(1);
                                            }
                                        }
                                    }
                                }
                            }
                            div { style: "display: flex; gap: 4px; flex-wrap: wrap;",
                                {
                                    let presets = [("1920×1080", 1920u32, 1080u32), ("1280×720", 1280, 720), ("3840×2160", 3840, 2160), ("1080×1920", 1080, 1920), ("1080×1080", 1080, 1080)];
                                    rsx! {
                                        for (label, w, h) in presets.into_iter() {
                                            button {
                                                style: "font-size: 8px; padding: 2px 6px; border: 1px solid rgba(255,255,255,0.1); border-radius: 3px; background: rgba(255,255,255,0.05); color: rgba(255,255,255,0.5); cursor: pointer;",
                                                onclick: move |_| {
                                                    let mut s = state.write();
                                                    s.project_width = w;
                                                    s.project_height = h;
                                                },
                                                "{label}"
                                            }
                                        }
                                    }
                                }
                            }
                            {
                                let w = state.read().project_width;
                                let h = state.read().project_height;
                                let aspect = if h > 0 { format!("{:.2}:1", w as f64 / h as f64) } else { "N/A".to_string() };
                                rsx! {
                                    div { style: "font-size: 9px; color: rgba(255,255,255,0.25); margin-top: 4px;", "Aspect Ratio: {aspect}" }
                                }
                            }
                        }
                    }
                }
            } else if is_global {
                div {
                    style: "display: flex; flex-direction: column; gap: 10px;",
                    div {
                        style: "font-size: 11px; font-weight: 600; color: rgba(255,255,255,0.8); text-transform: uppercase; letter-spacing: 0.05em; margin-bottom: 8px;",
                        "Global Effects"
                    }
                    div { style: "font-size: 11px; color: rgba(255,255,255,0.4);", "Effects applied to the final render." }
                    
                    Section {
                        title: "Post-Processing".to_string(),
                        default_open: true,

                        Slider {
                            label: "Bloom Strength".to_string(),
                            value: state.read().global_bloom,
                            min: 0.0,
                            max: 2.0,
                            step: 0.05,
                            on_change: move |v| {
                                state.write().set_global_bloom(v);
                            }
                        }
                        div { style: "height: 6px;" }
                        Slider {
                            label: "Chromatic Aberration".to_string(),
                            value: state.read().global_chromatic,
                            min: 0.0,
                            max: 1.0,
                            step: 0.01,
                            on_change: move |v| { state.write().global_chromatic = v; }
                        }
                        div { style: "height: 6px;" }
                        Slider {
                            label: "Film Grain".to_string(),
                            value: state.read().global_film_grain,
                            min: 0.0,
                            max: 1.0,
                            step: 0.01,
                            on_change: move |v| { state.write().global_film_grain = v; }
                        }
                        div { style: "height: 6px;" }
                        Slider {
                            label: "VHS Effect".to_string(),
                            value: state.read().global_vhs,
                            min: 0.0,
                            max: 1.0,
                            step: 0.01,
                            on_change: move |v| { state.write().global_vhs = v; }
                        }
                        div { style: "height: 6px;" }
                        Slider {
                            label: "Color Shift (Hue)".to_string(),
                            value: state.read().global_color_hue,
                            min: -180.0,
                            max: 180.0,
                            step: 1.0,
                            on_change: move |v| { state.write().global_color_hue = v; }
                        }
                        div { style: "height: 6px;" }
                        Slider {
                            label: "Color Saturation".to_string(),
                            value: state.read().global_color_saturation,
                            min: 0.0,
                            max: 2.0,
                            step: 0.05,
                            on_change: move |v| { state.write().global_color_saturation = v; }
                        }
                        div { style: "height: 6px;" }
                        Slider {
                            label: "Sharpening".to_string(),
                            value: state.read().global_sharpening,
                            min: 0.0,
                            max: 2.0,
                            step: 0.05,
                            on_change: move |v| { state.write().global_sharpening = v; }
                        }
                        div { style: "height: 6px;" }
                        Slider {
                            label: "Vignette".to_string(),
                            value: state.read().global_vignette,
                            min: 0.0,
                            max: 1.0,
                            step: 0.01,
                            on_change: move |v| { state.write().global_vignette = v; }
                        }
                    }
                }
            } else if let Some(layer) = selected {
                {
                    let id_vis = layer.id.clone();
                    let id_opacity = layer.id.clone();
                    let id_scale = layer.id.clone();
                    let id_pos_x = layer.id.clone();
                    let id_pos_y = layer.id.clone();
                    let id_pos_z = layer.id.clone();
                    let id_start = layer.id.clone();
                    let id_dur = layer.id.clone();
                    let id_fade_in = layer.id.clone();
                    let id_fade_out = layer.id.clone();
                    let id_reparent = layer.id.clone();
                    let id_filter = layer.id.clone();
                    let id_delete = layer.id.clone();
                    let id_custom_color = layer.id.clone();
                    let id_flip_x = layer.id.clone();
                    let id_flip_y = layer.id.clone();
                    let id_persp_x = layer.id.clone();
                    let id_persp_y = layer.id.clone();
                    
                    rsx! {
                        div { style: "display: flex; flex-direction: column; gap: 0;",

                            // Layer name header
                            div {
                                style: "display: flex; align-items: center; gap: 8px; padding-bottom: 10px; border-bottom: 1px solid rgba(255,255,255,0.06); margin-bottom: 6px;",
                                span { style: "font-size: 14px;", "{layer.layer_type.icon()}" }
                                div { style: "display: flex; flex-direction: column; gap: 1px; flex-grow: 1; min-width: 0;",
                                    span { style: "font-size: 12px; font-weight: 600; color: #fff; white-space: nowrap; overflow: hidden; text-overflow: ellipsis;", "{layer.name}" }
                                    span { style: "font-size: 9px; color: rgba(255,255,255,0.35); text-transform: uppercase; letter-spacing: 0.05em;", "{layer.layer_type.label()}" }
                                }
                                if layer.layer_type == LayerType::Composition {
                                    button {
                                        style: "width: 22px; height: 22px; border-radius: 4px; background: rgba(239,68,68,0.15); border: 1px solid rgba(239,68,68,0.25); color: #ef4444; cursor: pointer; display: flex; align-items: center; justify-content: center; font-size: 11px; flex-shrink: 0;",
                                        title: "Delete Composition",
                                        onclick: move |_| {
                                            state.write().remove_layer(&id_delete);
                                        },
                                        "✕"
                                    }
                                }
                            }

                            // Transform section
                            Section {
                                title: "Transform".to_string(),
                                default_open: true,

                                div { style: "display: flex; align-items: center; gap: 8px; margin-top: 6px; margin-bottom: 6px;",
                                    Toggle {
                                        label: "Visible".to_string(),
                                        checked: layer.visible,
                                        on_change: move |_| {
                                            state.write().toggle_visibility(&id_vis);
                                        }
                                    }
                                }

                                // Opacity
                                div { style: "margin-top: 6px;",
                                    Slider {
                                        label: "Opacity".to_string(),
                                        min: 0.0,
                                        max: 1.0,
                                        step: 0.05,
                                        value: layer.opacity as f64,
                                        on_change: move |v| {
                                            if let Some(l) = state.write().layers.iter_mut().find(|l| l.id == id_opacity) {
                                                l.opacity = v as f32;
                                            }
                                        }
                                    }
                                }

                                // Scale
                                div { style: "margin-top: 6px;",
                                    Slider {
                                        label: "Scale".to_string(),
                                        min: 0.1,
                                        max: 10.0,
                                        step: 0.1,
                                        value: layer.scale as f64,
                                        on_change: move |v| {
                                            if let Some(l) = state.write().layers.iter_mut().find(|l| l.id == id_scale) {
                                                l.scale = v as f32;
                                            }
                                        }
                                    }
                                }

                                // Rotation
                                {
                                    let id_rot = layer.id.clone();
                                    rsx! {
                                        div { style: "margin-top: 6px;",
                                            Slider {
                                                label: "Rotation (deg)".to_string(),
                                                min: -360.0,
                                                max: 360.0,
                                                step: 1.0,
                                                value: layer.rotation as f64,
                                                on_change: move |v| {
                                                    if let Some(l) = state.write().layers.iter_mut().find(|l| l.id == id_rot) {
                                                        l.rotation = v as f32;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }

                                // Skew X / Y
                                {
                                    let id_skx = layer.id.clone();
                                    let id_sky = layer.id.clone();
                                    rsx! {
                                        div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 4px; margin-top: 6px;",
                                            div { style: "display: flex; flex-direction: column; gap: 2px;",
                                                label { style: "font-size: 9px; color: rgba(255,255,255,0.3);", "Skew X" }
                                                input {
                                                    r#type: "range", min: "-45", max: "45", step: "1",
                                                    value: "{layer.skew_x}",
                                                    style: "width: 100%; accent-color: #f97316; cursor: pointer;",
                                                    oninput: move |evt| {
                                                        if let Ok(v) = evt.value().parse::<f64>() {
                                                            if let Some(l) = state.write().layers.iter_mut().find(|l| l.id == id_skx) {
                                                                l.skew_x = v as f32;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            div { style: "display: flex; flex-direction: column; gap: 2px;",
                                                label { style: "font-size: 9px; color: rgba(255,255,255,0.3);", "Skew Y" }
                                                input {
                                                    r#type: "range", min: "-45", max: "45", step: "1",
                                                    value: "{layer.skew_y}",
                                                    style: "width: 100%; accent-color: #f97316; cursor: pointer;",
                                                    oninput: move |evt| {
                                                        if let Ok(v) = evt.value().parse::<f64>() {
                                                            if let Some(l) = state.write().layers.iter_mut().find(|l| l.id == id_sky) {
                                                                l.skew_y = v as f32;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }

                                // Custom Color
                                {
                                    let col_val = layer.custom_color.clone().unwrap_or_else(|| layer.layer_type.color_hex().to_string());
                                    rsx! {
                                        div { style: "display: flex; align-items: center; gap: 6px; margin-top: 6px;",
                                            label { style: "font-size: 9px; color: rgba(255,255,255,0.3); flex: 1;", "Color" }
                                            input {
                                                r#type: "color",
                                                style: "width: 22px; height: 22px; padding: 0; border: none; background: none; cursor: pointer; flex-shrink: 0;",
                                                value: "{col_val}",
                                                oninput: move |evt| {
                                                    if let Some(l) = state.write().layers.iter_mut().find(|l| l.id == id_custom_color) {
                                                        l.custom_color = Some(evt.value());
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }

                                // Flip X / Y
                                {
                                    rsx! {
                                        div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 4px; margin-top: 6px;",
                                            Toggle {
                                                label: "Flip X".to_string(),
                                                checked: layer.flip_x,
                                                on_change: move |_| {
                                                    if let Some(l) = state.write().layers.iter_mut().find(|l| l.id == id_flip_x) {
                                                        l.flip_x = !l.flip_x;
                                                    }
                                                }
                                            }
                                            Toggle {
                                                label: "Flip Y".to_string(),
                                                checked: layer.flip_y,
                                                on_change: move |_| {
                                                    if let Some(l) = state.write().layers.iter_mut().find(|l| l.id == id_flip_y) {
                                                        l.flip_y = !l.flip_y;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }

                                // Perspective X / Y
                                {
                                    rsx! {
                                        div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 4px; margin-top: 6px;",
                                            div { style: "display: flex; flex-direction: column; gap: 2px;",
                                                label { style: "font-size: 9px; color: rgba(255,255,255,0.3);", "Persp X" }
                                                input {
                                                    r#type: "range", min: "-10", max: "10", step: "0.1",
                                                    value: "{layer.perspective[0]}",
                                                    style: "width: 100%; cursor: pointer;",
                                                    oninput: move |evt| {
                                                        if let Ok(v) = evt.value().parse::<f64>() {
                                                            if let Some(l) = state.write().layers.iter_mut().find(|l| l.id == id_persp_x) {
                                                                l.perspective[0] = v as f32;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            div { style: "display: flex; flex-direction: column; gap: 2px;",
                                                label { style: "font-size: 9px; color: rgba(255,255,255,0.3);", "Persp Y" }
                                                input {
                                                    r#type: "range", min: "-10", max: "10", step: "0.1",
                                                    value: "{layer.perspective[1]}",
                                                    style: "width: 100%; cursor: pointer;",
                                                    oninput: move |evt| {
                                                        if let Ok(v) = evt.value().parse::<f64>() {
                                                            if let Some(l) = state.write().layers.iter_mut().find(|l| l.id == id_persp_y) {
                                                                l.perspective[1] = v as f32;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }

                                // Position X/Y/Z
                                div { style: "display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 4px; margin-top: 6px;",
                                    div { style: "display: flex; flex-direction: column; gap: 2px;",
                                        label { style: "font-size: 9px; color: rgba(255,255,255,0.3);", "X" }
                                        input {
                                            r#type: "number", step: "0.5", class: "glass-input", style: "font-size: 10px; padding: 3px 4px;",
                                            value: "{layer.position[0]}",
                                            oninput: move |evt| {
                                                if let Ok(v) = evt.value().parse::<f64>() {
                                                    if let Some(l) = state.write().layers.iter_mut().find(|l| l.id == id_pos_x) {
                                                        l.position[0] = v as f32;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    div { style: "display: flex; flex-direction: column; gap: 2px;",
                                        label { style: "font-size: 9px; color: rgba(255,255,255,0.3);", "Y" }
                                        input {
                                            r#type: "number", step: "0.5", class: "glass-input", style: "font-size: 10px; padding: 3px 4px;",
                                            value: "{layer.position[1]}",
                                            oninput: move |evt| {
                                                if let Ok(v) = evt.value().parse::<f64>() {
                                                    if let Some(l) = state.write().layers.iter_mut().find(|l| l.id == id_pos_y) {
                                                        l.position[1] = v as f32;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    div { style: "display: flex; flex-direction: column; gap: 2px;",
                                        label { style: "font-size: 9px; color: rgba(255,255,255,0.3);", "Z" }
                                        input {
                                            r#type: "number", step: "0.5", class: "glass-input", style: "font-size: 10px; padding: 3px 4px;",
                                            value: "{layer.position[2]}",
                                            oninput: move |evt| {
                                                if let Ok(v) = evt.value().parse::<f64>() {
                                                    if let Some(l) = state.write().layers.iter_mut().find(|l| l.id == id_pos_z) {
                                                        l.position[2] = v as f32;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            
                            // Timing Section
                            Section {
                                title: "Timing".to_string(),
                                default_open: false,
                                div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 6px;",
                                    div { style: "display: flex; flex-direction: column; gap: 2px;",
                                        label { style: "font-size: 9px; color: rgba(255,255,255,0.3);", "Start (s)" }
                                        input {
                                            r#type: "number", step: "0.1", class: "glass-input", style: "font-size: 10px; padding: 3px 6px;",
                                            value: "{layer.start_time}",
                                            oninput: move |evt| {
                                                if let Ok(v) = evt.value().parse::<f64>() {
                                                    if let Some(l) = state.write().layers.iter_mut().find(|l| l.id == id_start) {
                                                        l.start_time = v.max(0.0);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    div { style: "display: flex; flex-direction: column; gap: 2px;",
                                        label { style: "font-size: 9px; color: rgba(255,255,255,0.3);", "Duration (s)" }
                                        input {
                                            r#type: "number", step: "0.1", class: "glass-input", style: "font-size: 10px; padding: 3px 6px;",
                                            value: "{layer.duration}",
                                            oninput: move |evt| {
                                                if let Ok(v) = evt.value().parse::<f64>() {
                                                    if let Some(l) = state.write().layers.iter_mut().find(|l| l.id == id_dur) {
                                                        l.duration = v.max(0.1);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                // Fade In / Fade Out
                                div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 6px; margin-top: 4px;",
                                    div { style: "display: flex; flex-direction: column; gap: 2px;",
                                        label { style: "font-size: 9px; color: rgba(255,255,255,0.3);", "Fade In (s)" }
                                        input {
                                            r#type: "number", step: "0.05", min: "0", class: "glass-input", style: "font-size: 10px; padding: 3px 6px;",
                                            value: "{layer.fade_in}",
                                            oninput: move |evt| {
                                                if let Ok(v) = evt.value().parse::<f64>() {
                                                    if let Some(l) = state.write().layers.iter_mut().find(|l| l.id == id_fade_in) {
                                                        l.fade_in = v.max(0.0);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    div { style: "display: flex; flex-direction: column; gap: 2px;",
                                        label { style: "font-size: 9px; color: rgba(255,255,255,0.3);", "Fade Out (s)" }
                                        input {
                                            r#type: "number", step: "0.05", min: "0", class: "glass-input", style: "font-size: 10px; padding: 3px 6px;",
                                            value: "{layer.fade_out}",
                                            oninput: move |evt| {
                                                if let Ok(v) = evt.value().parse::<f64>() {
                                                    if let Some(l) = state.write().layers.iter_mut().find(|l| l.id == id_fade_out) {
                                                        l.fade_out = v.max(0.0);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            // Parent info
                            Section {
                                title: "Hierarchy".to_string(),
                                default_open: false,
                                div { style: "display: flex; flex-direction: column; gap: 2px;",
                                    label { style: "font-size: 10px; color: rgba(255,255,255,0.4);", "Parent Composition / Group" }
                                    select {
                                        class: "glass-input",
                                        style: "font-size: 11px; padding: 4px 8px;",
                                        value: "{layer.parent_id.as_deref().unwrap_or(\"\")}",
                                        onchange: move |evt| {
                                            let val = evt.value();
                                            let new_parent = if val.is_empty() { None } else { Some(val) };
                                            state.write().reparent(&id_reparent, new_parent);
                                        },
                                        option { value: "", "-- Root (No Parent) --" }
                                        {
                                            let id_loop = id_filter.clone();
                                            state.read().layers.iter()
                                                .filter(move |l| l.id != id_loop)
                                                .map(|g| rsx! {
                                                    option { key: "{g.id}", value: "{g.id}", "{g.name}" }
                                                })
                                        }
                                    }
                                }
                            }

                            // Audio Reactivity
                            {
                                let id_react = layer.id.clone();
                                let current_react = format!("{:?}", layer.audio_react);
                                rsx! {
                                    Section {
                                        title: "Audio Reactivity".to_string(),
                                        default_open: false,
                                        div { style: "display: flex; flex-direction: column; gap: 4px;",
                                            label { style: "font-size: 10px; color: rgba(255,255,255,0.4);", "Frequency Band" }
                                            select {
                                                class: "glass-input",
                                                style: "font-size: 11px; padding: 4px 8px;",
                                                value: "{current_react}",
                                                onchange: move |evt| {
                                                    let val = evt.value();
                                                    let band = match val.as_str() {
                                                        "Bass" => AudioBand::Bass,
                                                        "Mid" => AudioBand::Mid,
                                                        "Treble" => AudioBand::Treble,
                                                        _ => AudioBand::None,
                                                    };
                                                    if let Some(l) = state.write().layers.iter_mut().find(|l| l.id == id_react) {
                                                        l.audio_react = band;
                                                    }
                                                },
                                                option { value: "None", "None" }
                                                option { value: "Bass", "🔊 Bass" }
                                                option { value: "Mid", "🎵 Mid" }
                                                option { value: "Treble", "✨ Treble" }
                                            }
                                            div { style: "font-size: 9px; color: rgba(255,255,255,0.25); margin-top: 2px;", "Links layer scale to audio frequency analysis." }
                                        }
                                    }
                                }
                            }

                            // Audio Properties
                            if layer.layer_type == LayerType::Audio {
                                {
                                    let id_audio_url = layer.id.clone();
                                    let id_audio_file = layer.id.clone();
                                    let id_audio_vol = layer.id.clone();
                                    rsx! {
                                        Section {
                                            title: "Audio Source".to_string(),
                                            default_open: true,
                                            div { style: "display: flex; flex-direction: column; gap: 4px;",
                                                label { style: "font-size: 10px; color: rgba(255,255,255,0.4);", "File/URL" }
                                                div { style: "display: flex; gap: 4px;",
                                                    input {
                                                        r#type: "text",
                                                        class: "glass-input",
                                                        style: "font-size: 10px; padding: 4px; flex-grow: 1; box-sizing: border-box;",
                                                        placeholder: "Enter URL...",
                                                        value: "{layer.media_url.as_deref().unwrap_or(\"\")}",
                                                        onchange: move |evt| {
                                                            let val = evt.value();
                                                            if let Some(l) = state.write().layers.iter_mut().find(|l| l.id == id_audio_url) {
                                                                l.media_url = Some(val.clone());
                                                            }
                                                            if let Some(engine) = &mut *audio_ctx.borrow_mut() {
                                                                engine.load_url(&val);
                                                            }
                                                        }
                                                    }
                                                    label {
                                                        style: "cursor: pointer; background: rgba(255,255,255,0.1); border-radius: 4px; padding: 0 8px; display: flex; align-items: center; justify-content: center; font-size: 10px; border: 1px solid rgba(255,255,255,0.05); color: #fff;",
                                                        "Browse"
                                                        input {
                                                            r#type: "file",
                                                            accept: "audio/*",
                                                            style: "display: none;",
                                                            onchange: move |_evt| { 
                                                                // Use a hack to extract the file URL directly via JS, then notify Dioxus
                                                                let id_clone = id_audio_file.clone();
                                                                let script = format!(r#"
                                                                    let input = event.target;
                                                                    if (input.files && input.files[0]) {{
                                                                        let url = URL.createObjectURL(input.files[0]);
                                                                        window.dispatchEvent(new CustomEvent("vibe_audio_uploaded", {{detail: {{id: "{}", url: url}}}}));
                                                                    }}
                                                                "#, id_clone);
                                                                let _ = js_sys::eval(&script);
                                                            },
                                                        }
                                                    }
                                                }
                                            }
                                            div { style: "height: 6px;" }
                                            Slider {
                                                label: "Track Volume".to_string(),
                                                min: 0.0,
                                                max: 1.0,
                                                step: 0.01,
                                                value: layer.opacity as f64, // Using opacity as track volume
                                                on_change: move |v| {
                                                    if let Some(l) = state.write().layers.iter_mut().find(|l| l.id == id_audio_vol) {
                                                        l.opacity = v as f32;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            // Text Properties
                            if layer.layer_type == LayerType::Text {
                                {
                                    let id_txt = layer.id.clone();
                                    let id_sz = layer.id.clone();
                                    let id_col = layer.id.clone();
                                    let id_scol = layer.id.clone();
                                    let id_sw = layer.id.clone();
                                    let id_shc = layer.id.clone();
                                    let id_shb = layer.id.clone();
                                    rsx! {
                                    Section {
                                        title: "Text Properties".to_string(),
                                        default_open: true,
                                        div { style: "display: flex; flex-direction: column; gap: 6px;",
                                            label { style: "font-size: 10px; color: rgba(255,255,255,0.4);", "Text Content" }
                                            input {
                                                r#type: "text",
                                                class: "glass-input",
                                                style: "font-size: 11px; padding: 4px; box-sizing: border-box;",
                                                value: "{layer.text_params.text}",
                                                oninput: move |evt| {
                                                    let val = evt.value();
                                                    if let Some(l) = state.write().layers.iter_mut().find(|l| l.id == id_txt) {
                                                        l.text_params.text = val;
                                                    }
                                                }
                                            }
                                            div { style: "display: flex; gap: 6px;",
                                                div { style: "flex: 1;",
                                                    label { style: "font-size: 10px; color: rgba(255,255,255,0.4);", "Font Size" }
                                                    input {
                                                        r#type: "number",
                                                        class: "glass-input",
                                                        style: "font-size: 11px; padding: 4px; width: 100%; box-sizing: border-box;",
                                                        value: "{layer.text_params.font_size}",
                                                        oninput: move |evt| {
                                                            if let Ok(v) = evt.value().parse::<f32>() {
                                                                if let Some(l) = state.write().layers.iter_mut().find(|l| l.id == id_sz) {
                                                                    l.text_params.font_size = v.max(1.0);
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                div { style: "flex: 1;",
                                                    label { style: "font-size: 10px; color: rgba(255,255,255,0.4);", "Fill Color" }
                                                    div { style: "display: flex; gap: 4px;",
                                                        input {
                                                            r#type: "color",
                                                            style: "width: 20px; height: 20px; padding: 0; border: none; background: none; cursor: pointer;",
                                                            value: "{layer.text_params.color}",
                                                            oninput: move |evt| {
                                                                if let Some(l) = state.write().layers.iter_mut().find(|l| l.id == id_col) {
                                                                    l.text_params.color = evt.value();
                                                                }
                                                            }
                                                        }
                                                        input {
                                                            r#type: "text",
                                                            class: "glass-input",
                                                            style: "font-size: 10px; padding: 2px 4px; flex: 1;",
                                                            value: "{layer.text_params.color}",
                                                        }
                                                    }
                                                }
                                            }
                                            
                                            div { style: "display: flex; gap: 6px; margin-top: 4px;",
                                                div { style: "flex: 1;",
                                                    label { style: "font-size: 10px; color: rgba(255,255,255,0.4);", "Stroke Width" }
                                                    input {
                                                        r#type: "number", step: "0.1", min: "0",
                                                        class: "glass-input",
                                                        style: "font-size: 11px; padding: 4px; width: 100%; box-sizing: border-box;",
                                                        value: "{layer.text_params.stroke_width}",
                                                        oninput: move |evt| {
                                                            if let Ok(v) = evt.value().parse::<f32>() {
                                                                if let Some(l) = state.write().layers.iter_mut().find(|l| l.id == id_sw) {
                                                                    l.text_params.stroke_width = v.max(0.0);
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                div { style: "flex: 1;",
                                                    label { style: "font-size: 10px; color: rgba(255,255,255,0.4);", "Stroke Color" }
                                                    div { style: "display: flex; gap: 4px;",
                                                        input {
                                                            r#type: "color",
                                                            style: "width: 20px; height: 20px; padding: 0; border: none; background: none; cursor: pointer;",
                                                            value: "{layer.text_params.stroke_color}",
                                                            oninput: move |evt| {
                                                                if let Some(l) = state.write().layers.iter_mut().find(|l| l.id == id_scol) {
                                                                    l.text_params.stroke_color = evt.value();
                                                                }
                                                            }
                                                        }
                                                        input {
                                                            r#type: "text",
                                                            class: "glass-input",
                                                            style: "font-size: 10px; padding: 2px 4px; flex: 1;",
                                                            value: "{layer.text_params.stroke_color}",
                                                        }
                                                    }
                                                }
                                            }

                                            div { style: "display: flex; gap: 6px; margin-top: 4px;",
                                                div { style: "flex: 1;",
                                                    label { style: "font-size: 10px; color: rgba(255,255,255,0.4);", "Shadow Blur" }
                                                    input {
                                                        r#type: "number", step: "0.1", min: "0",
                                                        class: "glass-input",
                                                        style: "font-size: 11px; padding: 4px; width: 100%; box-sizing: border-box;",
                                                        value: "{layer.text_params.shadow_blur}",
                                                        oninput: move |evt| {
                                                            if let Ok(v) = evt.value().parse::<f32>() {
                                                                if let Some(l) = state.write().layers.iter_mut().find(|l| l.id == id_shb) {
                                                                    l.text_params.shadow_blur = v.max(0.0);
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                div { style: "flex: 1;",
                                                    label { style: "font-size: 10px; color: rgba(255,255,255,0.4);", "Shadow Color" }
                                                    div { style: "display: flex; gap: 4px;",
                                                        input {
                                                            r#type: "color",
                                                            style: "width: 20px; height: 20px; padding: 0; border: none; background: none; cursor: pointer;",
                                                            value: "{layer.text_params.shadow_color}",
                                                            oninput: move |evt| {
                                                                if let Some(l) = state.write().layers.iter_mut().find(|l| l.id == id_shc) {
                                                                    l.text_params.shadow_color = evt.value();
                                                                }
                                                            }
                                                        }
                                                        input {
                                                            r#type: "text",
                                                            class: "glass-input",
                                                            style: "font-size: 10px; padding: 2px 4px; flex: 1;",
                                                            value: "{layer.text_params.shadow_color}"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                            // Image Properties
                            if layer.layer_type == LayerType::Image {
                                {
                                    let id_img_url = layer.id.clone();
                                    let id_img_file = layer.id.clone();
                                    rsx! {
                                        Section {
                                            title: "Image Source".to_string(),
                                            default_open: true,
                                            div { style: "display: flex; flex-direction: column; gap: 4px;",
                                                label { style: "font-size: 10px; color: rgba(255,255,255,0.4);", "File/URL" }
                                                div { style: "display: flex; gap: 4px;",
                                                    input {
                                                        r#type: "text",
                                                        class: "glass-input",
                                                        style: "font-size: 10px; padding: 4px; flex-grow: 1; box-sizing: border-box;",
                                                        placeholder: "Enter URL...",
                                                        value: "{layer.media_url.as_deref().unwrap_or(\"\")}",
                                                        onchange: move |evt| {
                                                            let val = evt.value();
                                                            if let Some(l) = state.write().layers.iter_mut().find(|l| l.id == id_img_url) {
                                                                l.media_url = Some(val);
                                                            }
                                                        }
                                                    }
                                                    label {
                                                        style: "cursor: pointer; background: rgba(255,255,255,0.1); border-radius: 4px; padding: 0 8px; display: flex; align-items: center; justify-content: center; font-size: 10px; border: 1px solid rgba(255,255,255,0.05); color: #fff;",
                                                        "Browse"
                                                        input {
                                                            r#type: "file",
                                                            accept: "image/*",
                                                            style: "display: none;",
                                                            onchange: move |_evt| {
                                                                let id_clone = id_img_file.clone();
                                                                let script = format!(r#"
                                                                    let input = event.target;
                                                                    if (input.files && input.files[0]) {{
                                                                        let url = URL.createObjectURL(input.files[0]);
                                                                        window.dispatchEvent(new CustomEvent("vibe_image_uploaded", {{detail: {{id: "{}", url: url}}}}));
                                                                    }}
                                                                "#, id_clone);
                                                                let _ = js_sys::eval(&script);
                                                            },
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            // Video Properties
                            if layer.layer_type == LayerType::Video {
                                {
                                    let id_vid_url = layer.id.clone();
                                    let id_vid_file = layer.id.clone();
                                    rsx! {
                                        Section {
                                            title: "Video Source".to_string(),
                                            default_open: true,
                                            div { style: "display: flex; flex-direction: column; gap: 4px;",
                                                label { style: "font-size: 10px; color: rgba(255,255,255,0.4);", "File/URL" }
                                                div { style: "display: flex; gap: 4px;",
                                                    input {
                                                        r#type: "text",
                                                        class: "glass-input",
                                                        style: "font-size: 10px; padding: 4px; flex-grow: 1; box-sizing: border-box;",
                                                        placeholder: "Enter video URL...",
                                                        value: "{layer.media_url.as_deref().unwrap_or(\"\")}",
                                                        onchange: move |evt| {
                                                            let val = evt.value();
                                                            if let Some(l) = state.write().layers.iter_mut().find(|l| l.id == id_vid_url) {
                                                                l.media_url = Some(val);
                                                            }
                                                        }
                                                    }
                                                    label {
                                                        style: "cursor: pointer; background: rgba(255,255,255,0.1); border-radius: 4px; padding: 0 8px; display: flex; align-items: center; justify-content: center; font-size: 10px; border: 1px solid rgba(255,255,255,0.05); color: #fff;",
                                                        "Browse"
                                                        input {
                                                            r#type: "file",
                                                            accept: "video/*",
                                                            style: "display: none;",
                                                            onchange: move |_evt| {
                                                                let id_clone = id_vid_file.clone();
                                                                let script = format!(r#"
                                                                    let input = event.target;
                                                                    if (input.files && input.files[0]) {{
                                                                        let url = URL.createObjectURL(input.files[0]);
                                                                        window.dispatchEvent(new CustomEvent("vibe_image_uploaded", {{detail: {{id: "{}", url: url}}}}));
                                                                    }}
                                                                "#, id_clone);
                                                                let _ = js_sys::eval(&script);
                                                            },
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            // Effect Properties
                            if layer.layer_type == LayerType::Starfield {
                                {
                                    let id_dir = layer.id.clone();
                                    rsx! {
                                        Section {
                                            title: "Effect Settings".to_string(),
                                            default_open: true,
                                            div { style: "display: flex; flex-direction: column; gap: 4px;",
                                                Toggle {
                                                    label: "Retreating (Away)".to_string(),
                                                    checked: layer.effect_params.direction == -1,
                                                    on_change: move |checked| {
                                                        if let Some(l) = state.write().layers.iter_mut().find(|l| l.id == id_dir) {
                                                            l.effect_params.direction = if checked { -1 } else { 1 };
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            // ColorCorrection per-layer parameters
                            if layer.layer_type == LayerType::ColorCorrection {
                                {
                                    let id_hue = layer.id.clone();
                                    let id_sat = layer.id.clone();
                                    let id_con = layer.id.clone();
                                    rsx! {
                                        Section {
                                            title: "Color Settings".to_string(),
                                            default_open: true,
                                            div { style: "display: flex; flex-direction: column; gap: 6px;",
                                                Slider {
                                                    label: "Hue Shift".to_string(),
                                                    min: -180.0,
                                                    max: 180.0,
                                                    step: 1.0,
                                                    value: layer.effect_params.hue_shift as f64,
                                                    on_change: move |v| {
                                                        if let Some(l) = state.write().layers.iter_mut().find(|l| l.id == id_hue) {
                                                            l.effect_params.hue_shift = v as f32;
                                                        }
                                                    }
                                                }
                                                Slider {
                                                    label: "Saturation".to_string(),
                                                    min: 0.0,
                                                    max: 3.0,
                                                    step: 0.05,
                                                    value: layer.effect_params.saturation as f64,
                                                    on_change: move |v| {
                                                        if let Some(l) = state.write().layers.iter_mut().find(|l| l.id == id_sat) {
                                                            l.effect_params.saturation = v as f32;
                                                        }
                                                    }
                                                }
                                                Slider {
                                                    label: "Contrast".to_string(),
                                                    min: 0.0,
                                                    max: 3.0,
                                                    step: 0.05,
                                                    value: layer.effect_params.contrast as f64,
                                                    on_change: move |v| {
                                                        if let Some(l) = state.write().layers.iter_mut().find(|l| l.id == id_con) {
                                                            l.effect_params.contrast = v as f32;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            // Composition Children listing
                            if is_composition {
                                Section {
                                    title: format!("Children ({})", comp_children.len()),
                                    default_open: true,
                                    div { style: "display: flex; flex-direction: column; gap: 2px;",
                                        if comp_children.is_empty() {
                                            div { style: "font-size: 9px; color: rgba(255,255,255,0.25); font-style: italic; padding: 4px 0;", "No children. Add layers inside this composition." }
                                        }
                                        for child in comp_children.iter() {
                                            {
                                                let child_id = child.id.clone();
                                                let child_icon = child.layer_type.icon();
                                                let child_name = child.name.clone();
                                                let child_type = child.layer_type.label();
                                                let vis_icon = if child.visible { "👁" } else { "·" };
                                                rsx! {
                                                    div {
                                                        style: "display: flex; align-items: center; gap: 6px; padding: 3px 4px; border-radius: 3px; cursor: pointer; transition: background 0.15s;",
                                                        onmouseenter: move |evt| {},
                                                        onclick: move |_| {
                                                            state.write().selected_id = Some(child_id.clone());
                                                        },
                                                        span { style: "font-size: 10px; flex-shrink: 0; opacity: 0.4;", "{vis_icon}" }
                                                        span { style: "font-size: 11px; flex-shrink: 0;", "{child_icon}" }
                                                        span { style: "font-size: 10px; color: rgba(255,255,255,0.7); overflow: hidden; text-overflow: ellipsis; white-space: nowrap; flex-grow: 1;", "{child_name}" }
                                                        span { style: "font-size: 8px; color: rgba(255,255,255,0.25); flex-shrink: 0;", "{child_type}" }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                div { 
                    style: "width: 240px; height: 100%; padding: 10px; display: flex; flex-direction: column; align-items: center; justify-content: center; background: #0e0e16; border-left: 1px solid rgba(255,255,255,0.05); flex-shrink: 0;",
                    span { style: "font-size: 24px; opacity: 0.1; margin-bottom: 6px;", "≡" }
                    span { style: "font-size: 11px; color: rgba(255,255,255,0.25);", "Select a layer or effect" }
                }
            }
        }
    }
}
