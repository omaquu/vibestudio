mod model;
mod sidebar;
mod timeline;
mod inspector;
mod audio;

use dioxus::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;
use gloo_timers::future::sleep;
use model::*;
use timeline::Timeline;
use inspector::Inspector;
use sidebar::Sidebar;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsValue;
use wasm_bindgen::JsCast;

fn main() {
    dioxus::launch(App);
}

// ─── Add Item Modal (overlays canvas preview) ─────────────────────────────────
#[component]
fn AddItemModal() -> Element {
    let mut state = use_context::<Signal<AppState>>();
    let mut search = use_signal(String::new);
    let mut hovered = use_signal(|| None::<LayerType>);
    let parent_id = state.read().add_parent_id.clone();
    let parent_name = parent_id.as_ref()
        .and_then(|pid| state.read().layers.iter().find(|l| l.id == *pid).map(|l| l.name.clone()))
        .unwrap_or_else(|| "Root (Unbound)".to_string());

    let search_str = search.read().to_lowercase();
    let all_types = LayerType::addable_types();
    let filtered: Vec<LayerType> = all_types.iter().copied().filter(|lt| {
        search_str.is_empty() || lt.label().to_lowercase().contains(&search_str)
    }).collect();

    rsx! {
        div {
            style: "position: absolute; inset: 0; z-index: 50; display: flex; align-items: center; justify-content: center; background: rgba(0,0,0,0.7); backdrop-filter: blur(10px);",
            onclick: move |_| { state.write().show_add_modal = false; search.set(String::new()); },

            div {
                style: "background: #12121e; border: 1px solid rgba(255,255,255,0.1); border-radius: 16px; width: 780px; height: 500px; display: flex; box-shadow: 0 32px 96px rgba(0,0,0,0.95); overflow: hidden;",
                onclick: move |evt| evt.stop_propagation(),

                // Left column: Grid
                div {
                    style: "flex: 2; display: flex; flex-direction: column; border-right: 1px solid rgba(255,255,255,0.08);",
                    // Header
                    div {
                        style: "padding: 20px 22px 14px; border-bottom: 1px solid rgba(255,255,255,0.07); flex-shrink: 0;",
                        div { style: "display: flex; justify-content: space-between; align-items: flex-start; margin-bottom: 14px;",
                            div {
                                h2 { style: "font-size: 17px; font-weight: 700; color: #fff; margin: 0; letter-spacing: -0.02em;", "Add Effect Layer" }
                                p { style: "font-size: 11px; color: rgba(255,255,255,0.35); margin: 5px 0 0;", "Parent: {parent_name}" }
                            }
                            button {
                                style: "background: rgba(255,255,255,0.07); border: 1px solid rgba(255,255,255,0.12); color: rgba(255,255,255,0.5); width: 30px; height: 30px; border-radius: 50%; cursor: pointer; font-size: 14px; display: flex; align-items: center; justify-content: center;",
                                onclick: move |_| { state.write().show_add_modal = false; search.set(String::new()); },
                                "✕"
                            }
                        }
                        div { style: "position: relative;",
                            input {
                                style: "width: 100%; box-sizing: border-box; background: rgba(255,255,255,0.05); border: 1px solid rgba(255,255,255,0.1); border-radius: 10px; padding: 9px 14px 9px 38px; color: #fff; font-size: 13px; outline: none;",
                                placeholder: "Search effects…",
                                value: "{search}",
                                oninput: move |evt| search.set(evt.value().clone()),
                            }
                            span { style: "position: absolute; left: 12px; top: 50%; transform: translateY(-50%); font-size: 13px; pointer-events: none; opacity: 0.35;", "🔍" }
                        }
                    }

                    // Effect grid
                    div {
                        style: "flex-grow: 1; min-height: 0; overflow-y: auto; padding: 16px; display: grid; grid-template-columns: repeat(3, 1fr); gap: 10px;",
                        if filtered.is_empty() {
                            div { style: "grid-column: 1/-1; text-align: center; padding: 32px; color: rgba(255,255,255,0.25); font-size: 13px;",
                                "No effects match your search"
                            }
                        }
                        for lt in filtered.iter() {
                            {
                                let layer_type = *lt;
                                let pid = parent_id.clone();
                                let col = layer_type.color_hex();
                                rsx! {
                                    button {
                                        key: "{layer_type.label()}",
                                        style: "background: rgba(255,255,255,0.025); border: 1px solid {col}22; border-radius: 12px; padding: 14px 10px 12px; cursor: pointer; display: flex; flex-direction: column; align-items: center; gap: 9px; transition: all 0.15s ease; text-align: center;",
                                        onmouseenter: move |_| hovered.set(Some(layer_type)),
                                        onmouseleave: move |_| hovered.set(None),
                                        onclick: move |_| {
                                            let mut s = state.write();
                                            s.add_layer(Layer::new(layer_type, pid.clone()));
                                            s.show_add_modal = false;
                                            drop(s);
                                            search.set(String::new());
                                        },
                                        div { style: "width: 46px; height: 30px; border-radius: 8px; background: {col}18; border: 1px solid {col}40; display: flex; align-items: center; justify-content: center; font-size: 18px;",
                                            "{layer_type.icon()}"
                                        }
                                        span { style: "font-size: 11px; font-weight: 500; color: rgba(255,255,255,0.85); line-height: 1.3;", "{layer_type.label()}" }
                                    }
                                }
                            }
                        }
                    }
                }
                
                // Right column: Preview Area
                div {
                    style: "flex: 1; background: rgba(0,0,0,0.2); display: flex; flex-direction: column; justify-content: center; align-items: center; padding: 24px; text-align: center;",
                    if let Some(lt) = hovered() {
                        div {
                            style: "display: flex; flex-direction: column; align-items: center; gap: 12px; animation: 0.2s ease-in fade-in;",
                            // Live mini-canvas preview
                            canvas {
                                id: "vibe-effect-preview-canvas",
                                width: "200",
                                height: "140",
                                style: "width: 200px; height: 140px; border-radius: 12px; background: #08080f; border: 1px solid {lt.color_hex()}44;",
                                "data-effect-type": "{lt:?}",
                                "data-effect-color": "{lt.color_hex()}",
                            }
                            h3 { style: "color: #fff; margin: 0; font-size: 16px; font-weight: 600;", "{lt.label()}" }
                            p { style: "color: rgba(255,255,255,0.6); font-size: 12px; line-height: 1.4; margin: 0;", "{lt.description()}" }
                        }
                    } else {
                        div {
                            style: "opacity: 0.3; display: flex; flex-direction: column; align-items: center; gap: 12px;",
                            span { style: "font-size: 32px;", "✨" }
                            p { style: "font-size: 12px; margin: 0;", "Hover over an effect to preview" }
                        }
                    }
                }
            }
        }
    }
}

