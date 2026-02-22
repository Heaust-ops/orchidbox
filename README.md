# orchidbox

Control your PC with a gamepad using a CSS-inspired config syntax for button mapping.

## Config

Mappings are defined in a CSS-like config file. The universal selector `*` defines global bindings, and class selectors target specific active windows by their WM class name.

```css
* {
  rb: "mouse";
  rb-y: "click";
  rb-a: "rightclick";
  rb-rs: "middleclick";
  rb-lt: "ctrl+c";
  rb-rt: "ctrl+v";
  rb-lb: "alt+tab";
  rb-right: "meta+ctrl+right";
  rb-left: "meta+ctrl+left";
  rb-lb-right: "meta+ctrl+shift+right";
  rb-lb-left: "meta+ctrl+shift+left";
  rb-x: "@voiceprint+/home/user/whisper/ggml/ggml-distil-large-v3.bin";
}

.firefox,
.Chromium-browser,
.firefox-nightly {
  lt: "ctrl+shift+tab";
  rt: "ctrl+tab";
  a: "ctrl+t";
  x: "ctrl+w";
  all: "@browser";
}
```

### Selectors

| Selector     | Behavior                                                |
| ------------ | ------------------------------------------------------- |
| `*`          | Global fallback, applies when no class selector matches |
| `.ClassName` | Applies when the active window's WM class matches       |
| `.A, .B`     | Applies to multiple window classes                      |

### Button Names

Buttons are referenced by short names:

| Name                          | Button       |
| ----------------------------- | ------------ |
| `a`, `b`, `x`, `y`            | Face buttons |
| `lb`, `rb`                    | Bumpers      |
| `lt`, `rt`                    | Triggers     |
| `up`, `down`, `left`, `right` | D-Pad        |
| `ls`, `rs`                    | Stick clicks |
| `start`, `select`             | Menu buttons |

Chords are written with `-` between buttons, e.g. `lb-right` means LB held while pressing right.

### Actions

Actions are either key combos or plugins. Plugins are prefixed with `@` and can accept arguments separated by `+`.

| Value           | Behavior                        |
| --------------- | ------------------------------- |
| `"key+combo"`   | Send a keyboard shortcut        |
| `"click"`       | Left mouse click                |
| `"rightclick"`  | Right mouse click               |
| `"middleclick"` | Middle mouse click              |
| `"mouse"`       | Use the stick as a mouse cursor |

### Plugins

Plugins extend orchidbox with additional functionality and are invoked with the `@` prefix.

| Plugin        | Example                            | Behavior                                                |
| ------------- | ---------------------------------- | ------------------------------------------------------- |
| `@browser`    | `"@browser"`                       | Enable browser-specific built-in bindings               |
| `@voiceprint` | `"@voiceprint+/path/to/model.bin"` | Start voice transcription using the given Whisper model |

## Usage

```
orchidbox [OPTIONS] --config <CONFIG>
```

### Options

| Flag                    | Description              | Default      |
| ----------------------- | ------------------------ | ------------ |
| `-c, --config <CONFIG>` | Path to your config file | _(required)_ |
| `-h, --help`            | Print help               |              |
| `-V, --version`         | Print version            |              |

### Example

```bash
orchidbox --config ~/.config/orchidbox/map.css
```

## Config Location

By convention, place your config at:

```
~/.config/orchidbox/map.css
```
