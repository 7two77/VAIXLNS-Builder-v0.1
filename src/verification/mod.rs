// Verification module - Formal Proof Verification
// VAIXLNS Builder v0.1

use std::process::Command;

pub struct LeanVerifier;

impl LeanVerifier {
    pub fn new() -> Self {
        Self
    }

    pub fn verify_proofs(&self, proof_dir: &str) -> Result<bool, String> {
        // Проверяем, что Lean установлен
        let lean_check = Command::new("lean")
            .arg("--version")
            .output()
            .map_err(|e| format!("Lean not found: {}", e))?;

        if !lean_check.status.success() {
            return Err("Lean is not installed or not in PATH".to_string());
        }

        // Проверяем lake build
        let status = Command::new("lake")
            .current_dir(proof_dir)
            .arg("build")
            .status()
            .map_err(|e| format!("Failed to run lake build: {}", e))?;

        Ok(status.success())
    }

    pub fn verify_single_proof(&self, proof_file: &str) -> Result<bool, String> {
        // Проверяем, что Lean установлен
        let lean_check = Command::new("lean")
            .arg("--version")
            .output()
            .map_err(|e| format!("Lean not found: {}", e))?;

        if !lean_check.status.success() {
            return Err("Lean is not installed or not in PATH".to_string());
        }

        // Проверяем существование файла
        if !std::path::Path::new(proof_file).exists() {
            return Err(format!("Proof file not found: {}", proof_file));
        }

        // Пытаемся проверить через lean
        let status = Command::new("lean")
            .arg(proof_file)
            .arg("--trust=0")
            .status()
            .map_err(|e| format!("Failed to run Lean: {}", e))?;

        Ok(status.success())
    }
}
