import "./Modes.css";
import type { Settings } from "../../store";
import HotkeyCaptureInput from "../HotkeyCaptureInput";

interface Props {
  settings: Settings;
  update: (patch: Partial<Settings>) => void;
}

const BUTTON_MAP = { L: "Left", M: "Middle", R: "Right" } as const;

export default function SimplePanel({ settings, update }: Props) {
  return (
    <div className="simple-panel">
      <div className="simple-row">
        <div className="simple-group">
          <span className="simple-label">Click Speed</span>
          <div className="simple-input-box">
            <input
              type="number"
              className="simple-number"
              value={settings.clickSpeed}
              min={1}
              max={2000}
              onChange={(e) => {
                const raw = e.target.value.replace(/^0+(?=\d)/, "");
                if (raw !== e.target.value) e.target.value = raw;
                update({ clickSpeed: raw === "" ? 0 : Number(raw) });
              }}
              style={{
                background: "transparent",
                border: "none",
                outline: "none",
                width: "48px",
              }}
            />
            <span className="simple-unit">cp{settings.clickInterval}</span>
          </div>
        </div>

        <div className="simple-group">
          <span className="simple-label">Button</span>
          <div className="simple-seg-group">
            {(["L", "M", "R"] as const).map((b) => (
              <button
                key={b}
                className={`simple-seg-btn ${settings.mouseButton === BUTTON_MAP[b] ? "active" : ""}`}
                onClick={() => update({ mouseButton: BUTTON_MAP[b] })}
              >
                {b}
              </button>
            ))}
          </div>
        </div>
      </div>

      <div className="simple-row">
        <div className="simple-group">
          <span className="simple-label">Hotkey</span>
          <div className="simple-input-box simple-hotkey">
            <HotkeyCaptureInput
              className="simple-hotkey-text"
              value={settings.hotkey}
              onChange={(hotkey) => update({ hotkey })}
              style={{
                background: "transparent",
                border: "none",
                outline: "none",
                width: "100px",
              }}
            />
          </div>
        </div>

        <div className="simple-group">
          <span className="simple-label">Mode</span>
          <div className="simple-seg-group">
            {(["Toggle", "Hold"] as const).map((m) => (
              <button
                key={m}
                className={`simple-seg-btn ${settings.mode === m ? "active" : ""}`}
                onClick={() => update({ mode: m })}
              >
                {m}
              </button>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}
