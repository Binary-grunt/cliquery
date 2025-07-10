use std::collections::VecDeque;
use std::env;
use std::fs::{create_dir_all, read_to_string, write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

/// Représente une entrée de cache minimale
#[derive(Debug, Clone)]
pub struct CacheEntry {
    pub timestamp: u64,
    pub prompt: String,
    pub response: String,
}

/// Charge le cache depuis un fichier JSON simplifié
pub fn load_cache(path: &Path) -> VecDeque<CacheEntry> {
    if let Ok(content) = read_to_string(path) {
        let mut entries = VecDeque::new();
        for line in content.lines() {
            if let Some((prompt, response)) = line.split_once(" => ") {
                entries.push_back(CacheEntry {
                    timestamp: now_ts(),
                    prompt: prompt.to_string(),
                    response: response.to_string(),
                });
            }
        }
        return entries;
    }
    VecDeque::new()
}

/// Sauvegarde le cache dans un fichier simple ligne à ligne
pub fn save_cache(path: &Path, cache: &VecDeque<CacheEntry>) {
    let lines: Vec<String> = cache
        .iter()
        .map(|entry| format!("{} => {}", entry.prompt, entry.response))
        .collect();

    let data = lines.join("\n");
    if let Err(e) = write(path, data) {
        eprintln!("[Erreur] Écriture cache échouée : {}", e);
    }
}

/// Retourne un chemin de cache (fallback dans ~/.cache/cliquery/cache.txt)
pub fn get_cache_file() -> PathBuf {
    let mut home = env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/tmp"));
    home.push(".cache/cliquery/request_cache.txt");
    if let Some(parent) = home.parent() {
        let _ = create_dir_all(parent);
    }
    home
}

fn now_ts() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
