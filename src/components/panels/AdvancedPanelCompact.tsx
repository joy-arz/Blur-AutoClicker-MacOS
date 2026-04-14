import type { Settings } from "../../store";
import AdvancedPanelLayout from "./AdvancedPanelLayout";

interface Props {
  settings: Settings;
  update: (patch: Partial<Settings>) => void;
  onPickPosition: () => Promise<void>;
}

export default function AdvancedPanelCompact({
  settings,
  update,
  onPickPosition,
}: Props) {
  return (
    <AdvancedPanelLayout
      settings={settings}
      update={update}
      onPickPosition={onPickPosition}
      compact
      showExplanations={false}
    />
  );
}
