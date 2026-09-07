// Evidence module - Build Evidence & Certification
// VAIXLNS Builder v0.1

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct Evidence {
    pub builder_version: String,
    pub timestamp: u64,
    pub spec_hash: String,
    pub ast_hash: String,
    pub ir_hash: String,
    pub plan_hash: String,
    pub artifact_hash: String,
    pub compiler_status: String,
    pub tests_passed: u32,
    pub tests_failed: u32,
    pub formal_proof_status: String,
    pub evidence_root: String,
    pub finality: String,
}

impl Evidence {
    pub fn new() -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        Self {
            builder_version: "0.1.0".to_string(),
            timestamp,
            spec_hash: String::new(),
            ast_hash: String::new(),
            ir_hash: String::new(),
            plan_hash: String::new(),
            artifact_hash: String::new(),
            compiler_status: "NOT_RUN".to_string(),
            tests_passed: 0,
            tests_failed: 0,
            formal_proof_status: "NOT_RUN".to_string(),
            evidence_root: String::new(),
            finality: "PENDING".to_string(),
        }
    }

    pub fn with_spec_hash(mut self, hash: &str) -> Self {
        self.spec_hash = hash.to_string();
        self
    }

    pub fn with_ast_hash(mut self, hash: &str) -> Self {
        self.ast_hash = hash.to_string();
        self
    }

    pub fn with_ir_hash(mut self, hash: &str) -> Self {
        self.ir_hash = hash.to_string();
        self
    }

    pub fn with_plan_hash(mut self, hash: &str) -> Self {
        self.plan_hash = hash.to_string();
        self
    }

    pub fn with_artifact_hash(mut self, hash: &str) -> Self {
        self.artifact_hash = hash.to_string();
        self
    }

    pub fn with_compiler_status(mut self, status: &str) -> Self {
        self.compiler_status = status.to_string();
        self
    }

    pub fn with_tests(mut self, passed: u32, failed: u32) -> Self {
        self.tests_passed = passed;
        self.tests_failed = failed;
        self
    }

    pub fn with_formal_proof(mut self, status: &str) -> Self {
        self.formal_proof_status = status.to_string();
        self
    }

    pub fn compute_combined_hash(&self) -> String {
        let mut hasher = DefaultHasher::new();
        self.spec_hash.hash(&mut hasher);
        self.ast_hash.hash(&mut hasher);
        self.ir_hash.hash(&mut hasher);
        self.plan_hash.hash(&mut hasher);
        self.artifact_hash.hash(&mut hasher);
        self.compiler_status.hash(&mut hasher);
        self.tests_passed.hash(&mut hasher);
        self.tests_failed.hash(&mut hasher);
        self.formal_proof_status.hash(&mut hasher);
        format!("{:016x}", hasher.finish())
    }

    pub fn finalize(&mut self) {
        self.evidence_root = self.compute_combined_hash();
        self.finality = "FINALIZED".to_string();
    }

    pub fn is_valid(&self) -> bool {
        self.finality == "FINALIZED"
            && !self.spec_hash.is_empty()
            && !self.ast_hash.is_empty()
            && !self.ir_hash.is_empty()
            && !self.plan_hash.is_empty()
            && !self.artifact_hash.is_empty()
            && self.compiler_status == "PASS"
            && self.tests_failed == 0
    }

    pub fn pretty_print(&self) -> String {
        let mut output = String::new();
        output.push_str("╔════════════════════════════════════════════╗\n");
        output.push_str("║           EVIDENCE REPORT                ║\n");
        output.push_str("╚════════════════════════════════════════════╝\n\n");
        
        output.push_str(&format!("Builder:      {}\n", self.builder_version));
        output.push_str(&format!("Timestamp:    {}\n", self.timestamp));
        output.push_str(&format!("Spec Hash:    {}\n", self.spec_hash));
        output.push_str(&format!("AST Hash:     {}\n", self.ast_hash));
        output.push_str(&format!("IR Hash:      {}\n", self.ir_hash));
        output.push_str(&format!("Plan Hash:    {}\n", self.plan_hash));
        output.push_str(&format!("Artifact Hash: {}\n", self.artifact_hash));
        output.push_str(&format!("Compiler:     {}\n", self.compiler_status));
        output.push_str(&format!("Tests:        {} passed, {} failed\n", self.tests_passed, self.tests_failed));
        output.push_str(&format!("Formal Proof: {}\n", self.formal_proof_status));
        output.push_str(&format!("Evidence Root: {}\n", self.evidence_root));
        output.push_str(&format!("Finality:     {}\n", self.finality));
        output.push_str("\n");
        
        output
    }
}

impl Default for Evidence {
    fn default() -> Self {
        Self::new()
    }
}
