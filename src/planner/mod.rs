// Planner module - Build Planner
// VAIXLNS Builder v0.1

use std::collections::HashMap;
use crate::ir::{CanonicalId, CanonicalSpec, DependencyGraph};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PhaseKind {
    Constitution,
    Kernel,
    Graph,
    Runtime,
    Verification,
    Release,
}

impl std::fmt::Display for PhaseKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PhaseKind::Constitution => write!(f, "Constitution"),
            PhaseKind::Kernel => write!(f, "Kernel"),
            PhaseKind::Graph => write!(f, "Graph"),
            PhaseKind::Runtime => write!(f, "Runtime"),
            PhaseKind::Verification => write!(f, "Verification"),
            PhaseKind::Release => write!(f, "Release"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BuildPhase {
    pub index: u32,
    pub kind: PhaseKind,
    pub nodes: Vec<CanonicalId>,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct BuildPlan {
    pub phases: Vec<BuildPhase>,
    pub total_nodes: usize,
    pub spec_hash: String,
}

impl BuildPlan {
    pub fn new() -> Self {
        Self {
            phases: Vec::new(),
            total_nodes: 0,
            spec_hash: String::new(),
        }
    }

    pub fn create(_spec: &CanonicalSpec, _graph: &DependencyGraph, sorted: &[CanonicalId]) -> Self {
        let mut plan = BuildPlan::new();
        
        if sorted.is_empty() {
            return plan;
        }
        
        let mut phase_map: HashMap<PhaseKind, Vec<CanonicalId>> = HashMap::new();
        
        for node in sorted {
            let kind = Self::determine_phase(node);
            phase_map.entry(kind).or_insert_with(Vec::new).push(node.clone());
        }
        
        let phase_order = vec![
            PhaseKind::Constitution,
            PhaseKind::Kernel,
            PhaseKind::Graph,
            PhaseKind::Runtime,
            PhaseKind::Verification,
            PhaseKind::Release,
        ];
        
        let mut index = 0;
        for kind in phase_order {
            if let Some(nodes) = phase_map.remove(&kind) {
                let description = format!("Phase {}: {}", index, kind);
                plan.phases.push(BuildPhase {
                    index,
                    kind,
                    nodes,
                    description,
                });
                index += 1;
            }
        }
        
        for (kind, nodes) in phase_map {
            let description = format!("Phase {}: {}", index, kind);
            plan.phases.push(BuildPhase {
                index,
                kind,
                nodes,
                description,
            });
            index += 1;
        }
        
        plan.total_nodes = sorted.len();
        plan.spec_hash = format!("{:016x}", sorted.len() * 0xDEADBEEF);
        
        plan
    }

    fn determine_phase(node: &CanonicalId) -> PhaseKind {
        let id = node.0.to_lowercase();
        
        if id.contains("constitution") || id.contains("law") || id.contains("rule") {
            PhaseKind::Constitution
        } else if id.contains("kernel") || id.contains("core") || id.contains("identity") {
            PhaseKind::Kernel
        } else if id.contains("graph") || id.contains("dependency") || id.contains("relation") {
            PhaseKind::Graph
        } else if id.contains("runtime") || id.contains("engine") || id.contains("execution") {
            PhaseKind::Runtime
        } else if id.contains("verify") || id.contains("proof") || id.contains("validate") {
            PhaseKind::Verification
        } else {
            PhaseKind::Release
        }
    }

    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        
        if self.phases.is_empty() {
            errors.push("Build plan has no phases".to_string());
            return Err(errors);
        }
        
        for phase in &self.phases {
            if phase.nodes.is_empty() {
                errors.push(format!("Phase {} has no nodes", phase.index));
            }
        }
        
        let total_planned: usize = self.phases.iter().map(|p| p.nodes.len()).sum();
        if total_planned != self.total_nodes {
            errors.push(format!(
                "Node count mismatch: planned {}, expected {}",
                total_planned, self.total_nodes
            ));
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    pub fn get_phase(&self, index: u32) -> Option<&BuildPhase> {
        self.phases.iter().find(|p| p.index == index)
    }

    pub fn get_phase_by_kind(&self, kind: PhaseKind) -> Option<&BuildPhase> {
        self.phases.iter().find(|p| p.kind == kind)
    }

    pub fn nodes_in_phase(&self, index: u32) -> Vec<&CanonicalId> {
        self.phases
            .iter()
            .find(|p| p.index == index)
            .map(|p| p.nodes.iter().collect())
            .unwrap_or_default()
    }

    pub fn pretty_print(&self) -> String {
        let mut output = String::new();
        output.push_str("╔════════════════════════════════════════════╗\n");
        output.push_str("║           BUILD PLAN                     ║\n");
        output.push_str("╚════════════════════════════════════════════╝\n\n");
        
        output.push_str(&format!("Total nodes: {}\n", self.total_nodes));
        output.push_str(&format!("Total phases: {}\n\n", self.phases.len()));
        
        for phase in &self.phases {
            output.push_str(&format!("Phase {}: {}\n", phase.index, phase.kind));
            output.push_str(&format!("  Description: {}\n", phase.description));
            output.push_str(&format!("  Nodes: {}\n", phase.nodes.len()));
            for (i, node) in phase.nodes.iter().enumerate() {
                output.push_str(&format!("    {}. {}\n", i + 1, node));
            }
            output.push_str("\n");
        }
        
        output
    }
}
