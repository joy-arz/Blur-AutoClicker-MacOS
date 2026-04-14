use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;

static STATS_LOCK: Mutex<()> = Mutex::new(());

const MAX_NORMAL_RUNS: usize = 100;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CumulativeStats {
    pub total_clicks: i64,
    pub total_time_secs: f64,
    pub total_sessions: i64,
    pub avg_cpu: f64,
}

#[derive(Debug, Clone)]
pub struct RunRecord {
    pub id: u64,
    pub clicks: i64,
    pub time_secs: f64,
    pub avg_cpu: f64,
    pub sent: bool,
    pub runs: u32,
    pub telemetry_enabled: bool,
    pub hash: String,
}

fn stats_file_path() -> PathBuf {
    let app_data = dirs::data_dir().unwrap_or_else(|| PathBuf::from("."));
    app_data
        .join("com.blur009.blurautoclicker")
        .join("stats.csv")
}

fn round2(v: f64) -> f64 {
    (v * 100.0).round() / 100.0
}

// -- Row hashing (HMAC-SHA256) --

fn get_signing_key() -> &'static str {
    // On macOS, we use a compiled-in key for HMAC verification
    // The actual key is not critical since stats are local-only
    "blur-autoclicker-stats-key-v1"
}

fn compute_hmac(record: &RunRecord, _key: &[u8]) -> String {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    type HmacSha256 = Hmac<Sha256>;

    let data = format!(
        "{}|{}|{}|{}|{}|{}|{}",
        record.id,
        record.clicks,
        record.time_secs,
        record.avg_cpu,
        record.sent,
        record.runs,
        record.telemetry_enabled
    );

    let key_bytes = get_signing_key().as_bytes();
    let mut mac = HmacSha256::new_from_slice(key_bytes).expect("HMAC can take key of any size");
    mac.update(data.as_bytes());
    let result = mac.finalize();
    hex::encode(result.into_bytes())
}

fn verify_hash(record: &RunRecord, key: &[u8]) -> bool {
    compute_hmac(record, key) == record.hash
}

// -- CSV read/write --

fn read_all_runs() -> Result<Vec<RunRecord>, String> {
    let path = stats_file_path();
    if !path.exists() {
        return Ok(Vec::new());
    }
    let contents =
        fs::read_to_string(&path).map_err(|e| format!("Failed to read stats file: {}", e))?;

    let key = get_signing_key().as_bytes().to_vec();
    let mut runs = Vec::new();
    let mut invalid_indices: Vec<usize> = Vec::new();

    for line in contents.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() < 5 {
            continue;
        }
        if parts[0] == "id" {
            continue;
        }

        let id: u64 = match parts[0].parse() {
            Ok(v) => v,
            Err(_) => continue,
        };
        let clicks: i64 = match parts[1].parse() {
            Ok(v) => v,
            Err(_) => continue,
        };
        let time_secs: f64 = match parts[2].parse() {
            Ok(v) => v,
            Err(_) => continue,
        };
        let avg_cpu: f64 = match parts[3].parse() {
            Ok(v) => v,
            Err(_) => continue,
        };

        let sent = parts.get(4).map(|s| *s == "1").unwrap_or(false);
        let runs_val: u32 = parts.get(5).and_then(|s| s.parse().ok()).unwrap_or(1);
        let telemetry_enabled = parts.get(6).map(|s| *s == "1").unwrap_or(false);
        let hash = parts.get(7).map(|s| s.to_string()).unwrap_or_default();

        let record = RunRecord {
            id,
            clicks,
            time_secs,
            avg_cpu,
            sent,
            runs: runs_val,
            telemetry_enabled,
            hash,
        };

        if !verify_hash(&record, &key) {
            invalid_indices.push(runs.len());
        }

        runs.push(record);
    }

    if !invalid_indices.is_empty() {
        log::warn!(
            "[Stats] {} records failed HMAC verification (may be tampered or old)",
            invalid_indices.len()
        );
    }

    Ok(runs)
}

fn write_all_runs(runs: &[RunRecord]) -> Result<(), String> {
    let path = stats_file_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create dir: {}", e))?;
    }

    let mut file = fs::File::create(&path).map_err(|e| format!("Failed to create file: {}", e))?;

    writeln!(file, "id,clicks,time_secs,avg_cpu,sent,runs,telemetry_enabled,hash").map_err(|e| format!("Write error: {}", e))?;

    for record in runs {
        writeln!(
            file,
            "{},{},{},{},{},{},{},{}",
            record.id,
            record.clicks,
            record.time_secs,
            record.avg_cpu,
            if record.sent { "1" } else { "0" },
            record.runs,
            if record.telemetry_enabled { "1" } else { "0" },
            record.hash
        ).map_err(|e| format!("Write error: {}", e))?;
    }

    Ok(())
}

// -- Public API --

pub fn record_run(clicks: i64, time_secs: f64, avg_cpu: f64, telemetry_enabled: bool) {
    let _guard = STATS_LOCK.lock().ok();

    let mut runs = read_all_runs().unwrap_or_default();

    let id = runs.len() as u64 + 1;
    let record = RunRecord {
        id,
        clicks,
        time_secs,
        avg_cpu,
        sent: false,
        runs: 1,
        telemetry_enabled,
        hash: String::new(),
    };

    let mut record_with_hash = record.clone();
    record_with_hash.hash = compute_hmac(&record, get_signing_key().as_bytes());

    runs.push(record_with_hash);

    if runs.len() > MAX_NORMAL_RUNS * 2 {
        runs = runs.split_off(runs.len() - MAX_NORMAL_RUNS);
    }

    let _ = write_all_runs(&runs);
}

pub fn print_run_stats(clicks: i64, time_secs: f64, avg_cpu: f64) {
    log::info!(
        "[Run] clicks={} time={:.2}s avg_cpu={:.2}",
        clicks,
        time_secs,
        avg_cpu
    );
}

pub fn get_stats() -> Result<CumulativeStats, String> {
    let _guard = STATS_LOCK.lock().map_err(|e| format!("Lock error: {}", e))?;

    let runs = read_all_runs()?;

    let total_clicks: i64 = runs.iter().map(|r| r.clicks).sum();
    let total_time_secs: f64 = runs.iter().map(|r| r.time_secs).sum();
    let total_sessions = runs.len() as i64;
    let avg_cpu = if runs.is_empty() {
        0.0
    } else {
        runs.iter().map(|r| r.avg_cpu).sum::<f64>() / runs.len() as f64
    };

    Ok(CumulativeStats {
        total_clicks,
        total_time_secs: round2(total_time_secs),
        total_sessions,
        avg_cpu: round2(avg_cpu),
    })
}

pub fn reset_stats() -> Result<CumulativeStats, String> {
    let _guard = STATS_LOCK.lock().map_err(|e| format!("Lock error: {}", e))?;

    let path = stats_file_path();
    if path.exists() {
        fs::remove_file(&path).map_err(|e| format!("Failed to delete stats: {}", e))?;
    }

    Ok(CumulativeStats {
        total_clicks: 0,
        total_time_secs: 0.0,
        total_sessions: 0,
        avg_cpu: 0.0,
    })
}
