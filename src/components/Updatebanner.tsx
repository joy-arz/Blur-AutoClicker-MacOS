import "./UpdateBanner.css";

interface UpdateBannerProps {
  currentVersion: string;
  latestVersion: string;
}

export default function UpdateBanner({
  currentVersion,
  latestVersion,
}: UpdateBannerProps) {
  const handleUpdate = async () => {
    console.log("[Updates] Update download not implemented on macOS");
  };

  return (
    <div className="update-banner">
      <span className="update-banner-text-old-version">v{currentVersion}</span>
      <span className="update-banner-text">→</span>
      <span className="update-banner-text-new-version">{latestVersion}</span>
      <button className="update-banner-btn" onClick={handleUpdate}>
        Download & Install Update
      </button>
    </div>
  );
}
