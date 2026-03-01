use dioxus::prelude::*;
use crate::model::*;

// ─── Single Layer Row ─────────────────────────────────────────────────────────
#[component]
fn LayerRow(
    layer_id: String,
    layer_name: String,
    layer_type: LayerType,
    visible: bool,
    is_selected: bool,
    has_children: bool,
    audio_react: AudioBand,
    depth: usize,
) -> Element {
    let mut state = use_context::<Signal<AppState>>();
    let mut hovered = use_signal(|| false);
    let mut editing = use_signal(|| false);
    let mut edit_buf = use_signal(|| String::new());
    let mut drag_hover_zone = use_signal(|| 0); // 0 = none, 1 = top, 2 = center(parent), 3 = bottom

    let id = layer_id.clone();
    let id2 = layer_id.clone();
    let id3 = layer_id.clone();
    let id4 = layer_id.clone();
    let id5 = layer_id.clone();
    let id6 = layer_id.clone();
    let id7 = layer_id.clone();
    let id8 = layer_id.clone();
    let color = layer_type.color_hex();
    let icon = layer_type.icon();
    let padding_left = format!("{}px", 4 + depth * 16);

    // Drop target handling
    let id_drop = layer_id.clone();
    let id_drag = layer_id.clone();
    let opacity_val = if visible { 1.0 } else { 0.35 };
    let font_weight = if is_selected { 600 } else { 400 };
    let is_dragging = state.read().drag.source_id.is_some();
    let btn_opacity = if visible { "0.8" } else { "0.3" };

    let drop_target_style = if is_dragging { 
        let bd = match *drag_hover_zone.read() {
            1 => "border-top: 2px solid #a78bfa;",
            2 => "background: rgba(167,139,250,0.2);",
            3 => "border-bottom: 2px solid #a78bfa;",
            _ => "",
        };
        format!("pointer-events: auto; {}", bd)
    } else { 
        "pointer-events: none;".to_string() 
    };

    rsx! {
        div {
            class: "layer-row",
            class: if is_selected { "layer-row-selected" } else { "" },
            style: "padding-left: {padding_left}; position: relative; display: flex; align-items: center;",
            onmouseenter: move |_| hovered.set(true),
            onmouseleave: move |_| hovered.set(false),

            // ── Drag handle ──
            div {
                class: "drag-handle",
                style: "display: flex; align-items: center; justify-content: center; cursor: grab; padding: 2px;",
                onpointerdown: move |evt| {
                    evt.stop_propagation();
                    state.write().drag.source_id = Some(id_drag.clone());
                },
                svg { width: "14", height: "14", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                    path { d: "M8 6h13M8 12h13M8 18h13M3 6h.01M3 12h.01M3 18h.01" }
                }
            }

            // ── Visibility toggle (Eye) ──
            button {
                class: "icon-btn",
                style: "display: flex; align-items: center; justify-content: center; background: transparent; border: none; color: inherit; cursor: pointer; padding: 2px; opacity: {btn_opacity};",
                onclick: move |evt| {
                    evt.stop_propagation();
                    state.write().toggle_visibility(&id);
                },
                if visible {
                    svg { width: "14", height: "14", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                        path { d: "M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z" },
                        circle { cx: "12", cy: "12", r: "3" }
                    }
                } else {
                    svg { width: "14", height: "14", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                        path { d: "M17.94 17.94A10.07 10.07 0 0112 20c-7 0-11-8-11-8a18.45 18.45 0 015.06-5.94M9.9 4.24A9.12 9.12 0 0112 4c7 0 11 8 11 8a18.5 18.5 0 01-2.16 3.19m-6.72-1.07a3 3 0 11-4.24-4.24" },
                        line { x1: "1", y1: "1", x2: "23", y2: "23" }
                    }
                }
            }

            // ── Add child button (+) ──
            button {
                class: "icon-btn add-child-btn",
                style: "display: flex; align-items: center; justify-content: center; background: transparent; border: none; color: rgba(255,255,255,0.4); cursor: pointer; padding: 2px; border-radius: 3px; transition: color 0.15s; margin-right: 4px;",
                onclick: move |evt| {
                    evt.stop_propagation();
                    let mut s = state.write();
                    s.add_parent_id = Some(id5.clone());
                    s.show_add_modal = true;
                },
                svg { width: "12", height: "12", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2.5", stroke_linecap: "round", stroke_linejoin: "round",
                    line { x1: "12", y1: "5", x2: "12", y2: "19" },
                    line { x1: "5", y1: "12", x2: "19", y2: "12" }
                }
            }

            div {
                class: "layer-name-area",
                style: "display: flex; align-items: center; gap: 4px; cursor: pointer; flex-grow: 1; min-width: 0; opacity: {opacity_val};",
                onclick: move |evt| {
                    evt.stop_propagation();
                    state.write().selected_id = Some(id2.clone());
                },
                ondoubleclick: move |evt| {
                    evt.stop_propagation();
                    editing.set(true);
                    edit_buf.set(layer_name.clone());
                },
                span {
                    style: "color: {color}; font-size: 11px; flex-shrink: 0;",
                    "{icon}"
                }
                if *editing.read() {
                    input {
                        style: "background: rgba(0,0,0,0.5); border: 1px solid rgba(123,97,255,0.5); color: #fff; font-size: 10px; padding: 2px 4px; border-radius: 2px; outline: none; width: 100%;",
                        autofocus: true,
                        value: "{edit_buf}",
                        oninput: move |evt| edit_buf.set(evt.value().clone()),
                        onkeydown: move |evt| {
                            if evt.key() == Key::Enter {
                                editing.set(false);
                                let name = edit_buf.read().trim().to_string();
                                if !name.is_empty() {
                                    state.write().rename_layer(&id3, name);
                                }
                            } else if evt.key() == Key::Escape {
                                editing.set(false);
                            }
                        },
                        onblur: move |_| {
                            editing.set(false);
                            let name = edit_buf.read().trim().to_string();
                            if !name.is_empty() {
                                state.write().rename_layer(&id4, name);
                            }
                        },
                    }
                } else {
                    span {
                        style: "overflow: hidden; text-overflow: ellipsis; white-space: nowrap; font-size: 10px; color: rgba(255,255,255,0.85); font-weight: {font_weight};",
                        "{layer_name}"
                    }
                }
            }

            // (Audio Reactivity moved to Inspector panel)

            // ── Drop target overlay ──
            {
                let id_drop_move = id_drop.clone();
                rsx! {
                    div {
                        class: "drop-target",
                        style: "{drop_target_style}",
                        onpointermove: move |evt| {
                            if is_dragging && id_drop_move != state.read().drag.source_id.clone().unwrap_or_default() {
                                let y = evt.element_coordinates().y;
                                let h = 28.0; // approximate row height
                                let mut dz = drag_hover_zone.write();
                                if y < h * 0.25 { *dz = 1; }
                                else if y > h * 0.75 { *dz = 3; }
                                else { *dz = 2; }
                            }
                        },
                        onpointerup: move |_| {
                            let mut s = state.write();
                            if let Some(source) = s.drag.source_id.take() {
                                if source != id_drop {
                                    let zone = *drag_hover_zone.read();
                                    if zone == 1 {
                                        s.reorder_layer(&source, &id_drop, true);
                                    } else if zone == 3 {
                                        s.reorder_layer(&source, &id_drop, false);
                                    } else if zone == 2 {
                                        s.reparent(&source, Some(id_drop.clone()));
                                    }
                                }
                            }
                            *drag_hover_zone.write() = 0;
                        },
                        onpointerenter: move |_| {
                            state.write().drag.hover_target_id = Some(id8.clone());
                        },
                        onpointerleave: move |_| {
                            let mut s = state.write();
                            if s.drag.hover_target_id.as_deref() == Some(&id7) {
                                s.drag.hover_target_id = None;
                            }
                            *drag_hover_zone.write() = 0;
                        },
                    }
                }
            }

            // ── Delete Button (hover only, far right) ──
            if *hovered.read() {
                div {
                    style: "margin-left: auto; display: flex; align-items: center; flex-shrink: 0;",
                    button {
                        class: "icon-btn delete-btn",
                        style: "display: flex; align-items: center; justify-content: center; background: transparent; border: none; color: rgba(239,68,68,0.7); cursor: pointer; padding: 2px; border-radius: 3px; transition: color 0.15s;",
                        onclick: move |evt| {
                            evt.stop_propagation();
                            state.write().remove_layer(&id6);
                        },
                        svg { width: "12", height: "12", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2.5", stroke_linecap: "round", stroke_linejoin: "round",
                            path { d: "M3 6h18M19 6v14a2 2 0 01-2 2H7a2 2 0 01-2-2V6m3 0V4a2 2 0 012-2h4a2 2 0 012 2v2M10 11v6M14 11v6" }
                        }
                    }
                }
            }
        }
    }
}

