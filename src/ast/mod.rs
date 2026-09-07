// AST module - Abstract Syntax Tree
// VAIXLNS Builder v0.1

#[derive(Debug, Clone, PartialEq)]
pub enum Decl {
    Meta(MetaDecl),
    Domain(DomainDecl),
    Entity(EntityDecl),
    Relation(RelationDecl),
    Law(LawDecl),
    Constraint(ConstraintDecl),
    Capability(CapabilityDecl),
}

#[derive(Debug, Clone, PartialEq)]
pub struct MetaDecl {
    pub id: String,
    pub name: String,
    pub version: String,
    pub authority: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DomainDecl {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EntityDecl {
    pub id: String,
    pub domain: String,
    pub kind: String,
    pub provides: Vec<String>,
    pub requires: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RelationDecl {
    pub from: String,
    pub to: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LawDecl {
    pub category: String,
    pub rules: Vec<LawRule>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LawRule {
    pub id: String,
    pub description: String,
    pub enforce: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConstraintDecl {
    pub id: String,
    pub description: String,
    pub condition: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CapabilityDecl {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Document {
    pub decls: Vec<Decl>,
}

impl Document {
    pub fn new() -> Self {
        Self { decls: Vec::new() }
    }

    pub fn find_entities(&self) -> Vec<&EntityDecl> {
        self.decls.iter()
            .filter_map(|d| match d {
                Decl::Entity(e) => Some(e),
                _ => None,
            })
            .collect()
    }

    pub fn find_relations(&self) -> Vec<&RelationDecl> {
        self.decls.iter()
            .filter_map(|d| match d {
                Decl::Relation(r) => Some(r),
                _ => None,
            })
            .collect()
    }
}