// ─── Canvas Preview Area ───────────────────────────────────────────────────────
#[component]
fn CanvasArea() -> Element {
    let mut state = use_context::<Signal<AppState>>();
    let audio_ctx = use_context::<Rc<RefCell<Option<audio::AudioEngine>>>>();
    let show_modal = state.read().show_add_modal;

    // Boot the JS canvas renderer once on first render
    use_effect(|| {
        let _ = js_sys::eval(r#"
(function(){
  if(window.__vibeRendererStarted) return;
  window.__vibeRendererStarted = true;
  window.__vibeLayers = [];
  window.__vibeTime = 0;

  window.__vibeAnalyser = null;
  window.__vibeFreq = { bass: 0, mid: 0, treble: 0 };

  function setupAnalyser() {
    const audio = document.getElementById('vibe-master-audio');
    if (!audio || !audio.src) return;
    if (window.__vibeAudioSrc) {
        if (window.__vibeAudioCtx && window.__vibeAudioCtx.state === 'suspended') {
            window.__vibeAudioCtx.resume();
        }
        return;
    }
    const ctx = new AudioContext();
    const src = ctx.createMediaElementSource(audio);
    const analyser = ctx.createAnalyser();
    analyser.fftSize = 256;
    src.connect(analyser);
    analyser.connect(ctx.destination);
    window.__vibeAudioCtx = ctx;
    window.__vibeAnalyser = analyser;
    window.__vibeAudioSrc = src;
    
    // Resume context on any user click if it's blocked by autoplay policy
    window.addEventListener('click', () => { 
        if(window.__vibeAudioCtx.state === 'suspended') window.__vibeAudioCtx.resume(); 
    });
  }

  function drawLayer(ctx, l, t, W, H) {
    const cx = W/2 + (l._abs_x||0)*W/200;
    const cy = H/2 + (l._abs_y||0)*H/200;
    let reactMult = 1.0;
    
    let fadeMult = 1.0;
    if (l.start_time !== undefined && l.duration) {
        const local_t = t - l.start_time;
        if (l.fade_in > 0 && local_t < l.fade_in) {
            fadeMult = Math.max(0, local_t / l.fade_in);
        } else if (l.fade_out > 0 && local_t > l.duration - l.fade_out) {
            fadeMult = Math.max(0, 1.0 - ((local_t - (l.duration - l.fade_out)) / l.fade_out));
        }
    }
    
    if (l.audio_react === 'Bass') reactMult = 1.0 + window.__vibeFreq.bass * 0.8;
    if (l.audio_react === 'Mid') reactMult = 1.0 + window.__vibeFreq.mid * 0.8;
    if (l.audio_react === 'Treble') reactMult = 1.0 + window.__vibeFreq.treble * 0.8;
    const sc = (l._abs_sc || 1) * reactMult;
    ctx.globalAlpha = (l._abs_op != null ? l._abs_op : 1.0) * fadeMult;
    const c = l.color || '#7b61ff';
    
    ctx.save();
    ctx.translate(cx, cy);
    if(l._abs_rot) ctx.rotate(l._abs_rot * Math.PI / 180.0);
    
    // Simulate perspective using scaling and skew approximation
    let px = l.perspective ? l.perspective[0] || 0 : 0;
    let py = l.perspective ? l.perspective[1] || 0 : 0;
    if (px !== 0 || py !== 0) {
       // Rough approximation for perspective effect
       let scaleX = 1 - Math.abs(px) * 0.05;
       let scaleY = 1 - Math.abs(py) * 0.05;
       ctx.scale(Math.max(scaleX, 0.1), Math.max(scaleY, 0.1));
       ctx.transform(1, py * 0.02, px * 0.02, 1, 0, 0);
    }
    
    // Flip X / Y
    let fx = l.flip_x ? -1 : 1;
    let fy = l.flip_y ? -1 : 1;
    if (fx < 0 || fy < 0) {
       ctx.scale(fx, fy);
    }

    if(l.skew_x || l.skew_y) {
      ctx.transform(1, Math.tan((l.skew_y||0)*Math.PI/180), Math.tan((l.skew_x||0)*Math.PI/180), 1, 0, 0);
    }
    ctx.translate(-cx, -cy);
    
    switch(l.type) {
      case 'Image':
      case 'Video': {
        // Render media via cached HTMLImageElement / HTMLVideoElement
        if(!window.__vibeMediaCache) window.__vibeMediaCache = {};
        const url = l.media_url;
        if(url) {
            if(!window.__vibeMediaCache[url]) {
                if(l.type === 'Video') {
                    const vid = document.createElement('video');
                    vid.src = url; vid.crossOrigin = 'anonymous'; vid.muted = false; vid.loop = true;
                    vid.play().catch(()=>{});
                    window.__vibeMediaCache[url] = vid;
                } else {
                    const img = new Image();
                    img.src = url; img.crossOrigin = 'anonymous';
                    window.__vibeMediaCache[url] = img;
                }
            }
            const media = window.__vibeMediaCache[url];
            if(media && (media.complete || media.readyState >= 2)) {
                const mw = media.videoWidth || media.naturalWidth || media.width || 200;
                const mh = media.videoHeight || media.naturalHeight || media.height || 150;
                const aspect = mw / mh;
                let dw = W * sc * 0.8;
                let dh = dw / aspect;
                if(dh > H * 0.9) { dh = H * 0.9; dw = dh * aspect; }
                ctx.drawImage(media, cx - dw/2, cy - dh/2, dw, dh);
            }
        } else {
            // No media loaded yet — show placeholder
            ctx.fillStyle = c + '30';
            ctx.fillRect(cx - 60*sc, cy - 40*sc, 120*sc, 80*sc);
            ctx.strokeStyle = c; ctx.lineWidth = 2;
            ctx.strokeRect(cx - 60*sc, cy - 40*sc, 120*sc, 80*sc);
            ctx.fillStyle = c;
            ctx.font = `${Math.round(10*sc)}px system-ui`;
            ctx.textAlign = 'center';
            ctx.fillText(l.type === 'Image' ? '🖼 Image' : '🎬 Video', cx, cy);
        }
        break;
      }
      case 'SpectrumCircle': {
        const r = 70*sc*reactMult, bars = 64;
        for(let i=0;i<bars;i++){
          const a = (i/bars)*Math.PI*2;
          const jump = 0.4 + 0.6 * Math.abs(Math.sin(t*1.5+i*0.25)) * reactMult;
          const len = r * jump;
          ctx.beginPath(); ctx.moveTo(cx+Math.cos(a)*r, cy+Math.sin(a)*r);
          ctx.lineTo(cx+Math.cos(a)*(r+len*0.5), cy+Math.sin(a)*(r+len*0.5));
          ctx.strokeStyle=c; ctx.lineWidth=2 * reactMult; ctx.stroke();
        }
        ctx.beginPath(); ctx.arc(cx,cy,r*0.25,0,Math.PI*2);
        ctx.fillStyle=c+'55'; ctx.fill(); break;
      }
      case 'SpectrumMountain': {
        ctx.beginPath(); ctx.moveTo(0,H);
        for(let i=0;i<=100;i++){
          const x=(i/100)*W;
          const y=cy+40*sc*reactMult*Math.sin(i*0.18+t*1.4)*Math.abs(Math.sin(i*0.05+t*0.3));
          ctx.lineTo(x,y);
        }
        ctx.lineTo(W,H); ctx.closePath();
        ctx.fillStyle=c+'50'; ctx.fill();
        ctx.strokeStyle=c; ctx.lineWidth=1.5; ctx.stroke(); break;
      }
      case 'Particles': {
        for(let i=0;i<50;i++){
          const a=i*2.4+t*0.8; const r=(15+i*2.8)*sc*reactMult;
          ctx.beginPath(); ctx.arc(cx+Math.cos(a)*r, cy+Math.sin(a)*r, (2+Math.sin(t+i)*1.2)*reactMult, 0, Math.PI*2);
          ctx.fillStyle=c; ctx.fill();
        } break;
      }
      case 'ParticleRings': {
        for(let ring=0;ring<4;ring++){
          const r=(25+ring*22)*sc*reactMult;
          ctx.beginPath(); ctx.arc(cx,cy,r,0,Math.PI*2);
          ctx.strokeStyle=c+(ring%2?'55':'33'); ctx.lineWidth=(1+ring*0.5)*reactMult; ctx.stroke();
          for(let i=0;i<8;i++){
            const a=i*Math.PI/4+t*(ring%2?0.8:-0.8);
            ctx.beginPath(); ctx.arc(cx+Math.cos(a)*r, cy+Math.sin(a)*r, 2.5*reactMult, 0, Math.PI*2);
            ctx.fillStyle=c; ctx.fill();
          }
        } break;
      }
      case 'Starfield': {
        let dir = l.dir !== undefined ? l.dir : 1.0;
        for(let i=0;i<120;i++){
          const a=i*137.508; 
          let dist = (Math.sqrt(i/120) - (t * 0.1 * dir)) % 1.0;
          if(dist < 0) dist += 1.0;
          const r=dist*Math.min(W,H)*0.8*sc*reactMult;
          if (r < 0.1) continue;
          const b = Math.min(dist * 3.0, 1.0); // fade in from center
          ctx.beginPath(); ctx.arc(cx+Math.cos(a)*r, cy+Math.sin(a)*r, (0.5+dist*3*sc)*reactMult, 0, Math.PI*2);
          ctx.fillStyle=`rgba(255,255,255,${b})`; ctx.fill();
        } break;
      }
      case 'Tunnel': {
        for(let i=9;i>0;i--){
          const r=(i*20*sc+(t*25)%20)*reactMult;
          ctx.beginPath(); ctx.arc(cx,cy,r,0,Math.PI*2);
          ctx.strokeStyle=c+Math.floor((i/9)*200).toString(16).padStart(2,'0');
          ctx.lineWidth=3 * reactMult; ctx.stroke();
        } break;
      }
      case 'Kaleidoscope': {
        for(let s=0;s<8;s++){
          ctx.save(); ctx.translate(cx,cy); ctx.rotate(s*Math.PI/4+t*0.25);
          ctx.beginPath(); ctx.moveTo(0,0); ctx.lineTo(65*sc,18*sc*Math.sin(t+s)); ctx.lineTo(80*sc,-12*sc); ctx.closePath();
          ctx.fillStyle=c+'55'; ctx.fill(); ctx.restore();
        } break;
      }
      case 'Laser': {
        for(let i=0;i<6;i++){
          const a=i*Math.PI/6+t*0.4;
          ctx.beginPath(); ctx.moveTo(cx,cy); ctx.lineTo(cx+Math.cos(a)*W*0.55*sc, cy+Math.sin(a)*H*0.55*sc);
          ctx.strokeStyle=c+'cc'; ctx.lineWidth=1.5; ctx.stroke();
        } break;
      }
      case 'Glitch': {
        for(let i=0;i<8;i++){
          const y=cy-50*sc+i*13*sc; const w=70*sc*(0.6+0.4*Math.random());
          const off=Math.sin(t*12+i)*12;
          ctx.fillStyle=c+Math.floor(180+Math.random()*75).toString(16);
          ctx.fillRect(cx-w/2+off, y, w, 7*sc);
        } break;
      }
      case 'Waveform': {
        ctx.beginPath();
        const tDom = window.__vibeTimeDomain || [];
        const len = tDom.length ? tDom.length : 120;
        for(let i=0;i<len;i++){
          const x=(i/(len-1||1))*W;
          const val = tDom.length ? ((tDom[i] - 128) / 128) : Math.sin((i*0.18+t)*2+Math.sin(i*0.05+t)*1.5);
          const y=cy+35*sc*val;
          i===0?ctx.moveTo(x,y):ctx.lineTo(x,y);
        }
        ctx.strokeStyle=c; ctx.lineWidth=2; ctx.stroke(); break;
      }
      case 'Text': {
        const textStr = l.text_str || 'TEXT';
        ctx.font=`bold ${Math.round((l.text_size || 48)*sc)}px system-ui`; ctx.textAlign='center'; ctx.textBaseline='middle';
        ctx.shadowColor = l.text_shadow || 'rgba(0,0,0,0.8)';
        ctx.shadowBlur = (l.text_shadow_b !== undefined ? l.text_shadow_b : 12) * sc;
        ctx.shadowOffsetX = 4*sc; ctx.shadowOffsetY = 4*sc;
        
        ctx.lineWidth = (l.text_stroke_w !== undefined ? l.text_stroke_w : 3) * sc;
        ctx.strokeStyle = l.text_stroke || '#000000';
        ctx.strokeText(textStr,cx,cy);
        
        ctx.fillStyle = l.text_color || '#ffffff';
        ctx.fillText(textStr,cx,cy);
        
        ctx.shadowBlur=0; ctx.shadowOffsetX=0; ctx.shadowOffsetY=0;
        break;
      }
      case 'ChromaticAberration': {
        const amt = (l._abs_op || 1) * 6 * sc;
        if(amt > 0.1 && window.__vibeOffscreenCanvas) {
           const size = 150 * sc;
           const sx = Math.max(0, cx - size);
           const sy = Math.max(0, cy - size);
           const sw = Math.min(W - sx, size * 2);
           const sh = Math.min(H - sy, size * 2);
           
           if (sw > 0 && sh > 0) {
               const cvs = ctx.canvas;
               const octx = window.__vibeOffscreenCanvas.getContext('2d');
               
               // Clear and copy only the localized region
               octx.clearRect(sx, sy, sw, sh); 
               octx.drawImage(cvs, sx, sy, sw, sh, sx, sy, sw, sh);
               
               ctx.globalCompositeOperation = 'screen';
               ctx.globalAlpha = 0.5;
               
               // Draw the offset snippets
               ctx.drawImage(window.__vibeOffscreenCanvas, sx, sy, sw, sh, sx + amt, sy, sw, sh);
               ctx.drawImage(window.__vibeOffscreenCanvas, sx, sy, sw, sh, sx - amt, sy, sw, sh);
               
               ctx.globalAlpha = 1.0;
               ctx.globalCompositeOperation = 'source-over';
           }
        } else {
           // Fallback for mini-preview: draw three offset colored circles (R/G/B)
           const r2 = 30 * sc;
           const off = 8 * sc;
           ctx.globalAlpha = 0.7;
           ctx.beginPath(); ctx.arc(cx - off, cy, r2, 0, Math.PI*2); ctx.fillStyle='rgba(255,0,0,0.6)'; ctx.fill();
           ctx.beginPath(); ctx.arc(cx, cy, r2, 0, Math.PI*2); ctx.fillStyle='rgba(0,255,0,0.6)'; ctx.fill();
           ctx.beginPath(); ctx.arc(cx + off, cy, r2, 0, Math.PI*2); ctx.fillStyle='rgba(0,80,255,0.6)'; ctx.fill();
           ctx.globalAlpha = 1.0;
           ctx.font = `${Math.round(9*sc)}px system-ui`; ctx.textAlign='center'; ctx.fillStyle='#fff';
           ctx.fillText('RGB SPLIT', cx, cy + r2 + 14*sc);
        }
        break;
      }
      case 'ColorCorrection': {
        const size = 150 * sc;
        const sx = Math.max(0, cx - size);
        const sy = Math.max(0, cy - size);
        const sw = Math.min(W - sx, size * 2);
        const sh = Math.min(H - sy, size * 2);
        // Draw a visible color wheel gradient swatch as representative visual
        const grad = ctx.createLinearGradient(sx, sy, sx + sw, sy + sh);
        grad.addColorStop(0, 'rgba(255,80,80,0.55)');
        grad.addColorStop(0.33, 'rgba(80,255,120,0.55)');
        grad.addColorStop(0.66, 'rgba(80,120,255,0.55)');
        grad.addColorStop(1, 'rgba(255,220,80,0.55)');
        ctx.globalCompositeOperation = 'overlay';
        ctx.fillStyle = grad;
        ctx.fillRect(sx, sy, sw, sh);
        ctx.globalCompositeOperation = 'source-over';
        // Add label in mini-preview context
        ctx.font = `${Math.round(9*sc)}px system-ui`; ctx.textAlign='center'; ctx.fillStyle='rgba(255,255,255,0.7)';
        ctx.fillText('COLOR GRADE', cx, cy + size * 0.7 + 14*sc);
        break;
      }
      case 'FilmGrain': {
        const amt = l._abs_op || 1;
        ctx.fillStyle = `rgba(255,255,255,${amt*0.06})`;
        const size = 150 * sc;
        const sx = Math.max(0, cx - size);
        const sy = Math.max(0, cy - size);
        const sw = Math.min(W - sx, size * 2);
        const sh = Math.min(H - sy, size * 2);
        // Only sprinkle grain in the local area to avoid massive loop lag over W*H
        for(let i=0; i<sw*sh*0.001*amt; i++) ctx.fillRect(sx + Math.random()*sw, sy + Math.random()*sh, 2, 2);
        break;
      }
      case 'VhsEffect': {
        const amt = l._abs_op || 1;
        ctx.fillStyle = `rgba(0,0,0,${amt*0.12})`;
        const size = 150 * sc;
        const sy = Math.max(0, cy - size);
        const sh = Math.min(H - sy, size * 2);
        // Localized VHS bands
        ctx.fillRect(0, sy + Math.random()*sh, W, 4 + Math.random()*10);
        ctx.fillStyle = `rgba(255,255,255,${amt*0.06})`;
        ctx.fillRect(0, sy + Math.random()*sh, W, 2);
        break;
      }
      case 'GlitchPost': {
        const amt = l._abs_op || 1;
        if(amt > 0.1) {
            const size = 150 * sc;
            const sx = Math.max(0, cx - size);
            const sy = Math.max(0, cy - size);
            const sw = Math.min(W - sx, size * 2);
            const sh = Math.min(H - sy, size * 2);
            if(sw > 0 && sh > 0) {
                // Try to shift from canvas content; also draw visible glitch bars as fallback
                try { ctx.drawImage(ctx.canvas, sx, sy + Math.random()*sh*0.5, sw, sh*0.3, sx + (Math.random()-0.5)*40*amt, sy + Math.random()*sh*0.5, sw, sh*0.3); } catch(e){}
                // Always draw glitch bar pattern so mini-preview is visible
                const barColors = [c+'cc', '#ff007088', '#00ffff88', '#ffffff44'];
                for(let i=0; i<6; i++) {
                    const bh = (4 + Math.random()*14) * sc;
                    const by = sy + Math.random() * sh;
                    const boff = (Math.random()-0.5) * 30 * amt * sc;
                    ctx.fillStyle = barColors[i % barColors.length];
                    ctx.fillRect(sx + boff, by, sw * (0.4 + Math.random()*0.6), bh);
                }
            }
        }
        break;
      }
      case 'Sharpening': {
        const amt = l._abs_op || 1;
        const size = 150 * sc;
        const sx = Math.max(0, cx - size);
        const sy = Math.max(0, cy - size);
        const sw = Math.min(W - sx, size * 2);
        const sh = Math.min(H - sy, size * 2);
        ctx.fillStyle = `rgba(255,255,255,${amt*0.03})`;
        ctx.globalCompositeOperation = 'overlay';
        ctx.fillRect(sx, sy, sw, sh);
        ctx.globalCompositeOperation = 'source-over';
        // Visible representative: draw a sharp edge-detect pattern
        const edgeR = 32 * sc;
        ctx.save();
        ctx.beginPath(); ctx.arc(cx, cy, edgeR, 0, Math.PI*2);
        ctx.strokeStyle = `rgba(255,255,255,${0.3 + amt*0.4})`; ctx.lineWidth = 2 * sc;
        ctx.shadowColor = '#fff'; ctx.shadowBlur = 6 * amt * sc;
        ctx.stroke();
        ctx.shadowBlur = 0;
        // Inner concentric rings to suggest edge clarity
        for(let i=1; i<=3; i++) {
            ctx.beginPath(); ctx.arc(cx, cy, edgeR*(0.35+i*0.2), 0, Math.PI*2);
            ctx.strokeStyle = `rgba(255,255,200,${0.15*amt})`; ctx.lineWidth = 1;
            ctx.stroke();
        }
        ctx.restore();
        ctx.font = `${Math.round(9*sc)}px system-ui`; ctx.textAlign='center'; ctx.fillStyle='rgba(255,255,255,0.6)';
        ctx.fillText('SHARPEN', cx, cy + edgeR + 14*sc);
        break;
      }
      case 'CameraShake': {
        const shakeAmt = (l._abs_op || 1) * 12 * sc * reactMult;
        const shakeX = (Math.random() - 0.5) * shakeAmt;
        const shakeY = (Math.random() - 0.5) * shakeAmt;
        ctx.translate(shakeX, shakeY);
        // Draw a shake indicator
        ctx.strokeStyle = c + '60';
        ctx.lineWidth = 2;
        ctx.setLineDash([4, 4]);
        ctx.strokeRect(cx - 60*sc + shakeX, cy - 40*sc + shakeY, 120*sc, 80*sc);
        ctx.setLineDash([]);
        ctx.fillStyle = c;
        ctx.font = `${Math.round(10*sc)}px system-ui`;
        ctx.textAlign = 'center';
        ctx.fillText('SHAKE', cx + shakeX, cy + shakeY);
        break;
      }
      default: {
        ctx.beginPath(); ctx.arc(cx,cy,32*sc,0,Math.PI*2);
        ctx.fillStyle=c+'40'; ctx.fill();
        ctx.strokeStyle=c; ctx.lineWidth=2; ctx.stroke();
      }
    }
    ctx.restore();
    ctx.globalAlpha=1;
  }

  function render(){
    const canvas=document.getElementById('vibe-preview-canvas');
    if(!canvas){requestAnimationFrame(render);return;}
    
    setupAnalyser();
    if (window.__vibeAnalyser) {
      const buf = new Uint8Array(window.__vibeAnalyser.frequencyBinCount);
      window.__vibeAnalyser.getByteFrequencyData(buf);
      const third = Math.floor(buf.length / 3);
      const avg = (a, b) => buf.slice(a,b).reduce((s,v)=>s+v,0) / ((b-a)||1) / 255;
      window.__vibeFreq = { bass: avg(0,third), mid: avg(third,2*third), treble: avg(2*third,buf.length) };
      
      const timeBuf = new Uint8Array(window.__vibeAnalyser.frequencyBinCount);
      window.__vibeAnalyser.getByteTimeDomainData(timeBuf);
      window.__vibeTimeDomain = Array.from(timeBuf);
    }

    if(!window.__vibeEventsAttached) {
       window.__vibeEventsAttached = true;
       window.__vibeHoveredLayer = null;
       canvas.addEventListener('mousemove', e => {
           const rect = canvas.getBoundingClientRect();
           const x = e.clientX - rect.left;
           const y = e.clientY - rect.top;
           let hit = null;
           const W = canvas.width;
           const H = canvas.height;
           for(let i=(window.__vibeLayers||[]).length-1; i>=0; i--) {
              let l = window.__vibeLayers[i];
              if(l.visible === false) continue;
              let cx = W/2 + (l.pos_x||0)*W/200;
              let cy = H/2 + (l.pos_y||0)*H/200;
              let sc = l.scale || 1;
              let r = 50 * sc; 
              let dx = x - cx;
              let dy = y - cy;
              if(dx*dx + dy*dy <= r*r) {
                 hit = l.id;
                 break;
              }
           }
           window.__vibeHoveredLayer = hit || null;
       });

       window.__vibePanning = false;
       window.__vibePanStartX = 0;
       window.__vibePanStartScroll = 0;
       window.addEventListener('mousedown', e => {
           if(e.button === 1) {
               const tl = document.querySelector('.timeline-track-area');
               if(tl && tl.contains(e.target)) {
                   e.preventDefault();
                   window.__vibePanning = true;
                   window.__vibePanStartX = e.clientX;
                   window.__vibePanStartScroll = tl.scrollLeft;
               }
           }
       });
       window.addEventListener('mousemove', e => {
           if(window.__vibePanning) {
               const tl = document.querySelector('.timeline-track-area');
               if(tl) {
                   tl.scrollLeft = window.__vibePanStartScroll - (e.clientX - window.__vibePanStartX);
               }
           }
       });
       window.addEventListener('mouseup', e => {
           if(e.button === 1) window.__vibePanning = false;
       });
    }
    const ctx=canvas.getContext('2d');
    const W=canvas.width=canvas.offsetWidth;
    const H=canvas.height=canvas.offsetHeight;
    window.__vibeTime=(window.__vibeTime||0)+0.016;
    ctx.fillStyle='#08080f'; ctx.fillRect(0,0,W,H);
    ctx.strokeStyle='rgba(255,255,255,0.025)'; ctx.lineWidth=1;
    for(let x=0;x<W;x+=40){ctx.beginPath();ctx.moveTo(x,0);ctx.lineTo(x,H);ctx.stroke();}
    for(let y=0;y<H;y+=40){ctx.beginPath();ctx.moveTo(0,y);ctx.lineTo(W,y);ctx.stroke();}
    
    const layers=(window.__vibeLayers||[]);
    // Draw project aspect ratio letterbox
    const pw = window.__vibeProjectW || 1920;
    const ph = window.__vibeProjectH || 1080;
    const pAspect = pw / ph;
    let frameW, frameH;
    if (W / H > pAspect) { frameH = H * 0.92; frameW = frameH * pAspect; }
    else { frameW = W * 0.92; frameH = frameW / pAspect; }
    const fx = (W - frameW) / 2;
    const fy = (H - frameH) / 2;
    ctx.strokeStyle = 'rgba(123,97,255,0.2)';
    ctx.lineWidth = 1;
    ctx.setLineDash([6, 4]);
    ctx.strokeRect(fx, fy, frameW, frameH);
    ctx.setLineDash([]);
    ctx.fillStyle = 'rgba(123,97,255,0.15)';
    ctx.font = '9px system-ui';
    ctx.textAlign = 'left';
    ctx.fillText(pw + 'x' + ph, fx + 4, fy + 12);
    let map = {};
    layers.forEach(l => { map[l['id']] = l; });
    
    // Compute absolute hierarchical transforms
    layers.forEach(l => {
       let abs_x = l.pos_x||0;
       let abs_y = l.pos_y||0;
       let abs_sc = l.scale||1;
       let abs_op = l.opacity!=null?l.opacity:1;
       let abs_rot = l.rot||0;
       let vis = l.visible!==false;
       
       let p = map[l.parent];
       let depth = 0;
       let p_react_bass = false;
       let p_react_mid = false;
       let p_react_treble = false;

       while(p && depth < 20) {
          abs_x += p.pos_x||0;
          abs_y += p.pos_y||0;
          abs_sc *= p.scale||1;
          abs_op *= p.opacity!=null?p.opacity:1;
          abs_rot += p.rot||0;
          if(p.visible===false) vis = false;
          
          if (p.audio_react === 'Bass') p_react_bass = true;
          if (p.audio_react === 'Mid') p_react_mid = true;
          if (p.audio_react === 'Treble') p_react_treble = true;

          p = map[p.parent];
          depth++;
       }
       l._abs_x = abs_x;
       l._abs_y = abs_y;
       l._abs_sc = abs_sc;
       l._abs_op = abs_op;
       l._abs_rot = abs_rot;
       l._abs_vis = vis;

       if (p_react_bass && l.audio_react !== 'Bass') l.audio_react = 'Bass';
       if (p_react_mid && l.audio_react !== 'Mid') l.audio_react = 'Mid';
       if (p_react_treble && l.audio_react !== 'Treble') l.audio_react = 'Treble';
    });

    let _visibleCount = 0;
    const globalTime = window.__vibeTime || 0;
    const playbackTime = window.__vibeCurrentTime !== undefined ? window.__vibeCurrentTime : globalTime;
    layers.forEach(l=>{
        if(!l._abs_vis) return;
        if(l.type === 'Composition' || l.type === 'Workstream') return;
        
        // Compute parent-relative playback time by walking up the parent chain
        let effectiveTime = playbackTime;
        let p = map[l.parent];
        while(p) {
            effectiveTime -= (p.start_time || 0);
            // Check parent is active at this time
            if(p.type === 'Composition' || p.type === 'Workstream') {
                if(p.duration !== undefined && p.duration > 0) {
                    if(effectiveTime < -0.001 || effectiveTime > p.duration + 0.001) return;
                }
            }
            p = map[p.parent];
        }
        
        // Check own time range using parent-relative time
        const local_ct = effectiveTime - (l.start_time || 0);
        if(l.duration !== undefined && l.duration > 0) {
            if(local_ct < -0.001 || local_ct > l.duration + 0.001) return;
        }
        
        _visibleCount++;
        drawLayer(ctx, l, globalTime, W, H);
    });
    
    if(_visibleCount===0){
      ctx.font='13px system-ui'; ctx.textAlign='center'; ctx.textBaseline='middle';
      ctx.fillStyle='rgba(255,255,255,0.15)';
      ctx.fillText('Canvas Preview — add layers in the sidebar',W/2,H/2);
    }

    // ── Multi-Audio/Video Player Management ──
    if(!window.__vibeAudioPlayers) window.__vibeAudioPlayers = {};
    if(!window.__vibePlayerAnalysers) window.__vibePlayerAnalysers = {};
    const mediaLayers = layers.filter(l => (l.type === 'Audio' || l.type === 'Video') && l._abs_vis && l.media_url);
    const isPlaying = window.__vibeIsPlaying || false;
    const masterVol = window.__vibeMasterVolume !== undefined ? window.__vibeMasterVolume : 1.0;
    
    // Ensure we have an AudioContext for connecting analyser nodes
    if(!window.__vibePlayerAudioCtx) {
        try { window.__vibePlayerAudioCtx = new AudioContext(); } catch(e) {}
    }
    if(!window.__vibePlayerAnalyser && window.__vibePlayerAudioCtx) {
        try {
            window.__vibePlayerAnalyser = window.__vibePlayerAudioCtx.createAnalyser();
            window.__vibePlayerAnalyser.fftSize = 256;
            window.__vibePlayerAnalyser.connect(window.__vibePlayerAudioCtx.destination);
        } catch(e) {}
    }
    
    // Create/update audio/video elements for each media layer
    mediaLayers.forEach(al => {
        if(!window.__vibeAudioPlayers[al.id]) {
            let mediaEl;
            if(al.type === 'Video') {
                // For video layers, reuse the cached video element if exists
                mediaEl = window.__vibeMediaCache && window.__vibeMediaCache[al.media_url];
                if(!mediaEl) {
                    mediaEl = document.createElement('video');
                    mediaEl.src = al.media_url;
                    mediaEl.crossOrigin = 'anonymous';
                    mediaEl.muted = false;
                    mediaEl.preload = 'auto';
                    if(!window.__vibeMediaCache) window.__vibeMediaCache = {};
                    window.__vibeMediaCache[al.media_url] = mediaEl;
                }
            } else {
                mediaEl = new Audio();
                mediaEl.crossOrigin = 'anonymous';
                mediaEl.src = al.media_url;
                mediaEl.preload = 'auto';
            }
            window.__vibeAudioPlayers[al.id] = { el: mediaEl, lastSrc: al.media_url, type: al.type };
            
            // Connect to shared analyser for audio reactivity
            if(window.__vibePlayerAudioCtx && window.__vibePlayerAnalyser && !window.__vibePlayerAnalysers[al.id]) {
                try {
                    if(window.__vibePlayerAudioCtx.state === 'suspended') window.__vibePlayerAudioCtx.resume();
                    const src = window.__vibePlayerAudioCtx.createMediaElementSource(mediaEl);
                    src.connect(window.__vibePlayerAnalyser);
                    window.__vibePlayerAnalysers[al.id] = src;
                } catch(e) { /* may fail if already connected */ }
            }
        }
        const player = window.__vibeAudioPlayers[al.id];
        if(player.lastSrc !== al.media_url) {
            player.el.src = al.media_url;
            player.lastSrc = al.media_url;
        }
        player.el.volume = (al._abs_op !== undefined ? al._abs_op : 1.0) * masterVol;
        
        const ct2 = window.__vibeCurrentTime !== undefined ? window.__vibeCurrentTime : window.__vibeTime;
        const local_ct2 = ct2 - (al.start_time || 0);
        const layerDur = al.duration !== undefined && al.duration > 0 ? al.duration : 999999;
        const inRange = local_ct2 >= -0.05 && local_ct2 <= layerDur + 0.05;
        
        if(isPlaying && inRange) {
            // Clamp currentTime so audio doesn't play beyond trimmed duration
            const clampedTime = Math.min(Math.max(0, local_ct2), layerDur);
            if(Math.abs(player.el.currentTime - clampedTime) > 0.3) {
                player.el.currentTime = clampedTime;
            }
            if(player.el.paused) player.el.play().catch(()=>{});
        } else {
            if(!player.el.paused) player.el.pause();
        }
    });
    // Pause any players for removed/inactive layers
    Object.keys(window.__vibeAudioPlayers).forEach(id => {
        if(!mediaLayers.find(al => al.id === id)) {
            window.__vibeAudioPlayers[id].el.pause();
        }
    });
    
    // Update shared analyser frequency data for audio reactivity
    if(window.__vibePlayerAnalyser) {
        const buf = new Uint8Array(window.__vibePlayerAnalyser.frequencyBinCount);
        window.__vibePlayerAnalyser.getByteFrequencyData(buf);
        const third = Math.floor(buf.length / 3);
        const avg = (a, b) => buf.slice(a,b).reduce((s,v)=>s+v,0) / ((b-a)||1) / 255;
        window.__vibeFreq = { bass: avg(0,third), mid: avg(third,2*third), treble: avg(2*third,buf.length) };
        
        const timeBuf = new Uint8Array(window.__vibePlayerAnalyser.frequencyBinCount);
        window.__vibePlayerAnalyser.getByteTimeDomainData(timeBuf);
        window.__vibeTimeDomain = Array.from(timeBuf);
    }

    // Apply Global Post-Effects
    const g = window.__vibeGlobals || {};
    const filterStr = `hue-rotate(${g.hue||0}deg) saturate(${g.sat!==undefined?g.sat:1.0}) contrast(${1.0 + (g.sharp||0)*0.5}) brightness(${1.0 + (g.bloom||0)*0.2}) drop-shadow(0 0 ${(g.bloom||0)*20}px rgba(255,255,255,${(g.bloom||0)*0.5}))`;
    canvas.style.filter = filterStr;

    if (g.film_grain > 0) {
      ctx.fillStyle = `rgba(255,255,255,${g.film_grain*0.05})`;
      for(let i=0; i<W*H*0.001*g.film_grain; i++) ctx.fillRect(Math.random()*W, Math.random()*H, 2, 2);
    }
    if (g.vhs > 0) {
      ctx.fillStyle = `rgba(0,0,0,${g.vhs*0.1})`;
      ctx.fillRect(0, Math.random()*H, W, 4 + Math.random()*10);
      ctx.fillStyle = `rgba(255,255,255,${g.vhs*0.05})`;
      ctx.fillRect(0, Math.random()*H, W, 2);
    }
    if (g.chromatic > 0) {
        if (!window.__vibeOffscreenCanvas) window.__vibeOffscreenCanvas = document.createElement('canvas');
        window.__vibeOffscreenCanvas.width = W; window.__vibeOffscreenCanvas.height = H;
        const octx = window.__vibeOffscreenCanvas.getContext('2d');
        octx.clearRect(0, 0, W, H);
        octx.drawImage(canvas, 0, 0);
        
        ctx.globalCompositeOperation = 'screen';
        ctx.globalAlpha = g.chromatic * 0.5;
        ctx.drawImage(window.__vibeOffscreenCanvas, g.chromatic * 8, 0);
        ctx.drawImage(window.__vibeOffscreenCanvas, -g.chromatic * 8, 0);
        ctx.globalAlpha = 1.0;
        ctx.globalCompositeOperation = 'source-over';
    }
    if (g.vignette > 0) {
        const vGrad = ctx.createRadialGradient(W/2, H/2, H*0.25, W/2, H/2, H*0.85);
        vGrad.addColorStop(0, 'rgba(0,0,0,0)');
        vGrad.addColorStop(1, `rgba(0,0,0,${g.vignette})`);
        ctx.fillStyle = vGrad;
        ctx.fillRect(0, 0, W, H);
    }
    // ── Waveform Canvases on Timeline ──
    if(!window.__vibeWaveformCache) window.__vibeWaveformCache = {};
    document.querySelectorAll('canvas[id^="wavecanvas-"]').forEach(wc => {
        const wCtx = wc.getContext('2d');
        const wW = wc.width = wc.offsetWidth || 200;
        const wH = wc.height = wc.offsetHeight || 22;
        wCtx.clearRect(0, 0, wW, wH);
        const wColor = wc.getAttribute('data-wave-color') || '#34d399';
        
        // Extract layer ID from canvas id: "wavecanvas-<layerId>"
        const layerId = wc.id.replace('wavecanvas-', '');
        
        // Find matching layer to get its media_url
        const audioLayer = layers.find(l => l.id === layerId);
        const mediaUrl = audioLayer ? audioLayer.media_url : null;
        
        // If we have a URL, try to decode and cache the waveform
        if(mediaUrl && !window.__vibeWaveformCache[layerId] && !window.__vibeWaveformCache['_loading_' + layerId]) {
            window.__vibeWaveformCache['_loading_' + layerId] = true;
            fetch(mediaUrl)
                .then(r => r.arrayBuffer())
                .then(buf => {
                    const actx = new (window.AudioContext || window.webkitAudioContext)();
                    return actx.decodeAudioData(buf);
                })
                .then(decoded => {
                    const raw = decoded.getChannelData(0);
                    const samples = 500;
                    const step = Math.floor(raw.length / samples) || 1;
                    const waveData = [];
                    for(let i = 0; i < samples; i++) {
                        let sum = 0, count = 0;
                        for(let j = i * step; j < (i+1) * step && j < raw.length; j++) {
                            sum += Math.abs(raw[j]);
                            count++;
                        }
                        waveData.push(count > 0 ? sum / count : 0);
                    }
                    window.__vibeWaveformCache[layerId] = waveData;
                    delete window.__vibeWaveformCache['_loading_' + layerId];
                })
                .catch(() => { delete window.__vibeWaveformCache['_loading_' + layerId]; });
        }
        
        // Draw from cache or fallback to sine placeholder
        const cached = window.__vibeWaveformCache[layerId];
        wCtx.beginPath();
        if(cached && cached.length > 0) {
            const len = cached.length;
            for(let i = 0; i < len; i++) {
                const x = (i / (len - 1)) * wW;
                const barH = cached[i] * wH * 0.8;
                wCtx.moveTo(x, wH/2 - barH);
                wCtx.lineTo(x, wH/2 + barH);
            }
            wCtx.strokeStyle = wColor;
            wCtx.lineWidth = 1;
            wCtx.stroke();
        } else {
            // Animated sine placeholder while loading
            const len = 100;
            for(let i = 0; i < len; i++) {
                const x = (i / (len - 1)) * wW;
                const val = Math.sin(i * 0.12 + (window.__vibeTime || 0)) * 0.5;
                const y = wH / 2 + val * (wH * 0.4);
                i === 0 ? wCtx.moveTo(x, y) : wCtx.lineTo(x, y);
            }
            wCtx.strokeStyle = wColor;
            wCtx.lineWidth = 1.5;
            wCtx.stroke();
        }
    });

    // ── Mini Preview Canvas for AddItemModal ──
    const miniPreview = document.getElementById('vibe-effect-preview-canvas');
    if(miniPreview) {
        const mCtx = miniPreview.getContext('2d');
        const mW = miniPreview.width;
        const mH = miniPreview.height;
        mCtx.fillStyle = '#08080f';
        mCtx.fillRect(0, 0, mW, mH);
        const effectType = miniPreview.getAttribute('data-effect-type');
        const effectColor = miniPreview.getAttribute('data-effect-color') || '#7b61ff';
        if(effectType) {
            const demoLayer = {
                type: effectType,
                color: effectColor,
                _abs_x: 0, _abs_y: 0, _abs_sc: 1, _abs_op: 1, _abs_rot: 0, _abs_vis: true,
                pos_x: 0, pos_y: 0, scale: 0.6, opacity: 1, rot: 0,
                audio_react: 'None', visible: true, dir: 1,
                flip_x: false, flip_y: false, skew_x: 0, skew_y: 0,
                text_str: 'DEMO', text_size: 28, text_color: '#ffffff',
                text_stroke: '#000000', text_stroke_w: 2,
                text_shadow: 'rgba(0,0,0,0.8)', text_shadow_b: 8,
                perspective: [0, 0]
            };
            drawLayer(mCtx, demoLayer, window.__vibeTime || 0, mW, mH);
        }
    }

    requestAnimationFrame(render);
  }
  requestAnimationFrame(render);

  // ── Waveform Cache ──
  if(!window.__vibeWaveformCache) window.__vibeWaveformCache = {};

  // ── Middle-Click Pan Handler ──
  if(!window.__vibePanSetup) {
    window.__vibePanSetup = true;
    let panActive = false, panStartX = 0, panStartScroll = 0;
    document.addEventListener('pointerdown', (e) => {
        if(e.button !== 1) return; // Middle button only
        const ta = e.target.closest('.timeline-track-area');
        if(!ta) return;
        e.preventDefault();
        panActive = true;
        panStartX = e.clientX;
        panStartScroll = ta.scrollLeft;
        ta.setPointerCapture(e.pointerId);
    });
    document.addEventListener('pointermove', (e) => {
        if(!panActive) return;
        const ta = document.querySelector('.timeline-track-area');
        if(!ta) return;
        ta.scrollLeft = panStartScroll - (e.clientX - panStartX);
    });
    document.addEventListener('pointerup', (e) => {
        if(e.button !== 1) return;
        panActive = false;
    });
    // Prevent context menu on middle click in timeline
    document.addEventListener('auxclick', (e) => {
        if(e.button === 1 && e.target.closest('.timeline-track-area')) e.preventDefault();
    });
  }
})();
        "#);
    });

    // Sync layer data from Rust -> JS every render frame
    {
        let s = state.read();
        let layers_json = s.layers.iter()
            .map(|l| format!(
                r#"{{ "id":"{id}","parent":"{pid}","name":"{name}","type":"{ty:?}","visible":{vis},"start_time":{st:?},"duration":{dur:?},"fade_in":{fin:?},"fade_out":{fout:?},"opacity":{op:.2},"scale":{sc:.2},"rot":{rot:.2},"skew_x":{skx:.2},"skew_y":{sky:.2},"flip_x":{fx},"flip_y":{fy},"pos_x":{px:.1},"pos_y":{py:.1},"dir":{dir},"color":"{col}","custom_color":{ccol},"media_url":{url},"audio_react":"{areact:?}","text_str":"{t_str}","text_size":{t_sz},"text_color":"{t_c}","text_stroke":"{t_sc}","text_stroke_w":{t_sw},"text_shadow":"{t_shc}","text_shadow_b":{t_shb},"perspective":[{persp0},{persp1}] }}"#,
                id = l.id,
                pid = l.parent_id.as_deref().unwrap_or(""),
                name = l.name.replace('"', "'"),
                ty = l.layer_type,
                vis = if l.visible { "true" } else { "false" },
                st = l.start_time,
                dur = l.duration,
                fin = l.fade_in,
                fout = l.fade_out,
                op = l.opacity,
                sc = l.scale,
                rot = l.rotation,
                skx = l.skew_x,
                sky = l.skew_y,
                fx = if l.flip_x { "true" } else { "false" },
                fy = if l.flip_y { "true" } else { "false" },
                px = l.position[0],
                py = l.position[1],
                dir = l.effect_params.direction,
                col = l.custom_color.as_deref().unwrap_or(l.layer_type.color_hex()),
                ccol = match &l.custom_color { Some(c) => format!("\"{}\"", c), None => "null".to_string() },
                url = match &l.media_url { Some(u) => format!("\"{}\"", u), None => "null".to_string() },
                areact = l.audio_react,
                t_str = l.text_params.text.escape_default(),
                t_sz = l.text_params.font_size,
                t_c = l.text_params.color,
                t_sc = l.text_params.stroke_color,
                t_sw = l.text_params.stroke_width,
                t_shc = l.text_params.shadow_color,
                t_shb = l.text_params.shadow_blur,
                persp0 = l.perspective[0],
                persp1 = l.perspective[1]
            ))
            .collect::<Vec<String>>().join(",");
        let globals_json = format!(
            r#"{{"bloom":{},"chromatic":{},"film_grain":{},"vhs":{},"hue":{},"sat":{},"sharp":{},"vignette":{}}}"#,
            s.global_bloom, s.global_chromatic, s.global_film_grain, s.global_vhs,
            s.global_color_hue, s.global_color_saturation, s.global_sharpening, s.global_vignette
        );
        let ct = s.current_time;
        let is_playing = s.is_playing;
        let js = format!(
            "window.__vibeLayers=[{}]; window.__vibeGlobals={}; window.__vibeMasterVolume={}; window.__vibeProjectW={}; window.__vibeProjectH={}; window.__vibeSelectedId='{}'; window.__vibeCurrentTime={}; window.__vibeIsPlaying={};", 
            layers_json, globals_json, s.master_volume,
            s.project_width, s.project_height,
            s.selected_id.as_deref().unwrap_or(""),
            ct, is_playing
        );
        let _ = js_sys::eval(&js);
    }

    rsx! {
        div {
            class: "canvas-area",
            style: "flex-grow: 1; position: relative; overflow: hidden; min-width: 0; min-height: 0; background: #08080f;",

            // Animated canvas
            canvas {
                id: "vibe-preview-canvas",
                style: "position: absolute; inset: 0; width: 100%; height: 100%; display: block;",
                onpointerdown: move |evt| {
                    // Restrict dragging strictly to the currently selected layer
                    let mut s = state.write();
                    s.drag.is_canvas_drag = true;
                    s.drag.last_pos = Some((evt.client_coordinates().x, evt.client_coordinates().y));
                },
                onpointermove: move |evt| {
                    let mut s = state.write();
                    if s.drag.is_canvas_drag {
                        if let Some((lx, ly)) = s.drag.last_pos {
                            let cx = evt.client_coordinates().x;
                            let cy = evt.client_coordinates().y;
                            let dx = cx - lx;
                            let dy = cy - ly;
                            s.drag.last_pos = Some((cx, cy));
                            
                            // Apply movement to the selected layer
                            if let Some(sel_id) = s.selected_id.clone() {
                                if let Some(layer) = s.layers.iter_mut().find(|l| l.id == sel_id) {
                                    // Read exact canvas client bounds to match mouse 1:1
                                    let mut w = 1600.0;
                                    let mut h = 900.0;
                                    if let Ok(w_val) = js_sys::eval("document.getElementById('vibe-preview-canvas').offsetWidth") {
                                        if let Some(v) = w_val.as_f64() { w = v; }
                                    }
                                    if let Ok(h_val) = js_sys::eval("document.getElementById('vibe-preview-canvas').offsetHeight") {
                                        if let Some(v) = h_val.as_f64() { h = v; }
                                    }
                                    let mult_x = 200.0 / w;
                                    let mult_y = 200.0 / h;
                                    layer.position[0] += (dx as f32) * (mult_x as f32);
                                    layer.position[1] += (dy as f32) * (mult_y as f32);
                                }
                            }
                        }
                    }
                },
                onwheel: move |evt| {
                    let mut s = state.write();
                    if let Some(sel_id) = s.selected_id.clone() {
                        if let Some(layer) = s.layers.iter_mut().find(|l| l.id == sel_id) {
                            let delta = match evt.delta() {
                                dioxus::html::geometry::WheelDelta::Pixels(p) => p.y,
                                dioxus::html::geometry::WheelDelta::Lines(p) => p.y * 16.0,
                                dioxus::html::geometry::WheelDelta::Pages(p) => p.y * 100.0,
                            };
                            // delta is usually 100 or -100 per tick
                            let zoom_delta = (delta / 10.0) as f32; // ~10.0 units per tick
                            layer.position[2] -= zoom_delta; // negative delta (scroll up) increases Z (closer)
                        }
                    }
                },
                onpointerup: move |_| {
                    state.write().drag.is_canvas_drag = false;
                    state.write().drag.last_pos = None;
                },
                onmouseleave: move |_| {
                    state.write().drag.is_canvas_drag = false;
                    state.write().drag.last_pos = None;
                }
            }

            // Audio status chip (non-blocking overlay)
            if state.read().audio_loaded {
                div {
                    style: "position: absolute; top: 10px; left: 10px; background: rgba(0,0,0,0.55); border: 1px solid rgba(52,211,153,0.35); border-radius: 20px; padding: 4px 12px; font-size: 10px; color: #34d399; pointer-events: none;",
                    "♪ {state.read().audio_file_name.as_deref().unwrap_or(\"\")}"
                }
            }

            // Global upload event listeners
            {
                let state_clone = state.to_owned();
                let audio_ctx_clone = audio_ctx.clone();
                use_effect(move || {
                    let window = web_sys::window().unwrap();
                    
                    // Audio Upload Listener
                    let mut s_audio = state_clone.to_owned();
                    let actx = audio_ctx_clone.clone();
                    let closure_audio = Closure::wrap(Box::new(move |evt: web_sys::CustomEvent| {
                        let detail = evt.detail();
                        if let Ok(id_val) = js_sys::Reflect::get(&detail, &JsValue::from_str("id")) {
                            if let Ok(url_val) = js_sys::Reflect::get(&detail, &JsValue::from_str("url")) {
                                if let (Some(id), Some(url)) = (id_val.as_string(), url_val.as_string()) {
                                    let mut parent_to_expand: Option<(String, f64)> = None;
                                    {
                                        let mut sw = s_audio.write();
                                        if let Some(l) = sw.layers.iter_mut().find(|l| l.id == id) {
                                            l.media_url = Some(url.clone());
                                            if let Ok(dur_val) = js_sys::Reflect::get(&detail, &JsValue::from_str("duration")) {
                                                if let Some(dur) = dur_val.as_f64() {
                                                    l.duration = dur;
                                                    // Store parent expansion info
                                                    if let Some(pid) = l.parent_id.clone() {
                                                        parent_to_expand = Some((pid, l.start_time + dur));
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    // Expand parent composition to fit audio duration
                                    if let Some((pid, child_end)) = parent_to_expand {
                                        s_audio.write().expand_parent_duration(&pid, child_end);
                                    }
                                    if let Some(engine) = &mut *actx.borrow_mut() {
                                        engine.load_url(&url);
                                    }
                                    if let Ok(name_val) = js_sys::Reflect::get(&detail, &JsValue::from_str("name")) {
                                        if let Some(n) = name_val.as_string() {
                                            s_audio.write().audio_file_name = Some(n);
                                        }
                                    }
                                    s_audio.write().audio_loaded = true;
                                }
                            }
                        }
                    }) as Box<dyn FnMut(_)>);
                    let _ = window.add_event_listener_with_callback("vibe_audio_uploaded", closure_audio.as_ref().unchecked_ref());
                    closure_audio.forget(); // leak intentionally for global listening

                    // Image Upload Listener
                    let mut s_img = state_clone.to_owned();
                    let closure_img = Closure::wrap(Box::new(move |evt: web_sys::CustomEvent| {
                        let detail = evt.detail();
                        if let Ok(id_val) = js_sys::Reflect::get(&detail, &JsValue::from_str("id")) {
                            if let Ok(url_val) = js_sys::Reflect::get(&detail, &JsValue::from_str("url")) {
                                if let (Some(id), Some(url)) = (id_val.as_string(), url_val.as_string()) {
                                    if let Some(l) = s_img.write().layers.iter_mut().find(|l| l.id == id) {
                                        l.media_url = Some(url);
                                    }
                                }
                            }
                        }
                    }) as Box<dyn FnMut(_)>);
                    let _ = window.add_event_listener_with_callback("vibe_image_uploaded", closure_img.as_ref().unchecked_ref());
                    closure_img.forget();
                });
            }



            // Add-layer modal overlays the canvas
            if show_modal {
                AddItemModal {}
            }
        }
    }
}

// ─── Settings Panel Component ──────────────────────────────────────────────────
#[component]
fn SettingsPanel() -> Element {
    let mut state = use_context::<Signal<AppState>>();
    
    rsx! {
        div {
            style: "position: absolute; top: 38px; left: 12px; width: 260px; background: rgba(18, 18, 30, 0.95); backdrop-filter: blur(16px); border: 1px solid rgba(255,255,255,0.1); border-radius: 12px; padding: 16px; z-index: 100; box-shadow: 0 16px 48px rgba(0,0,0,0.8); display: flex; flex-direction: column; gap: 16px;",
            
            // Header
            div {
                style: "display: flex; justify-content: space-between; align-items: center; padding-bottom: 12px; border-bottom: 1px solid rgba(255,255,255,0.08);",
                span { style: "font-size: 13px; font-weight: 600; color: #fff; letter-spacing: -0.01em;", "Studio Settings" }
                button {
                    style: "background: none; border: none; color: rgba(255,255,255,0.4); cursor: pointer; display: flex; align-items: center; justify-content: center; width: 20px; height: 20px; border-radius: 4px; transition: all 0.15s ease;",
                    onclick: move |_| state.write().show_settings = false,
                    onmouseenter: move |evt| evt.stop_propagation(), // visual hover via CSS normally, skip inline for brevity
                    "✕"
                }
            }
            
            // Master Volume Slider
            div { style: "display: flex; flex-direction: column; gap: 8px;",
                div { style: "display: flex; justify-content: space-between; align-items: center;",
                    span { style: "font-size: 11px; color: rgba(255,255,255,0.6);", "Master Volume" }
                    span { style: "font-size: 10px; color: #4ade80; font-family: monospace;", "{state.read().master_volume * 100.0:.0}%" }
                }
                input {
                    r#type: "range", min: "0", max: "1", step: "0.01",
                    value: "{state.read().master_volume}",
                    style: "width: 100%; accent-color: #4ade80; cursor: pointer;",
                    oninput: move |evt| {
                        if let Ok(val) = evt.value().parse::<f64>() {
                            state.write().master_volume = val;
                        }
                    }
                }
            }

            // UI Scale Slider
            div { style: "display: flex; flex-direction: column; gap: 8px;",
                div { style: "display: flex; justify-content: space-between; align-items: center;",
                    span { style: "font-size: 11px; color: rgba(255,255,255,0.6);", "UI Scale" }
                    span { style: "font-size: 10px; color: #a855f7; font-family: monospace;", "{state.read().ui_scale * 100.0:.0}%" }
                }
                input {
                    r#type: "range", min: "0.75", max: "1.5", step: "0.05",
                    value: "{state.read().ui_scale}",
                    style: "width: 100%; accent-color: #a855f7; cursor: pointer;",
                    oninput: move |evt| {
                        if let Ok(val) = evt.value().parse::<f64>() {
                            state.write().ui_scale = val;
                        }
                    }
                }
            }

            // Terminal Toggle
            div { style: "display: flex; justify-content: space-between; align-items: center; margin-top: 4px;",
                span { style: "font-size: 11px; color: rgba(255,255,255,0.6);", "Show Terminal" }
                input {
                    r#type: "checkbox",
                    checked: "{state.read().show_terminal}",
                    style: "cursor: pointer; accent-color: #a855f7;",
                    onchange: move |evt| {
                        state.write().show_terminal = evt.value() == "true";
                    }
                }
            }
        }
    }
}

// ─── Main App ─────────────────────────────────────────────────────────────────
#[component]
fn App() -> Element {
    let mut state = use_signal(AppState::default);
    use_context_provider(|| state);

    let audio_ctx = use_context_provider(|| Rc::new(RefCell::new(audio::AudioEngine::new().ok())));

    let mut terminal_input = use_signal(String::new);

    use_future(move || {
        let audio_sync = audio_ctx.clone();
        async move {
            loop {
                sleep(std::time::Duration::from_millis(16)).await;
                
                // Keep audio volume continuously synced
                {
                    let s = state.read();
                    let master_vol = s.master_volume;
                    let current_t = s.current_time;
                    let mut audio_track_vol = 0.0;
                    for l in s.layers.iter() {
                        if l.layer_type == LayerType::Audio && l.visible {
                            let local_t = current_t - l.start_time;
                            if local_t >= 0.0 && local_t <= l.duration {
                                let mut fade_mult = 1.0;
                                if l.fade_in > 0.0 && local_t < l.fade_in {
                                    fade_mult = (local_t / l.fade_in).max(0.0);
                                } else if l.fade_out > 0.0 && local_t > l.duration - l.fade_out {
                                    fade_mult = (1.0 - ((local_t - (l.duration - l.fade_out)) / l.fade_out)).max(0.0);
                                }
                                audio_track_vol = l.opacity as f64 * fade_mult;
                            }
                            break;
                        }
                    }
                    if let Some(eng) = audio_sync.borrow().as_ref() {
                        eng.set_volume(master_vol * audio_track_vol);
                    }
                }

                if state.read().is_playing {
                    let mut s = state.write();
                    let audio_loaded = s.audio_loaded;
                    let t = if audio_loaded {
                        if let Some(eng) = audio_sync.borrow().as_ref() {
                            let dur = eng.duration();
                            if dur > 0.0 && !dur.is_nan() {
                                let mut to_expand = Vec::new();
                                for l in s.layers.iter_mut() {
                                    if l.layer_type == LayerType::Audio && (l.duration - dur).abs() > 0.1 {
                                        l.duration = dur;
                                        if let Some(pid) = &l.parent_id {
                                            to_expand.push((pid.clone(), l.start_time + dur));
                                        }
                                    }
                                }
                                for (pid, end_time) in to_expand {
                                    s.expand_parent_duration(&pid, end_time);
                                }
                            }
                            Some(eng.current_time())
                        } else {
                            None
                        }
                    } else {
                        None
                    };

                    let next = t.unwrap_or(s.current_time + 0.016);
                    let dur = s.timeline_duration();
                    
                    if next > dur {
                        if s.loop_playback {
                            s.current_time = 0.0;
                            if audio_loaded {
                                if let Some(eng) = audio_sync.borrow().as_ref() {
                                    eng.seek(0.0);
                                    let _ = eng.play();
                                }
                            }
                        } else {
                            s.current_time = dur;
                            s.is_playing = false;
                            if audio_loaded {
                                if let Some(eng) = audio_sync.borrow().as_ref() {
                                    let _ = eng.pause();
                                    eng.seek(0.0);
                                }
                            }
                        }
                    } else {
                        // Gap-skip: if current time is in a gap between workstreams, jump to next
                        let skipped = s.next_active_time(next);
                        s.current_time = skipped;
                    }
                }
            }
        }
    });

    rsx! {
        div {
            id: "app-root",
            style: "position: absolute; inset: 0; display: flex; flex-direction: column; background: #0d0d12; color: #fff; overflow: hidden; transform-origin: top left; transform: scale({state.read().ui_scale}); width: calc(100% / {state.read().ui_scale}); height: calc(100% / {state.read().ui_scale});",
            onpointermove: move |evt| {
                let mut s = state.write();
                // Track cursor for sidebar drag ghost
                if s.drag.source_id.is_some() && !s.drag.is_canvas_drag {
                    s.drag.cursor_x = evt.client_coordinates().x;
                    s.drag.cursor_y = evt.client_coordinates().y;
                }
                if let Some(panel) = s.resizing_panel.clone() {
                    let cx = evt.client_coordinates().x;
                    let cy = evt.client_coordinates().y;
                    let scale = s.ui_scale;
                    match panel.as_str() {
                        "left" => {
                            let new_width = cx / scale;
                            s.left_panel_width = new_width.clamp(150.0, 800.0);
                        },
                        "right" => {
                            // Calculate width back from the right edge
                            if let Some(window) = web_sys::window() {
                                if let Ok(inner_width) = window.inner_width() {
                                    if let Some(w_px) = inner_width.as_f64() {
                                        let new_width = (w_px - cx) / scale;
                                        s.right_panel_width = new_width.clamp(180.0, 600.0);
                                    }
                                }
                            }
                        },
                        "bottom" => {
                            if let Some(window) = web_sys::window() {
                                if let Ok(inner_height) = window.inner_height() {
                                    if let Some(h_px) = inner_height.as_f64() {
                                        // Take terminal height into account if it's open (110px). Let's just use raw Cy to bottom.
                                        let mut new_height = (h_px - cy) / scale;
                                        if s.show_terminal {
                                            new_height -= 110.0;
                                        }
                                        s.bottom_panel_height = new_height.clamp(120.0, 800.0);
                                    }
                                }
                            }
                        },
                        _ => {}
                    }
                }
            },
            onpointerup: move |_| {
                let mut s = state.write();
                s.drag.source_id = None;
                s.drag.hover_target_id = None;
                s.resizing_panel = None;
                s.end_clip_drag();
            },
            onmouseleave: move |_| {
                let mut s = state.write();
                s.resizing_panel = None;
            },

            // ── Top Menu Bar ──
            div {
                style: "height: 32px; flex-shrink: 0; background: #09090e; border-bottom: 1px solid rgba(255,255,255,0.05); display: flex; align-items: center; justify-content: space-between; padding: 0 12px; user-select: none; z-index: 20;",
                div {
                    style: "display: flex; align-items: center; gap: 6px;",
                    div { style: "width: 8px; height: 8px; border-radius: 50%; background: var(--accent-color); box-shadow: 0 0 8px var(--accent-color);" }
                    span { style: "font-weight: 700; letter-spacing: 0.1em; font-size: 10px; text-transform: uppercase; opacity: 0.9;", "Vibe Studio" }
                }
                div {
                    style: "display: flex; align-items: center; gap: 12px;",
                    {
                        let is_settings_open = state.read().show_settings;
                        let cog_color = if is_settings_open { "#fff" } else { "rgba(255,255,255,0.4)" };
                        rsx! {
                            button {
                                style: "background: none; border: none; color: {cog_color}; cursor: pointer; display: flex; align-items: center; justify-content: center; font-size: 14px; transition: color 0.2s;",
                                onclick: move |_| {
                                    let mut s = state.write();
                                    s.show_settings = !s.show_settings;
                                },
                                "⚙"
                            }
                        }
                    }
                }
            }

            // Settings Overlay
            if state.read().show_settings {
                SettingsPanel {}
            }

            // ── Main Workspace ──
            div {
                style: "display: flex; flex-grow: 1; overflow: hidden; min-height: 0;",
                Sidebar {}
                
                // Left Resizer
                div {
                    style: "width: 4px; background: rgba(255,255,255,0.02); cursor: col-resize; z-index: 50; flex-shrink: 0; transition: background 0.2s;",
                    onmouseenter: move |_| { let _ = js_sys::eval("event.target.style.background = 'rgba(123,97,255,0.5)'"); },
                    onmouseleave: move |_| { let _ = js_sys::eval("event.target.style.background = 'rgba(255,255,255,0.02)'"); },
                    onpointerdown: move |_| { state.write().resizing_panel = Some("left".to_string()); }
                }

                div {
                    style: "display: flex; flex-direction: column; flex-grow: 1; min-width: 0; overflow: hidden;",
                    CanvasArea {}
                }

                // Right Resizer
                div {
                    style: "width: 4px; background: rgba(255,255,255,0.02); cursor: col-resize; z-index: 50; flex-shrink: 0; transition: background 0.2s;",
                    onmouseenter: move |_| { let _ = js_sys::eval("event.target.style.background = 'rgba(123,97,255,0.5)'"); },
                    onmouseleave: move |_| { let _ = js_sys::eval("event.target.style.background = 'rgba(255,255,255,0.02)'"); },
                    onpointerdown: move |_| { state.write().resizing_panel = Some("right".to_string()); }
                }

                Inspector {}
            }

            // Bottom Resizer
            div {
                style: "height: 4px; background: rgba(255,255,255,0.02); cursor: row-resize; z-index: 50; flex-shrink: 0; transition: background 0.2s;",
                onmouseenter: move |_| { let _ = js_sys::eval("event.target.style.background = 'rgba(123,97,255,0.5)'"); },
                onmouseleave: move |_| { let _ = js_sys::eval("event.target.style.background = 'rgba(255,255,255,0.02)'"); },
                onpointerdown: move |_| { state.write().resizing_panel = Some("bottom".to_string()); }
            }

            // ── Timeline ──
            Timeline {}

            if state.read().show_terminal {
                // ── Terminal (full-width; presets/settings as typed commands) ──
                div {
                    style: "height: 110px; flex-shrink: 0; background: #020205; border-top: 1px solid rgba(255,255,255,0.06); display: flex; flex-direction: column; overflow: hidden;",

                    div {
                        style: "flex-grow: 1; overflow-y: auto; padding: 5px 12px; font-family: 'JetBrains Mono', 'Fira Code', monospace; font-size: 10px; line-height: 1.7; display: flex; flex-direction: column; justify-content: flex-end;",
                        for log in state.read().terminal_logs.iter() {
                            {
                                let is_sys = log.starts_with("> [SYSTEM]") || log.starts_with("[INFO]");
                                let is_cmd = log.starts_with("> ") && !is_sys;
                                let color = if is_sys { "#fbbf24" } else if is_cmd { "#a78bfa" } else { "#4ade80" };
                                rsx! { div { style: "color: {color};", "{log}" } }
                            }
                        }
                    }

                    div {
                        style: "display: flex; align-items: center; border-top: 1px solid rgba(34,197,94,0.12); padding: 3px 12px; gap: 6px; flex-shrink: 0; background: #050508;",
                        span { style: "color: #4ade80; font-size: 10px; font-family: monospace; flex-shrink: 0; opacity: 0.7;", "~$" }
                        input {
                            style: "flex-grow: 1; background: transparent; outline: none; border: none; color: #86efac; font-size: 11px; font-family: 'JetBrains Mono', monospace;",
                            placeholder: "help | settings | preset clean|dreamy|intense | bloom <0-2> | clear",
                            value: "{terminal_input}",
                            oninput: move |evt| terminal_input.set(evt.value().clone()),
                            onkeydown: move |evt| {
                                if evt.key() == Key::Enter {
                                    let cmd = terminal_input.read().trim().to_string();
                                    if !cmd.is_empty() {
                                        state.write().log_terminal(&format!("> {}", cmd));
                                        let parts: Vec<&str> = cmd.splitn(2, ' ').collect();
                                        match parts[0] {
                                            "help" => {
                                                state.write().log_terminal("[INFO] Available commands:");
                                                state.write().log_terminal("[INFO]   current          — list all active layers and global settings");
                                                state.write().log_terminal("[INFO]   settings         — show current global settings");
                                                state.write().log_terminal("[INFO]   preset clean     — apply Clean preset (bloom 0.15)");
                                                state.write().log_terminal("[INFO]   preset dreamy    — apply Dreamy preset (bloom 0.85)");
                                                state.write().log_terminal("[INFO]   preset intense   — apply Intense preset (bloom 2.0)");
                                                state.write().log_terminal("[INFO]   bloom <0.0-2.0>  — set bloom strength directly");
                                                state.write().log_terminal("[INFO]   clear            — clear terminal");
                                            }
                                            "clear" => {
                                                state.write().terminal_logs.clear();
                                            }
                                            "current" => {
                                                let mut logs = vec!["[INFO] === Current Active Scene ===".to_string()];
                                                let s = state.read();
                                                logs.push(format!("[INFO] Global Bloom: {:.2}", s.global_bloom));
                                                logs.push(format!("[INFO] Master Volume: {:.0}%", s.master_volume * 100.0));
                                                logs.push("[INFO] Layers:".to_string());
                                                for l in &s.layers {
                                                    if l.layer_type != LayerType::Composition && l.layer_type != LayerType::Workstream {
                                                        let vis = if l.visible { "ON " } else { "OFF" };
                                                        logs.push(format!("[INFO]   [{}] {} ({:?}) | {} | opac:{:.2} scale:{:.2} pos:[{:.1}, {:.1}]", 
                                                            l.id.chars().take(6).collect::<String>(), l.name, l.layer_type, vis, l.opacity, l.scale, l.position[0], l.position[1]));
                                                    }
                                                }
                                                drop(s);
                                                let mut s_write = state.write();
                                                for msg in logs {
                                                    s_write.log_terminal(&msg);
                                                }
                                            }
                                            "settings" => {
                                                let bloom = state.read().global_bloom;
                                                state.write().log_terminal(&format!("[INFO] bloom:         {:.2}", bloom));
                                                state.write().log_terminal("[INFO] master_volume: 1.00");
                                            }
                                            "preset" => {
                                                match parts.get(1).copied().unwrap_or("") {
                                                    "clean"   => { state.write().set_global_bloom(0.15); }
                                                    "dreamy"  => { state.write().set_global_bloom(0.85); }
                                                    "intense" => { state.write().set_global_bloom(2.00); }
                                                    other => { state.write().log_terminal(&format!("[INFO] Unknown preset: {}. Try clean|dreamy|intense", other)); }
                                                }
                                            }
                                            "bloom" => {
                                                if let Some(val_str) = parts.get(1) {
                                                    if let Ok(v) = val_str.parse::<f64>() {
                                                        let v = v.clamp(0.0, 2.0);
                                                        state.write().set_global_bloom(v);
                                                    } else {
                                                        state.write().log_terminal("[INFO] Usage: bloom <0.0-2.0>");
                                                    }
                                                }
                                            }
                                            other => {
                                                state.write().log_terminal(&format!("[INFO] Unknown command: {}  (type 'help')", other));
                                            }
                                        }
                                    }
                                    terminal_input.set(String::new());
                                }
                            }
                        }
                    }
                }
            } // end show_terminal

            // Drag Indicator Overlay
            {
                let s = state.read();
                let drag_source = s.drag.source_id.clone();
                let drag_pos = if s.drag.is_canvas_drag {
                    s.drag.last_pos.clone()
                } else if drag_source.is_some() {
                    Some((s.drag.cursor_x, s.drag.cursor_y))
                } else {
                    None
                };
                
                let mut name = "Layer".to_string();
                let mut icon = "📄";
                if let Some(source_id) = &drag_source {
                    if source_id.starts_with("asset:") {
                        let id = source_id.trim_start_matches("asset:");
                        if let Some(asset) = s.project_assets.iter().find(|a| a.id == id) {
                            name = asset.name.clone();
                            icon = match asset.asset_type.as_str() {
                                "video" => "🎬",
                                "audio" => "🎵",
                                _ => "🖼️"
                            };
                        }
                    } else if let Some(layer) = s.layers.iter().find(|l| l.id == *source_id) {
                        name = layer.name.clone();
                        icon = match layer.layer_type {
                            LayerType::Composition => "📁",
                            LayerType::Workstream => "🌊",
                            LayerType::Audio => "🎵",
                            LayerType::Video => "🎬",
                            _ => "📄"
                        };
                    }
                }
                
                if let (Some((x, y)), Some(_)) = (drag_pos, drag_source) {
                    rsx! {
                        div {
                            style: "position: absolute; left: {x}px; top: {y}px; transform: translate(14px, 14px); pointer-events: none; z-index: 9999; background: rgba(123,97,255,0.95); padding: 5px 12px; border-radius: 6px; border: 1px solid rgba(255,255,255,0.5); font-size: 11px; font-weight: 500; display: flex; align-items: center; gap: 6px; box-shadow: 0 4px 16px rgba(0,0,0,0.6); backdrop-filter: blur(4px);",
                            span { style: "font-size: 12px;", "{icon}" }
                            span { style: "color: #fff;", "{name}" }
                        }
                    }
                } else {
                    rsx! { div { style: "display: none;" } }
                }
            }
        }
    }
}

