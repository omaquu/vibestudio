# Vibe Visualizer - DEVELOPMENT PLAN

Tämä suunnitelma kuvaa askeleet, joilla Vibe Visualizer muutetaan täysiveriseksi videoeditointityökaluksi, jossa on ammattimaiset visualisoinnit ja helppokäyttöiset efektit.

## 1. Arkkitehtuuri ja Aikajana (Timeline)

Nykyinen ohjelma on reaaliaikainen visualisoija. Muutos videoeditoriksi vaatii "Timeline-pohjaisen" ajattelun.

- **Globaali Aikajana:** Lisätään `startTime` ja `duration` jokaiselle layerille.
- **Layer-hallinta:** Tuki useille päällekkäisille layereille, jotka voivat alkaa ja loppua eri aikoina.
- **Keyframes:** Mahdollisuus animoida arvoja (esim. opacity tai scale) ajan funktiona (ei pelkästään audion mukaan).
- **Audio-trackit:** Tuki useammalle audio-raidalle tai audion leikkaamiselle.

## 2. Efektikirjasto (Effect Library)

Laajennetaan efektivalikoimaa "valmiilla ja helposti käytettävillä" palikoilla.

- **Transitionit:** Fade in/out, glitch transition, slide.
- **Uudet Visualisoinnit:**
    - `tunnel`: 3D-tunneli, joka reagoi bassoon.
    - `stars`: Syvyyssuunnassa liikkuvat tähdet.
    - `waveform-3d`: Maastoa muistuttava 3D-waveform.
- **Värikorjaus (Color Grading):** LUT-tuki tai valmiit preseti-filtterit (Vaporwave, Retro, Cyberpunk).
- **Tekstianimaatiot:** Typewriter-efekti, hiekkapuhallus, neon-vilkunta.

## 3. Käyttöliittymän Parannukset

Tehdään käyttöliittymästä ammattimaisempi ja sujuvampi.

- **Timeline UI:** Visuaalinen aikajana alareunaan, jossa näkyy blockit jokaiselle layerille.
- **Drag & Drop:** Layereiden siirtäminen aikajanalla.
- **Terminal 2.0:** Laajennettu komentokieli (esim. `add text "Hello" at 0:05 duration 10s`).
- **Asset Browser:** Esikatselukuvat ladatuille medioille.

## 4. Renderöinti ja Suorituskyky

- **WebWorker-renderöinti:** OffscreenCanvasin käyttö, jotta UI ei jäädy renderöinnin aikana.
- **Export-profiilit:** Valmiit asetukset (TikTok 9:16, YouTube 16:9, High Quality, Draft).
- **Caching:** Renderöityjen framejen cachetus esikatselun nopeuttamiseksi.

## 5. Tarkennettu Efekti-roadmap

| Efekti | Tyyppi | Tila |
| :--- | :--- | :--- |
| Audio Reactive Scale | Transform | ✅ Käytössä |
| Bloom / Glow | Post-proc | ✅ Käytössä |
| Chromatic Aberration | Post-proc | ✅ Käytössä |
| Glitch Layer | Layer | ✅ Käytössä (Perus) |
| RGB Split | Post-proc | ⏳ Suunniteltu |
| VHS / Retro Lines | Layer/Post | ⏳ Suunniteltu |
| Camera Shake | Camera | ⏳ Suunniteltu |
| Mirror / Kaleidoscope | Post-proc | ⏳ Suunniteltu |

---

*Päivitetty: 2026-02-22 - Sulo (The Builder)*
