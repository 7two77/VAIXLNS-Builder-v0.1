// IR module - Canonical Intermediate Representation
// VAIXLNS Builder v0.1

use std::collections::{BTreeMap, HashSet, HashMap, VecDeque};
use crate::ast::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CanonicalId(pub String);

impl CanonicalId {
    pub fn new(id: &str) -> Self {
        Self(id.to_string())
    }
}

impl std::fmt::Display for CanonicalId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CanonicalEntity {
    pub id: CanonicalId,
    pub domain: String,
    pub kind: String,
    pub provides: Vec<String>,
    pub requires: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CanonicalRelation {
    pub from: CanonicalId,
    pub to: CanonicalId,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CanonicalSpec {
    pub meta: Option<MetaDecl>,
    pub entities: BTreeMap<CanonicalId, CanonicalEntity>,
    pub relations: Vec<CanonicalRelation>,
    pub laws: Vec<LawDecl>,
    pub constraints: Vec<ConstraintDecl>,
    pub capabilities: Vec<CapabilityDecl>,
}

impl CanonicalSpec {
    pub fn new() -> Self {
        Self {
            meta: None,
            entities: BTreeMap::new(),
            relations: Vec::new(),
            laws: Vec::new(),
            constraints: Vec::new(),
            capabilities: Vec::new(),
        }
    }

    pub fn from_ast(doc: &Document) -> Result<Self, String> {
        let mut spec = CanonicalSpec::new();
        
        for decl in &doc.decls {
            match decl {
                Decl::Meta(meta) => {
                    if spec.meta.is_some() {
                        return Err("Multiple meta declarations".to_string());
                    }
                    spec.meta = Some(meta.clone());
                }
                Decl::Entity(entity) => {
                    let id = CanonicalId::new(&entity.id);
                    if spec.entities.contains_key(&id) {
                        return Err(format!("Duplicate entity: {}", entity.id));
                    }
                    spec.entities.insert(id.clone(), CanonicalEntity {
                        id: id.clone(),
                        domain: entity.domain.clone(),
                        kind: entity.kind.clone(),
                        provides: entity.provides.clone(),
                        requires: entity.requires.clone(),
                    });
                }
                Decl::Relation(rel) => {
                    spec.relations.push(CanonicalRelation {
                        from: CanonicalId::new(&rel.from),
                        to: CanonicalId::new(&rel.to),
                    });
                }
                Decl::Law(law) => {
                    spec.laws.push(law.clone());
                }
                Decl::Constraint(constraint) => {
                    spec.constraints.push(constraint.clone());
                }
                Decl::Capability(capability) => {
                    spec.capabilities.push(capability.clone());
                }
                Decl::Domain(_) => {
                    continue;
                }
            }
        }
        
        spec.relations.sort();
        
        Ok(spec)
    }

    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        
        for rel in &self.relations {
            if !self.entities.contains_key(&rel.from) {
                errors.push(format!("Relation from {} references non-existent entity", rel.from));
            }
            if !self.entities.contains_key(&rel.to) {
                errors.push(format!("Relation to {} references non-existent entity", rel.to));
            }
        }
        
        for entity in self.entities.values() {
            for req in &entity.requires {
                let req_id = CanonicalId::new(req);
                if !self.entities.contains_key(&req_id) {
                    errors.push(format!(
                        "Entity {} requires {}, but it doesn't exist",
                        entity.id, req
                    ));
                }
            }
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    pub fn dependency_graph(&self) -> DependencyGraph {
        let mut graph = DependencyGraph::new();
        
        for entity in self.entities.values() {
            graph.add_node(entity.id.clone());
        }
        
        for entity in self.entities.values() {
            for req in &entity.requires {
                let dep_id = CanonicalId::new(req);
                graph.add_edge(entity.id.clone(), dep_id);
            }
        }
        
        for rel in &self.relations {
            graph.add_edge(rel.from.clone(), rel.to.clone());
        }
        
        graph
    }
}

#[derive(Debug, Clone)]
pub struct DependencyGraph {
    pub nodes: HashSet<CanonicalId>,
    pub edges: HashMap<CanonicalId, Vec<CanonicalId>>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashSet::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: CanonicalId) {
        self.nodes.insert(node);
    }

    pub fn add_edge(&mut self, from: CanonicalId, to: CanonicalId) {
        self.edges.entry(from).or_insert_with(Vec::new).push(to);
    }

    pub fn detect_cycles(&self) -> Option<Vec<CanonicalId>> {
        let mut visited = HashSet::new();
        let mut recursion_stack = HashSet::new();
        let mut cycle = Vec::new();
        
        for node in &self.nodes {
            if !visited.contains(node) {
                if self.dfs_detect_cycle(node, &mut visited, &mut recursion_stack, &mut cycle) {
                    cycle.reverse();
                    return Some(cycle);
                }
            }
        }
        
        None
    }

    fn dfs_detect_cycle(
        &self,
        node: &CanonicalId,
        visited: &mut HashSet<CanonicalId>,
        recursion_stack: &mut HashSet<CanonicalId>,
        cycle: &mut Vec<CanonicalId>,
    ) -> bool {
        visited.insert(node.clone());
        recursion_stack.insert(node.clone());
        
        if let Some(neighbors) = self.edges.get(node) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    if self.dfs_detect_cycle(neighbor, visited, recursion_stack, cycle) {
                        cycle.push(node.clone());
                        return true;
                    }
                } else if recursion_stack.contains(neighbor) {
                    cycle.push(neighbor.clone());
                    cycle.push(node.clone());
                    return true;
                }
            }
        }
        
        recursion_stack.remove(node);
        false
    }

    pub fn topological_sort(&self) -> Vec<CanonicalId> {
        let mut in_degree = HashMap::new();
        for node in &self.nodes {
            in_degree.insert(node.clone(), 0);
        }
        
        for (_from, tos) in &self.edges {
            for to in tos {
                *in_degree.entry(to.clone()).or_insert(0) += 1;
            }
        }
        
        let mut queue = VecDeque::new();
        for (node, degree) in &in_degree {
            if *degree == 0 {
                queue.push_back(node.clone());
            }
        }
        
        let mut result = Vec::new();
        while let Some(node) = queue.pop_front() {
            result.push(node.clone());
            if let Some(neighbors) = self.edges.get(&node) {
                for neighbor in neighbors {
                    let degree = in_degree.get_mut(neighbor).unwrap();
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push_back(neighbor.clone());
                    }
                }
            }
        }
        
        result
    }

    pub fn is_dag(&self) -> bool {
        self.detect_cycles().is_none()
    }
}
