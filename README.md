# orchidbox

Control your PC with a gamepad using a CSS-inspired config syntax for button remapping - with per-application overrides, plugin support, and service support.

---

## How It Works

orchidbox reads a CSS-like config file and maps gamepad inputs to keyboard shortcuts, mouse actions, or plugins. A global `*` block defines fallback bindings for all contexts, while class selectors (`.ClassName`) override those bindings when a specific window is active, identified by its WM class name.

---

## Config

Place your config at `./config.css` in the project root. See the example below for the full syntax.

### Example

```css
/* Plugin/service configuration */
#plugin.transcription.service {
  --model: "/home/user/whisper/ggml/ggml-distil-large-v3.bin";
  --host: "127.0.0.1";
  --port: "3277";
}

* {
  /* While holding RB → left analog stick moves mouse, right analog stick scrolls */
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

  /* Invoke a plugin */
  rb-x: "@voiceprint";
}

/* App-specific overrides: active when Firefox or Chromium is focused */
.firefox,
.Chromium-browser,
.firefox-nightly {
  lt: "ctrl+shift+tab";
  rt: "ctrl+tab";
  a: "ctrl+t";
  x: "ctrl+w";
}
```

---

## Selectors

| Selector     | Behavior                                                 |
| ------------ | -------------------------------------------------------- |
| `*`          | Global fallback - applies when no class selector matches |
| `.ClassName` | Applies when the active window's WM class matches        |
| `.A, .B`     | Applies to multiple window classes                       |

---

## Button Names

Buttons are referenced by short names. Chords are written with `-` between buttons - e.g. `rb-lt` means RB held while pressing LT.

| Name                          | Button       |
| ----------------------------- | ------------ |
| `a`, `b`, `x`, `y`            | Face buttons |
| `lb`, `rb`                    | Bumpers      |
| `lt`, `rt`                    | Triggers     |
| `up`, `down`, `left`, `right` | D-Pad        |
| `ls`, `rs`                    | Stick clicks |
| `start`, `select`             | Menu buttons |

---

## Actions

| Value           | Behavior                               |
| --------------- | -------------------------------------- |
| `"key+combo"`   | Send a keyboard shortcut               |
| `"click"`       | Left mouse click                       |
| `"rightclick"`  | Right mouse click                      |
| `"middleclick"` | Middle mouse click                     |
| `"mouse"`       | Use the analog stick as a mouse cursor |

---

## Plugins

Plugins extend orchidbox with additional functionality and are invoked with the `@` prefix. Plugin scripts can be shell scripts, JS files, or TypeScript files - orchidbox invokes the appropriate runtime based on the file extension. No extension runs the script directly, `.js` invokes it with `node`, and `.ts` invokes it with `ts-node`.

| Plugin        | Behavior                                                        |
| ------------- | --------------------------------------------------------------- |
| `@voiceprint` | Trigger voice transcription via a running transcription service |

### Plugin Arguments

There are two ways to pass arguments to a plugin.

**Inline** - append args with `+` directly in the binding value:

```css
rb-x: "@voiceprint+arg1+arg2";
```

This invokes `./voiceprint arg1 arg2`.

**Config block** - declare a `#plugin.<n>` block at the top of your config. Properties prefixed with `--` are forwarded as CLI flags:

If the plugin name ends in `.service`, orchidbox treats it as a long-running background service and starts it automatically at launch in its own thread.

```css
#plugin.transcription.service {
  --model: "/path/to/model.bin";
  --host: "127.0.0.1";
  --port: "3277";
}
```

This invokes `./transcription.service --model /path/to/model.bin --host 127.0.0.1 --port 3277`.

The transcription service used in the example above is a separate project: [transcription_server](https://github.com/Heaust-ops/transcription_server).

### Plugin Output

The stdout of any plugin invocation is interpreted as a command. A key combo like `ctrl+c` will be executed as a shortcut, and plain text will be typed out keystroke by keystroke. This is what allows the voiceprint plugin to type out transcribed speech — it simply prints the result to stdout.

---

## Running

Currently, orchidbox is run directly via Cargo from the project root. It will automatically read `./config.css`:

```sh
cargo run
```

---

## Future Work

- **Applet-specific configs** - overrides scoped not just to an application, but to a specific context within it. In a browser, this would mean tab-specific bindings; in a terminal, bindings scoped to the currently active process. Both would be inferred from the window title.

---

## Built With

- [Rust](https://www.rust-lang.org/)
