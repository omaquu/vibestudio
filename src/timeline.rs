use dioxus::prelude::*;
use crate::model::*;
use std::rc::Rc;
use std::cell::RefCell;

// ─── Main Timeline Component ──────────────────────────────────────────────────
#[component]
pub fn Timeline() -> Element {
    let mut state = use_context::<Signal<AppState>>();
    let audio_ctx = use_context::<Rc<RefCell<Option<crate::audio::AudioEngine>>>>();

    let s = state.read();
    let duration = s.timeline_duration();
    let current_time = s.current_time;
    let is_playing = s.is_playing;
    let zoom = s.timeline_zoom as f64;

    // Gather compositions sorted by start_time
    let mut compositions: Vec<Layer> = s.root_compositions().into_iter().cloned().collect();
    compositions.sort_by(|a, b| a.start_time.partial_cmp(&b.start_time).unwrap_or(std::cmp::Ordering::Equal));

    let unbound: Vec<Layer> = s.unbound_layers().into_iter().cloned().collect();

    // Gather open comp children
    let mut open_comp_children: Vec<(String, Vec<Layer>)> = Vec::new();
    for comp in &compositions {
        if s.is_comp_open(&comp.id) {
            let mut descs: Vec<Layer> = s.descendants_of(&comp.id).into_iter().cloned().collect();
            // Deduplicate by name
            let mut seen = Vec::new();
            descs.retain(|d| {
                if seen.contains(&d.name) { false } else { seen.push(d.name.clone()); true }
            });
            open_comp_children.push((comp.id.clone(), descs));
        }
    }

    let selected_id = s.selected_id.clone();
    drop(s);

    // --- Tick calculation ---
    // We want ticks spaced at least 60px apart visually.
    // total track width ≈ 100% - 80px label col. Approximate as 600px * zoom.
    let track_width_approx = 600.0 * zoom;
    let px_per_sec = if duration > 0.0 { track_width_approx / duration } else { 10.0 };

    // Pick tick interval so ticks are at least 60px apart
    let mut tick_interval = 0.1_f64;
    for candidate in [0.1, 0.25, 0.5, 1.0, 2.0, 5.0, 10.0, 15.0, 30.0, 60.0, 120.0, 300.0] {
        if candidate * px_per_sec >= 60.0 {
            tick_interval = candidate;
            break;
        }
    }

    let num_ticks = ((duration / tick_interval).ceil() as usize).min(500);

    let ticks_elements: Vec<Element> = (0..=num_ticks).filter_map(|i| {
        let t = (i as f64) * tick_interval;
        if t > duration + 0.001 { return None; }
        let pct = if duration > 0.0 { (t / duration) * 100.0 } else { 0.0 };
        let _show_label = true;
        let _tick_height = "6px";
        let m = (t / 60.0).floor() as u32;
        let s_part = t % 60.0;
        let label = if m > 0 {
            format!("{}:{:04.1}", m, s_part)
        } else {
            format!("{:.1}s", t)
        };
        Some(rsx! {
            div {
                key: "tick-{i}",
                style: "position: absolute; left: calc({pct}%); top: 0; bottom: 0; display: flex; flex-direction: column; align-items: flex-start; pointer-events: none;",
                div { style: "font-size: 8px; color: rgba(255,255,255,0.35); padding-left: 2px; line-height: 14px; white-space: nowrap;", "{label}" }
                div { style: "width: 1px; flex-grow: 1; background: rgba(255,255,255,0.08);" }
            }
        })
    }).collect();

    // Format time helper
    let fmt_time = |secs: f64| -> String {
        let m = (secs / 60.0).floor() as u32;
        let s = secs % 60.0;
        format!("{}:{:04.1}", m, s)
    };

    let time_str = fmt_time(current_time);
    let dur_str = fmt_time(duration);
    let playhead_pct = if duration > 0.0 { (current_time / duration) * 100.0 } else { 0.0 };

    rsx! {
        div {
            class: "timeline",
            style: "display: flex; flex-direction: column; height: {state.read().bottom_panel_height}px; flex-shrink: 0; background: #0b0b14; border-top: 1px solid rgba(255,255,255,0.06);",

            // ── Transport Bar ──
            div {
                style: "height: 36px; display: flex; align-items: center; padding: 0 10px; gap: 8px; border-bottom: 1px solid rgba(255,255,255,0.05); flex-shrink: 0; background: #0d0d1a;",

                // Play/Pause
                {
                    let play_audio_ctx = audio_ctx.clone();
                    rsx! {
                        button {
                            style: "width: 26px; height: 26px; border-radius: 50%; background: var(--accent-color); border: none; color: #fff; cursor: pointer; display: flex; align-items: center; justify-content: center; box-shadow: 0 2px 8px rgba(123,97,255,0.35); flex-shrink: 0; font-size: 10px;",
                            onclick: move |_| {
                                let mut s = state.write();
                                let next = !s.is_playing;
                                s.is_playing = next;
                                if let Some(eng) = &*play_audio_ctx.borrow() {
                                    if next { let _ = eng.play(); } else { let _ = eng.pause(); }
                                }
                            },
                            if is_playing { "⏸" } else { "▶" }
                        }
                    }
                }

                // Stop / Rewind
                button {
                    style: "width: 22px; height: 22px; border-radius: 4px; background: rgba(255,255,255,0.05); border: 1px solid rgba(255,255,255,0.1); color: #fff; cursor: pointer; display: flex; align-items: center; justify-content: center; font-size: 10px; flex-shrink: 0;",
                    onclick: move |_| { state.write().seek_to(0.0); state.write().is_playing = false; },
                    "⏹"
                }

                span { style: "font-size: 10px; font-family: monospace; color: rgba(255,255,255,0.6); min-width: 72px;", "{time_str} / {dur_str}" }

                // Zoom controls
                span { style: "font-size: 9px; color: rgba(255,255,255,0.4); margin-left: 8px;", "ZOOM" }
                button {
                    style: "width: 20px; height: 20px; background: rgba(255,255,255,0.05); border: 1px solid rgba(255,255,255,0.1); border-radius: 3px; color: #fff; cursor: pointer; font-size: 12px; display: flex; align-items: center; justify-content: center;",
                    onclick: move |_| {
                        let z = (state.read().timeline_zoom / 1.3).max(0.1);
                        state.write().timeline_zoom = z;
                    },
                    "−"
                }
                span { style: "font-size: 9px; color: rgba(255,255,255,0.5); font-family: monospace; min-width: 30px; text-align: center;", "{zoom:.1}×" }
                button {
                    style: "width: 20px; height: 20px; background: rgba(255,255,255,0.05); border: 1px solid rgba(255,255,255,0.1); border-radius: 3px; color: #fff; cursor: pointer; font-size: 12px; display: flex; align-items: center; justify-content: center;",
                    onclick: move |_| {
                        let z = (state.read().timeline_zoom * 1.3).min(50.0);
                        state.write().timeline_zoom = z;
                    },
                    "+"
                }
                button {
                    style: "margin-left: 6px; font-size: 9px; padding: 3px 8px; background: rgba(123,97,255,0.2); border: 1px solid rgba(123,97,255,0.3); border-radius: 3px; color: #fff; cursor: pointer;",
                    onclick: move |_| { state.write().timeline_zoom = 1.0; },
                    "FIT"
                }
                
                // Loop toggle
                {
                    let loop_on = state.read().loop_playback;
                    let toggle_bg = if loop_on { "rgba(123,97,255,0.2)" } else { "rgba(255,255,255,0.05)" };
                    let toggle_border = if loop_on { "rgba(123,97,255,0.4)" } else { "rgba(255,255,255,0.1)" };
                    rsx! {
                        button {
                            style: "margin-left: 8px; font-size: 11px; padding: 3px 6px; background: {toggle_bg}; border: 1px solid {toggle_border}; border-radius: 3px; color: #fff; cursor: pointer; transition: 0.2s;",
                            onclick: move |_| {
                                let mut s = state.write();
                                s.loop_playback = !s.loop_playback;
                            },
                            "🔁"
                        }
                    }
                }
                
                // Scrollwheel toggle
                {
                    let scroll_zoom = state.read().timeline_scroll_zoom;
                    let toggle_bg = if scroll_zoom { "rgba(123,97,255,0.2)" } else { "rgba(255,255,255,0.05)" };
                    let toggle_border = if scroll_zoom { "rgba(123,97,255,0.4)" } else { "rgba(255,255,255,0.1)" };
                    let toggle_text = if scroll_zoom { "WHEEL: ZOOM" } else { "WHEEL: PAN" };
                    rsx! {
                        button {
                            style: "margin-left: 8px; font-size: 9px; padding: 3px 6px; background: {toggle_bg}; border: 1px solid {toggle_border}; border-radius: 3px; color: #fff; cursor: pointer; transition: 0.2s;",
                            onclick: move |_| {
                                let mut s = state.write();
                                s.timeline_scroll_zoom = !s.timeline_scroll_zoom;
                            },
                            "{toggle_text}"
                        }
                    }
                }

                div { style: "margin-left: auto; display: flex; gap: 6px;",
                    
                    // Volume slider
                    {
                        let vol_audio_ctx = audio_ctx.clone();
                        rsx! {
                            div {
                                style: "display: flex; align-items: center; gap: 6px; margin-right: 12px; background: rgba(255,255,255,0.02); padding: 0 8px; border-radius: 4px; border: 1px solid rgba(255,255,255,0.05);",
                                span { style: "font-size: 9px; color: rgba(255,255,255,0.4);", "VOL" }
                                input {
                                    r#type: "range",
                                    min: "0.0",
                                    max: "1.0",
                                    step: "0.01",
                                    value: "{state.read().master_volume}",
                                    style: "width: 60px; height: 4px; appearance: none; background: rgba(255,255,255,0.1); border-radius: 2px; outline: none; cursor: pointer;",
                                    oninput: move |evt| {
                                        if let Ok(v) = evt.value().parse::<f64>() {
                                            state.write().master_volume = v;
                                            if let Some(eng) = &*vol_audio_ctx.borrow() {
                                                let _ = eng.set_volume(v);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    button {
                        style: "font-size: 9px; padding: 3px 8px; background: rgba(255,255,255,0.05); border: 1px solid rgba(255,255,255,0.1); border-radius: 4px; color: #fff; cursor: pointer;",
                        onclick: move |_| { println!("Load MP3 clicked"); },
                        "↑ Load MP3"
                    }
                    button {
                        style: "font-size: 9px; padding: 4px 12px; background: var(--accent-color); border: none; border-radius: 4px; color: #fff; cursor: pointer; font-weight: 600; letter-spacing: 0.05em;",
                        onclick: move |_| { println!("Render clicked"); },
                        "↓ RENDER"
                    }
                }
            }

            // ── Track Area ──
            div {
                class: "timeline-track-area",
                style: "flex-grow: 1; overflow-x: auto; overflow-y: hidden; position: relative;",
                onwheel: move |evt| {
                    if state.read().timeline_scroll_zoom {
                        // Scroll to zoom
                        let delta = evt.delta().strip_units().y;
                        let z = state.read().timeline_zoom;
                        let new_z = if delta > 0.0 {
                            (z / 1.1).max(0.1)
                        } else {
                            (z * 1.1).min(50.0)
                        };
                        state.write().timeline_zoom = new_z;
                        evt.stop_propagation();
                        // Preventing default here requires raw JS, but Dioxus onwheel covers basics.
                    } else {
                        // Let the browser handle standard horizontal/vertical scrolling.
                    }
                },
                onpointerdown: move |evt| {
                    if evt.trigger_button() == Some(dioxus::html::input_data::MouseButton::Auxiliary) { // Middle click
                        let mut s = state.write();
                        s.is_panning_timeline = true;
                        s.last_pan_x = evt.client_coordinates().x;
                    }
                },
                onpointermove: move |evt| {
                    let mut s = state.write();
                    if s.is_panning_timeline {
                        let cx = evt.client_coordinates().x;
                        let dx = s.last_pan_x - cx;
                        s.last_pan_x = cx;
                        let _ = js_sys::eval(&format!("document.querySelector('.timeline-track-area').scrollBy({}, 0)", dx));
                    }
                    if s.clip_drag.mode.is_some() {
                        let zoom_val = s.timeline_zoom as f64;
                        let pps = 100.0 * zoom_val;
                        s.update_clip_drag(evt.client_coordinates().x, pps);
                    }
                },
                onpointerup: move |_| { 
                    let mut s = state.write();
                    s.is_panning_timeline = false;
                    s.end_clip_drag(); 
                },
                onmouseleave: move |_| { 
                    let mut s = state.write();
                    s.is_panning_timeline = false;
                    s.end_clip_drag(); 
                },

                // Inner container that scales with zoom
                div {
                    style: "width: calc(100% * {zoom}); min-width: calc(100% * {zoom}); height: 100%; display: flex; flex-direction: column; position: relative;",

                    // ── Ruler ──
                    div {
                        style: "height: 22px; flex-shrink: 0; background: #09090f; border-bottom: 1px solid rgba(255,255,255,0.07); position: relative; overflow: hidden;",
                        {ticks_elements.into_iter()}
                        // Scrubbing overlay
                        input {
                            r#type: "range",
                            min: "0.0",
                            max: "{duration}",
                            step: "0.01",
                            value: "{current_time}",
                            style: "position: absolute; inset: 0; width: 100%; height: 100%; opacity: 0; cursor: ew-resize; z-index: 10;",
                            oninput: move |evt| {
                                if let Ok(v) = evt.value().parse::<f64>() {
                                    state.write().seek_to(v);
                                }
                            }
                        }
                        // Playhead Needle
                        div {
                            style: "position: absolute; top: 0; bottom: -2000px; left: {playhead_pct}%; z-index: 15; pointer-events: none;",
                            div { style: "position: absolute; left: -6px; top: 0; width: 0; height: 0; border-left: 6px solid transparent; border-right: 6px solid transparent; border-top: 10px solid #ef4444;" }
                            div { style: "position: absolute; left: -1px; top: 0; width: 2px; height: 100%; background: #ef4444;" }
                        }
                    }

                    // ── Composition Panels (side by side, each takes proportional width) ──
                    div {
                        style: "flex-grow: 1; display: flex; flex-direction: row; overflow: hidden; min-height: 0; position: relative;",

                        for comp in compositions.iter() {
                            {
                                let comp_pct = if duration > 0.0 { (comp.duration / duration) * 100.0 } else { 100.0 / compositions.len().max(1) as f64 };
                                let comp_id_open = comp.id.clone();
                                let comp_id_toggle = comp.id.clone();
                                let comp_color = comp.layer_type.color_hex();
                                let is_open = state.read().is_comp_open(&comp.id);
                                let comp_selected = selected_id.as_deref() == Some(&*comp.id);
                                let comp_border = if comp_selected { format!("2px solid {}", comp_color) } else { "1px solid rgba(255,255,255,0.06)".to_string() };

                                let comp_descendants = open_comp_children.iter()
                                    .find(|(cid, _)| cid == &comp.id)
                                    .map(|(_, d)| d.clone())
                                    .unwrap_or_default();

                                rsx! {
                                    div {
                                        key: "comp-{comp.id}",
                                        style: "width: {comp_pct}%; flex-shrink: 0; display: flex; flex-direction: column; border-right: {comp_border}; background: rgba(255,255,255,0.01); overflow: hidden; min-height: 0;",

                                        // Comp header
                                        div {
                                            style: "height: 22px; flex-shrink: 0; background: rgba(251,191,36,0.07); border-bottom: 1px solid rgba(251,191,36,0.15); display: flex; align-items: center; gap: 4px; padding: 0 6px; cursor: pointer; user-select: none; position: relative;",
                                            onclick: move |_| { state.write().toggle_comp(&comp_id_toggle); },
                                            onpointerdown: move |evt| {
                                                evt.stop_propagation();
                                                let mut s = state.write();
                                                s.selected_id = Some(comp_id_open.clone());
                                                s.begin_clip_drag(&comp_id_open, crate::model::ClipDragMode::Move, evt.client_coordinates().x);
                                            },
                                            span { style: "font-size: 8px; color: rgba(255,255,255,0.4);", if is_open { "▾" } else { "▸" } }
                                            span { style: "font-size: 9px; font-weight: 600; color: #fbbf24; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;", "{comp.name}" }
                                            span { style: "font-size: 7px; color: rgba(255,255,255,0.3); margin-left: auto; white-space: nowrap;", "{comp.duration:.0}s" }
                                            
                                            // Handle Right (Comps can only specify duration, start_time is sequence-driven)
                                            {
                                                let comp_id_resize = comp.id.clone();
                                                rsx! {
                                                    div {
                                                        style: "position: absolute; right: 0; top: 0; bottom: 0; width: 6px; cursor: ew-resize; background: rgba(255,255,255,0.1); z-index: 10;",
                                                        onpointerdown: move |evt| {
                                                            state.write().begin_clip_drag(&comp_id_resize, crate::model::ClipDragMode::TrimRight, evt.client_coordinates().x);
                                                            evt.stop_propagation();
                                                        }
                                                    }
                                                }
                                            }
                                        }

                                        // Inner layer rows (child tracks inside this comp)
                                        div {
                                            style: "flex-grow: 1; overflow-y: auto; display: flex; flex-direction: column;",

                                            if is_open {
                                                if comp_descendants.is_empty() {
                                                    div { style: "font-size: 9px; color: rgba(255,255,255,0.2); padding: 8px 8px; flex-grow: 1; display: flex; align-items: center;", "Empty composition" }
                                                }
                                                for desc in comp_descendants.iter() {
                                                    {
                                                        let desc_color = desc.layer_type.color_hex();
                                                        let desc_selected = selected_id.as_deref() == Some(&*desc.id);
                                                        let bg = if desc_selected { "rgba(123,97,255,0.15)" } else { "transparent" };
                                                        let desc_id_sel = desc.id.clone();
                                                        let desc_id_drag = desc.id.clone();
                                                        // Layer spans full width of comp at proportional horizontal pos
                                                        let layer_pct_left = if comp.duration > 0.0 { ((desc.start_time - comp.start_time).max(0.0) / comp.duration) * 100.0 } else { 0.0 };
                                                        let layer_pct_width = if comp.duration > 0.0 { (desc.duration / comp.duration) * 100.0 } else { 100.0 };
                                                        let layer_pct_width = layer_pct_width.max(2.0);
                                                        rsx! {
                                                            div {
                                                                key: "desc-{desc.id}",
                                                                style: "height: 22px; flex-shrink: 0; position: relative; border-bottom: 1px solid rgba(255,255,255,0.03); background: {bg};",
                                                                onclick: move |_| { state.write().selected_id = Some(desc_id_sel.clone()); },
                                                                // Track label
                                                                div { style: "position: absolute; left: 0; top: 0; bottom: 0; display: flex; align-items: center; padding-left: 6px; font-size: 8px; color: rgba(255,255,255,0.5); pointer-events: none; z-index: 2; background: rgba(11,11,20,0.6); width: 60px; text-overflow: ellipsis; overflow: hidden; white-space: nowrap;",
                                                                    "{desc.name}"
                                                                }
                                                                // Clip bar
                                                                div { style: "position: absolute; left: calc(60px + ({layer_pct_left}% * (100% - 60px) / 100)); width: calc({layer_pct_width}% * (100% - 60px) / 100); top: 3px; bottom: 3px; background: {desc_color}30; border: 1px solid {desc_color}80; border-radius: 2px; overflow: hidden; min-width: 4px; cursor: grab;",
                                                                    onpointerdown: move |evt| {
                                                                        let mut s = state.write();
                                                                        s.selected_id = Some(desc_id_drag.clone());
                                                                        s.begin_clip_drag(&desc_id_drag, crate::model::ClipDragMode::Move, evt.client_coordinates().x);
                                                                        evt.stop_propagation();
                                                                    },
                                                                    div { style: "font-size: 7px; color: {desc_color}; padding: 0 3px; line-height: 14px; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; pointer-events: none;", "{desc.name}" }
                                                                    {
                                                                        let id_l = desc.id.clone();
                                                                        let id_r = desc.id.clone();
                                                                        rsx! {
                                                                            div {
                                                                                style: "position: absolute; left: 0; top: 0; bottom: 0; width: 4px; cursor: ew-resize; background: rgba(255,255,255,0.15); z-index: 10;",
                                                                                onpointerdown: move |evt| {
                                                                                    state.write().begin_clip_drag(&id_l, crate::model::ClipDragMode::TrimLeft, evt.client_coordinates().x);
                                                                                    evt.stop_propagation();
                                                                                }
                                                                            }
                                                                            div {
                                                                                style: "position: absolute; right: 0; top: 0; bottom: 0; width: 4px; cursor: ew-resize; background: rgba(255,255,255,0.15); z-index: 10;",
                                                                                onpointerdown: move |evt| {
                                                                                    state.write().begin_clip_drag(&id_r, crate::model::ClipDragMode::TrimRight, evt.client_coordinates().x);
                                                                                    evt.stop_propagation();
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
                                                div { style: "flex-grow: 1; display: flex; align-items: center; justify-content: center; font-size: 8px; color: rgba(255,255,255,0.15); text-align: center; padding: 4px;",
                                                    "Click ▸ to expand"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // ── Unbound Layers Panel (Bottom Tracks) ──
                    if !unbound.is_empty() {
                        div {
                            style: "flex-shrink: 0; border-top: 1px solid rgba(123,97,255,0.2); background: rgba(123,97,255,0.02); display: flex; flex-direction: column; overflow: hidden; padding: 4px 0;",
                            for layer in unbound.iter() {
                                {
                                    let layer_color = layer.layer_type.color_hex();
                                    let layer_selected = selected_id.as_deref() == Some(&*layer.id);
                                    let bg = if layer_selected { "rgba(123,97,255,0.15)" } else { "transparent" };
                                    let layer_id_sel = layer.id.clone();
                                    let layer_id_drag = layer.id.clone();
                                    
                                    // Calculate span across entire global duration
                                    let layer_pct_left = if duration > 0.0 { ((layer.start_time).max(0.0) / duration) * 100.0 } else { 0.0 };
                                    let layer_pct_width = if duration > 0.0 { (layer.duration / duration) * 100.0 } else { 100.0 };
                                    let layer_pct_width = layer_pct_width.max(0.5);

                                    rsx! {
                                        div {
                                            key: "unbound-{layer.id}",
                                            style: "height: 24px; flex-shrink: 0; display: flex; position: relative; border-bottom: 1px solid rgba(255,255,255,0.03); background: {bg}; cursor: pointer;",
                                            onclick: move |_| { state.write().selected_id = Some(layer_id_sel.clone()); },
                                            
                                            // Left Sticky Label
                                            div { style: "position: absolute; left: 0; top: 0; bottom: 0; display: flex; align-items: center; padding-left: 8px; font-size: 9px; color: rgba(255,255,255,0.7); pointer-events: none; z-index: 2; background: rgba(11,11,20,0.8); width: 100px; text-overflow: ellipsis; overflow: hidden; white-space: nowrap; border-right: 1px solid rgba(255,255,255,0.05);",
                                                div { style: "width: 6px; height: 6px; border-radius: 50%; background: {layer_color}; margin-right: 6px; flex-shrink: 0;" }
                                                "{layer.name}"
                                            }

                                            // Track Bar
                                            div { style: "position: absolute; left: calc(100px + ({layer_pct_left}% * (100% - 100px) / 100)); width: calc({layer_pct_width}% * (100% - 100px) / 100); top: 4px; bottom: 4px; background: {layer_color}40; border: 1px solid {layer_color}80; border-radius: 2px; overflow: hidden; min-width: 4px; cursor: grab;",
                                                onpointerdown: move |evt| {
                                                    let mut s = state.write();
                                                    s.selected_id = Some(layer_id_drag.clone());
                                                    s.begin_clip_drag(&layer_id_drag, crate::model::ClipDragMode::Move, evt.client_coordinates().x);
                                                    evt.stop_propagation();
                                                },
                                                div { style: "font-size: 8px; color: #fff; padding: 0 4px; line-height: 14px; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; pointer-events: none;", "{layer.name} ({layer.duration:.0}s)" }
                                                {
                                                    let id_l = layer.id.clone();
                                                    let id_r = layer.id.clone();
                                                    rsx! {
                                                        div {
                                                            style: "position: absolute; left: 0; top: 0; bottom: 0; width: 6px; cursor: ew-resize; background: rgba(255,255,255,0.15); z-index: 10;",
                                                            onpointerdown: move |evt| {
                                                                state.write().begin_clip_drag(&id_l, crate::model::ClipDragMode::TrimLeft, evt.client_coordinates().x);
                                                                evt.stop_propagation();
                                                            }
                                                        }
                                                        div {
                                                            style: "position: absolute; right: 0; top: 0; bottom: 0; width: 6px; cursor: ew-resize; background: rgba(255,255,255,0.15); z-index: 10;",
                                                            onpointerdown: move |evt| {
                                                                state.write().begin_clip_drag(&id_r, crate::model::ClipDragMode::TrimRight, evt.client_coordinates().x);
                                                                evt.stop_propagation();
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
                    }

                    // Playhead vertical line over the track area
                    div {
                        style: "position: absolute; top: 0; bottom: 0; left: {playhead_pct}%; width: 1px; background: rgba(255,255,255,0.8); z-index: 10; pointer-events: none;",
                    }
                }
            }
        }
    }
}