// ─── Recursive Layer Tree ─────────────────────────────────────────────────────
#[component]
pub fn LayerTree(parent_id: Option<String>, depth: usize, exclude_comps: Option<bool>) -> Element {
    let mut state = use_context::<Signal<AppState>>();
    let s = state.read();
    let skip_comps = exclude_comps.unwrap_or(false);

    let children: Vec<Layer> = s.layers.iter()
        .filter(|l| l.parent_id == parent_id)
        .filter(|l| !skip_comps || l.layer_type != LayerType::Composition)
        .cloned()
        .collect();

    if children.is_empty() {
        return rsx! {};
    }

    rsx! {
        for child in children.iter() {
            {
                let child_id = child.id.clone();
                let has_kids = s.layers.iter().any(|l| l.parent_id.as_deref() == Some(&child_id));
                rsx! {
                    div { key: "{child.id}",
                        LayerRow {
                            layer_id: child.id.clone(),
                            layer_name: child.name.clone(),
                            layer_type: child.layer_type,
                            visible: child.visible,
                            is_selected: s.selected_id.as_deref() == Some(&*child.id),
                            has_children: has_kids,
                            audio_react: child.audio_react,
                            depth: depth,
                        }

                        // Recurse into children — unbound layers default collapsed
                        if has_kids {
                            {
                                let toggle_id = child.id.clone();
                                let is_comp_or_ws = child.layer_type == LayerType::Composition || child.layer_type == LayerType::Workstream;
                                let is_expanded = if is_comp_or_ws {
                                    // Compositions/Workstreams use their existing open state
                                    s.is_comp_open(&child.id)
                                } else {
                                    // Other layers default collapsed; must be explicitly opened
                                    s.is_comp_open(&child.id)
                                };
                                rsx! {
                                    if !is_comp_or_ws {
                                        div {
                                            style: "padding-left: {4 + depth * 16}px; cursor: pointer; font-size: 9px; color: rgba(255,255,255,0.35); user-select: none; padding: 1px 0 1px {4 + depth * 16}px;",
                                            onclick: move |_| {
                                                state.write().toggle_comp(&toggle_id);
                                            },
                                            if is_expanded { "▾ children" } else { "▸ children" }
                                        }
                                    }
                                    if is_expanded {
                                        div { class: "nested-children",
                                            LayerTree {
                                                parent_id: Some(child.id.clone()),
                                                depth: depth + 1,
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
}

// ─── Sidebar Component ────────────────────────────────────────────────────────
#[component]
pub fn Sidebar() -> Element {
    let mut state = use_context::<Signal<AppState>>();

    // Collect data before rendering
    let workstreams: Vec<Layer> = {
        let s = state.read();
        s.root_workstreams().into_iter().cloned().collect()
    };

    let has_unbound = {
        let s = state.read();
        !s.unbound_layers().is_empty()
    };

    rsx! {
        div { 
            style: "width: {state.read().left_panel_width}px; display: flex; flex-direction: column; height: 100%; background: #0d0d14; border-right: 1px solid rgba(255,255,255,0.05); z-index: 10; flex-shrink: 0; overflow: hidden;",
            // ── Header ──
            div { 
                style: "padding: 8px 10px; border-bottom: 1px solid rgba(255,255,255,0.05); background: rgba(0,0,0,0.3); display: flex; align-items: center; gap: 6px; flex-shrink: 0; cursor: pointer;",
                onclick: move |_| {
                    state.write().selected_id = Some("__project__".to_string());
                },
                span { 
                    style: "display: flex; align-items: center; color: #7b61ff;",
                    svg { width: "16", height: "16", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                        path { d: "M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1 0-5H20" }
                    }
                }
                span { style: "font-weight: 600; font-size: 11px; letter-spacing: 0.05em; text-transform: uppercase;", "Project" }
            }

            // ── Scrollable content ──
            div { style: "overflow-y: auto; overflow-x: hidden; flex-grow: 1; display: flex; flex-direction: column;",
                // Global Effects
                div { style: "border-bottom: 1px solid rgba(255,255,255,0.05);",
                    div {
                        style: "display: flex; align-items: center; gap: 5px; padding: 6px 8px; cursor: pointer; user-select: none;",
                        onclick: move |_| {
                            state.write().selected_id = Some("__global_effects__".to_string());
                        },
                        span { 
                            style: "display: flex; align-items: center; color: #34d399;",
                            svg { width: "14", height: "14", view_box: "0 0 24 24", fill: "currentColor", stroke: "currentColor", stroke_width: "1", stroke_linecap: "round", stroke_linejoin: "round",
                                path { d: "M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z" }
                            }
                        }
                        span { style: "font-size: 10px; font-weight: 500;", "Global Effects" }
                        span { 
                            style: "margin-left: auto; display: flex; align-items: center; opacity: 0.5; color: rgba(255,255,255,0.5);",
                            svg { width: "12", height: "12", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                                polyline { points: "9 18 15 12 9 6" }
                            }
                        }
                    }
                }

                // ── Workstreams with child Compositions ──
                for ws in workstreams.iter() {
                    {
                        let ws_id = ws.id.clone();
                        let ws_id2 = ws.id.clone();
                        let ws_id3 = ws.id.clone();
                        let ws_id_add = ws.id.clone();
                        let is_ws_open = state.read().is_comp_open(&ws.id);
                        let is_ws_selected = state.read().selected_id.as_deref() == Some(&*ws.id);
                        let ws_bg = if is_ws_selected { "rgba(59,130,246,0.1)" } else { "transparent" };
                        let ws_text = if is_ws_selected { "#3b82f6" } else { "#ffffff" };
                        let is_dragging = state.read().drag.source_id.is_some();

                        // Gather child compositions under this workstream
                        let child_comps: Vec<Layer> = state.read().children_of(&ws.id)
                            .into_iter()
                            .filter(|l| l.layer_type == LayerType::Composition)
                            .cloned()
                            .collect();
                        let child_count = child_comps.len();

                        rsx! {
                            div {
                                style: "border-bottom: 1px solid rgba(255,255,255,0.05);",
                                // Workstream header
                                div {
                                    style: "position: relative; display: flex; align-items: center; gap: 5px; padding: 6px 8px; cursor: pointer; user-select: none; background: {ws_bg};",

                                    div {
                                        style: "display: flex; align-items: center; gap: 5px; flex-grow: 1;",
                                        onclick: move |evt| {
                                            evt.stop_propagation();
                                            state.write().toggle_comp(&ws_id);
                                        },
                                        span {
                                            style: "display: flex; align-items: center; color: rgba(255,255,255,0.5);",
                                            if is_ws_open {
                                                svg { width: "12", height: "12", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", polyline { points: "6 9 12 15 18 9" } }
                                            } else {
                                                svg { width: "12", height: "12", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", polyline { points: "9 18 15 12 9 6" } }
                                            }
                                        }
                                        span {
                                            style: "display: flex; align-items: center; color: #3b82f6;",
                                            svg { width: "14", height: "14", view_box: "0 0 24 24", fill: "currentColor", stroke: "currentColor", stroke_width: "1", stroke_linecap: "round", stroke_linejoin: "round",
                                                path { d: "M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z" }
                                            }
                                        }
                                        span {
                                            style: "font-size: 10px; font-weight: 500; color: {ws_text};",
                                            "{ws.name}"
                                        }
                                        span { style: "font-size: 9px; color: rgba(255,255,255,0.3);", "{child_count}" }
                                    }

                                    button {
                                        style: "background: transparent; border: 1px solid rgba(255,255,255,0.1); display: flex; align-items: center; justify-content: center; color: rgba(255,255,255,0.5); padding: 4px; border-radius: 4px; cursor: pointer; z-index: 10;",
                                        onclick: move |evt| {
                                            evt.stop_propagation();
                                            state.write().selected_id = Some(ws_id2.clone());
                                        },
                                        svg { width: "12", height: "12", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                                            circle { cx: "12", cy: "12", r: "3" },
                                            path { d: "M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z" }
                                        }
                                    }

                                    // Drop target for reparenting INTO this workstream
                                    div {
                                        style: if is_dragging { "position: absolute; inset: 0; z-index: 5; pointer-events: auto;" } else { "position: absolute; inset: 0; z-index: 5; pointer-events: none;" },
                                        onpointerup: move |_| {
                                            let mut s = state.write();
                                            if let Some(source) = s.drag.source_id.take() {
                                                s.reparent(&source, Some(ws_id3.clone()));
                                            }
                                        },
                                    }
                                }

                                // Children: compositions and their children
                                if is_ws_open {
                                    div {
                                        style: "padding-left: 6px; padding-right: 4px; padding-bottom: 6px; border-left: 1px solid rgba(59,130,246,0.15); margin-left: 14px;",

                                        // Show child compositions
                                        for comp in child_comps.iter() {
                                            {
                                                let comp_id = comp.id.clone();
                                                let comp_id2 = comp.id.clone();
                                                let comp_id3 = comp.id.clone();
                                                let comp_id4 = comp.id.clone();
                                                let comp_id5 = comp.id.clone();
                                                let is_open = state.read().is_comp_open(&comp.id);
                                                let is_selected = state.read().selected_id.as_deref() == Some(&*comp.id);
                                                let comp_child_count = state.read().children_of(&comp.id).len();
                                                let bg_color = if is_selected { "rgba(251,191,36,0.1)" } else { "transparent" };
                                                let text_color = if is_selected { "#fbbf24" } else { "#ffffff" };
                                                let is_comp_dragging = state.read().drag.source_id.is_some();

                                                rsx! {
                                                    div {
                                                        style: "border-bottom: 1px solid rgba(255,255,255,0.03);",
                                                        // Composition header
                                                        div {
                                                            style: "position: relative; display: flex; align-items: center; gap: 5px; padding: 5px 6px; cursor: pointer; user-select: none; background: {bg_color};",
                                                            div {
                                                                style: "display: flex; align-items: center; gap: 4px; flex-grow: 1;",
                                                                onclick: move |evt| {
                                                                    evt.stop_propagation();
                                                                    state.write().toggle_comp(&comp_id);
                                                                },
                                                                span {
                                                                    style: "display: flex; align-items: center; color: rgba(255,255,255,0.5);",
                                                                    if is_open {
                                                                        svg { width: "10", height: "10", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", polyline { points: "6 9 12 15 18 9" } }
                                                                    } else {
                                                                        svg { width: "10", height: "10", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", polyline { points: "9 18 15 12 9 6" } }
                                                                    }
                                                                }
                                                                span {
                                                                    style: "display: flex; align-items: center; color: #fbbf24;",
                                                                    svg { width: "12", height: "12", view_box: "0 0 24 24", fill: "currentColor", stroke: "currentColor", stroke_width: "1", stroke_linecap: "round", stroke_linejoin: "round",
                                                                        path { d: "M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z" }
                                                                    }
                                                                }
                                                                span {
                                                                    style: "font-size: 10px; font-weight: 500; color: {text_color};",
                                                                    "{comp.name}"
                                                                }
                                                                span { style: "font-size: 9px; color: rgba(255,255,255,0.3);", "{comp_child_count}" }
                                                            }

                                                            button {
                                                                style: "background: transparent; border: 1px solid rgba(255,255,255,0.1); display: flex; align-items: center; justify-content: center; color: rgba(255,255,255,0.5); padding: 3px; border-radius: 3px; cursor: pointer; z-index: 10;",
                                                                onclick: move |evt| {
                                                                    evt.stop_propagation();
                                                                    state.write().selected_id = Some(comp_id2.clone());
                                                                },
                                                                svg { width: "10", height: "10", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                                                                    circle { cx: "12", cy: "12", r: "3" },
                                                                    path { d: "M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z" }
                                                                }
                                                            }

                                                            // Drop target for reparenting INTO this composition
                                                            div {
                                                                style: if is_comp_dragging { "position: absolute; inset: 0; z-index: 5; pointer-events: auto;" } else { "position: absolute; inset: 0; z-index: 5; pointer-events: none;" },
                                                                onpointerup: move |_| {
                                                                    let mut s = state.write();
                                                                    if let Some(source) = s.drag.source_id.take() {
                                                                        s.reparent(&source, Some(comp_id3.clone()));
                                                                    }
                                                                },
                                                            }
                                                        }

                                                        // Children (recursive)
                                                        if is_open {
                                                            div {
                                                                style: "padding-left: 6px; padding-right: 4px; padding-bottom: 4px; border-left: 1px solid rgba(255,255,255,0.05); margin-left: 14px;",
                                                                LayerTree {
                                                                    parent_id: Some(comp_id4.clone()),
                                                                    depth: 2,
                                                                }

                                                                // Add layer button
                                                                button {
                                                                    style: "margin-top: 4px; width: 100%; font-size: 10px; padding: 5px 0; border: 1px dashed rgba(123,97,255,0.3); border-radius: 4px; background: transparent; color: #7b61ff; cursor: pointer; display: flex; align-items: center; justify-content: center; gap: 4px;",
                                                                    onclick: move |_| {
                                                                        let mut s = state.write();
                                                                        s.add_parent_id = Some(comp_id5.clone());
                                                                        s.show_add_modal = true;
                                                                    },
                                                                    span { style: "font-size: 10px;", "+" }
                                                                    "Add Layer"
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }

                                        // Unbound children of this workstream (non-comp)
                                        LayerTree {
                                            parent_id: Some(ws_id_add.clone()),
                                            depth: 1,
                                            exclude_comps: true,
                                        }

                                        // Add composition button for this workstream
                                        button {
                                            style: "margin-top: 4px; width: 100%; font-size: 10px; padding: 5px 0; border: 1px dashed rgba(251,191,36,0.3); border-radius: 4px; background: transparent; color: #fbbf24; cursor: pointer; display: flex; align-items: center; justify-content: center; gap: 4px;",
                                            onclick: {
                                                let ws_id_for_add = ws.id.clone();
                                                move |_| {
                                                    state.write().add_composition(Some(&ws_id_for_add));
                                                }
                                            },
                                            span { style: "font-size: 10px;", "📁+" }
                                            "Add Composition"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                // ── Unbound Layers ──
                if has_unbound {
                    div { 
                        style: "margin-top: 8px; border-top: 1px solid rgba(255,255,255,0.05);",
                        div { 
                            style: "padding: 6px 8px; font-size: 10px; font-weight: 600; color: rgba(255,255,255,0.5); letter-spacing: 0.05em; position: relative;",
                            // Drop here to unbind a layer
                            div {
                                style: "position: absolute; inset: 0; z-index: 5;",
                                onpointerup: move |_| {
                                    let mut s = state.write();
                                    if let Some(source) = s.drag.source_id.take() {
                                        s.reparent(&source, None);
                                    }
                                },
                            }
                            span { "UNBOUND LAYERS" }
                        }
                        div { 
                            style: "padding-bottom: 6px; padding-left: 6px; padding-right: 4px;",
                            LayerTree {
                                parent_id: None::<String>,
                                depth: 0,
                                exclude_comps: true,
                            }
                        }
                    }
                }

                // ── "Drop here to unbind" zone (fallback empty area) ──
                div {
                    style: "flex-grow: 1; min-height: 40px; position: relative;",
                    onpointerup: move |_| {
                        let mut s = state.write();
                        if let Some(source) = s.drag.source_id.take() {
                            s.reparent(&source, None);
                        }
                    }
                }

                // ── Bottom Add Buttons ──
                div { 
                    style: "padding: 6px 8px; border-top: 1px solid rgba(255,255,255,0.05); display: flex; gap: 4px; flex-shrink: 0;",
                    
                    button {
                        style: "flex: 1; font-size: 9px; padding: 6px 0; border: 1px solid rgba(251,191,36,0.5); border-radius: 4px; background: rgba(251,191,36,0.1); color: #fbbf24; cursor: pointer; display: flex; align-items: center; justify-content: center; gap: 4px; font-weight: 600;",
                        onclick: move |_| {
                            state.write().add_composition(None);
                        },
                        span { style: "font-size: 10px;", "📁+" }
                        "Comp"
                    }
                    button {
                        style: "flex: 1; font-size: 9px; padding: 6px 0; border: 1px solid rgba(59,130,246,0.5); border-radius: 4px; background: rgba(59,130,246,0.1); color: #3b82f6; cursor: pointer; display: flex; align-items: center; justify-content: center; gap: 4px; font-weight: 600;",
                        onclick: move |_| {
                            // Prompt user for workstream duration
                            let dur: f64 = js_sys::eval("parseFloat(prompt('Workstream duration (seconds):', '30')) || 30")
                                .ok()
                                .and_then(|v| v.as_f64())
                                .unwrap_or(30.0);
                            if dur > 0.0 {
                                state.write().add_workstream_with_duration(dur);
                            }
                        },
                        span { style: "font-size: 10px;", "🌊+" }
                        "WS"
                    }
                    button {
                        style: "flex: 1; font-size: 9px; padding: 6px 0; border: 1px dashed rgba(123,97,255,0.3); border-radius: 4px; background: transparent; color: #7b61ff; cursor: pointer; display: flex; align-items: center; justify-content: center; gap: 4px;",
                        onclick: move |_| {
                            let mut s = state.write();
                            s.add_parent_id = None;
                            s.show_add_modal = true;
                        },
                        span { style: "font-size: 10px;", "+" }
                        "Layer"
                    }
                }
            }
        }
    }
}
