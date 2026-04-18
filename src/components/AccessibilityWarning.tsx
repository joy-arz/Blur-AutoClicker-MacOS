import { useEffect, useState } from "react";
import "./AccessibilityWarning.css";
import { checkAccessibilityPermission, openAccessibilitySettings } from "../store";

interface Props {
  onGranted?: () => void;
}

export default function AccessibilityWarning({ onGranted }: Props) {
  const [checking, setChecking] = useState(false);

  const handleOpenSettings = async () => {
    await openAccessibilitySettings();
    setChecking(true);
  };

  useEffect(() => {
    if (!checking) return;

    const interval = setInterval(async () => {
      const granted = await checkAccessibilityPermission();
      if (granted) {
        setChecking(false);
        onGranted?.();
      }
    }, 2000);

    return () => clearInterval(interval);
  }, [checking, onGranted]);

  return (
    <div className="accessibility-warning">
      <div className="accessibility-warning-icon">
        <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
          <path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z" />
          <line x1="12" y1="9" x2="12" y2="13" />
          <line x1="12" y1="17" x2="12.01" y2="17" />
        </svg>
      </div>
      <div className="accessibility-warning-content">
        <h3>Accessibility Permission Required</h3>
        <p>
          This app needs Accessibility access to control the mouse and simulate clicks.
          Without it, the auto-clicker will not work.
        </p>
        <div className="accessibility-warning-actions">
          <button onClick={handleOpenSettings} className="accessibility-warning-btn">
            Open System Settings
          </button>
          <span className="accessibility-warning-note">
            Add BlurAutoClicker to the list and enable it
          </span>
          {checking && (
            <span className="accessibility-warning-note" style={{ marginTop: 8 }}>
              Checking for permission...
            </span>
          )}
        </div>
      </div>
    </div>
  );
}