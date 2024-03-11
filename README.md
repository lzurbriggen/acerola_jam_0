Unlicensed

Created within 2 weeks for the [Acerola Jam 0](https://itch.io/jam/acerola-jam-0)

## Controls

### Gamepad

| Action             | PlayStation | Xbox        |
| ------------------ | ----------- | ----------- |
| Move               | Left Stick  | Left Stick  |
| Aim                | Right Stick | Right Stick |
| Shoot/Dash/Confirm | X           | A           |
| Settings           | Start       | Start       |

### Keyboard

| Action             | Keys       | Mouse          |
| ------------------ | ---------- | -------------- |
| Move               | WASD       |                |
| Aim                | Arrow keys | Mouse movement |
| Shoot/Dash/Confirm | E, Space   | Left Click     |
| Settings           | Esc        |                |

## Assets used

- Fonts: https://nimblebeastscollective.itch.io/nb-pixel-font-bundle
- Sfx: https://mikiz.itch.io/mini-40-sounds-pack
- Sfx: https://kenney.nl/assets/rpg-audio
- Sfx: https://kenney.nl/assets/impact-sounds
- Color palette: https://lospec.com/palette-list/aap-64
- Noise texture: https://screamingbrainstudios.itch.io/noise-texture-pack

## Known bugs

- Gamepad does not work in the Web build

## Dev

### Build WASM

```sh
cargo build --target wasm32-unknown-unknown -r
basic-http-server .
```
