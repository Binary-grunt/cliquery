// Description: Fonctions utilitaires réutilisables (horodatage, nettoyage, fallback, etc.)

use std::time::{SystemTime, UNIX_EPOCH};

/// Retourne un timestamp Unix actuel (secondes)
pub fn now_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// Nettoie un texte multi-lignes, supprime les blancs et réduit les sauts de ligne
pub fn clean_multiline(input: &str) -> String {
    input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

/// Vérifie si une ligne est un commentaire ou vide
pub fn is_ignorable_line(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with("//")
}
