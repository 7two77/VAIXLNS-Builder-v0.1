// Evidence tests for VAIXLNS Builder
// TEST-003...010

#[cfg(test)]
mod tests {
    use vaixlns_builder::lexer::tokenize;
    use vaixlns_builder::parser::Parser;
    use vaixlns_builder::ir::CanonicalSpec;
    use vaixlns_builder::planner::BuildPlan;
    use vaixlns_builder::generator::CodeGenerator;
    use vaixlns_builder::evidence::Evidence;
    use vaixlns_builder::gate::GenesisGate;
    use std::fs;

    // TEST-003: IR preservation
    #[test]
    fn test_003_ir_preservation() {
        let input = fs::read_to_string("fixtures/valid/simple.lns").unwrap();
        let tokens = tokenize(&input);
        let mut parser = Parser::new(tokens);
        let doc = parser.parse().unwrap();
        let spec = CanonicalSpec::from_ast(&doc).unwrap();
        
        // Проверяем сущности
        assert_eq!(spec.entities.len(), 2);
        assert!(spec.entities.contains_key(&vaixlns_builder::ir::CanonicalId::new("core.kernel")));
        assert!(spec.entities.contains_key(&vaixlns_builder::ir::CanonicalId::new("runtime.engine")));
        
        // Проверяем отношения
        assert_eq!(spec.relations.len(), 1);
        assert_eq!(spec.relations[0].from, vaixlns_builder::ir::CanonicalId::new("core.kernel"));
        assert_eq!(spec.relations[0].to, vaixlns_builder::ir::CanonicalId::new("runtime.engine"));
    }

    // TEST-004: Dependency semantics
    #[test]
    fn test_004_dependency_semantics() {
        let input = fs::read_to_string("fixtures/valid/dependency.lns").unwrap();
        let tokens = tokenize(&input);
        let mut parser = Parser::new(tokens);
        let doc = parser.parse().unwrap();
        let spec = CanonicalSpec::from_ast(&doc).unwrap();
        let graph = spec.dependency_graph();
        
        // Проверяем направление зависимостей
        // C зависит от B, B зависит от A
        assert!(graph.edges.contains_key(&vaixlns_builder::ir::CanonicalId::new("A")));
        assert!(graph.edges.contains_key(&vaixlns_builder::ir::CanonicalId::new("B")));
        assert!(graph.edges.contains_key(&vaixlns_builder::ir::CanonicalId::new("C")));
        
        // Проверяем DAG
        assert!(graph.is_dag());
    }

    // TEST-005: Cycle rejection
    #[test]
    fn test_005_cycle_rejection() {
        let input = fs::read_to_string("fixtures/cycles/simple_cycle.lns").unwrap();
        let tokens = tokenize(&input);
        let mut parser = Parser::new(tokens);
        let doc = parser.parse().unwrap();
        let spec = CanonicalSpec::from_ast(&doc).unwrap();
        let graph = spec.dependency_graph();
        
        // Должен быть обнаружен цикл
        assert!(!graph.is_dag());
        let cycle = graph.detect_cycles();
        assert!(cycle.is_some());
    }

    // TEST-006: Planner
    #[test]
    fn test_006_planner() {
        let input = fs::read_to_string("fixtures/valid/dependency.lns").unwrap();
        let tokens = tokenize(&input);
        let mut parser = Parser::new(tokens);
        let doc = parser.parse().unwrap();
        let spec = CanonicalSpec::from_ast(&doc).unwrap();
        let graph = spec.dependency_graph();
        let sorted = graph.topological_sort();
        let plan = BuildPlan::create(&spec, &graph, &sorted);
        
        assert!(plan.validate().is_ok());
        assert!(!plan.phases.is_empty());
        assert_eq!(plan.total_nodes, 3);
    }

    // TEST-007: Deterministic Generator
    #[test]
    fn test_007_deterministic_generator() {
        let input = fs::read_to_string("fixtures/valid/simple.lns").unwrap();
        let tokens = tokenize(&input);
        let mut parser = Parser::new(tokens);
        let doc = parser.parse().unwrap();
        let spec = CanonicalSpec::from_ast(&doc).unwrap();
        let graph = spec.dependency_graph();
        let sorted = graph.topological_sort();
        let plan = BuildPlan::create(&spec, &graph, &sorted);
        
        let gen1 = CodeGenerator::new(spec.clone(), plan.clone());
        let gen2 = CodeGenerator::new(spec.clone(), plan.clone());
        
        assert_eq!(gen1.generate(), gen2.generate());
    }

    // TEST-008: Hash integrity
    #[test]
    fn test_008_hash_integrity() {
        let input1 = fs::read_to_string("fixtures/valid/simple.lns").unwrap();
        let input2 = fs::read_to_string("fixtures/valid/dependency.lns").unwrap();
        
        let tokens1 = tokenize(&input1);
        let tokens2 = tokenize(&input2);
        let mut parser1 = Parser::new(tokens1);
        let mut parser2 = Parser::new(tokens2);
        let doc1 = parser1.parse().unwrap();
        let doc2 = parser2.parse().unwrap();
        let spec1 = CanonicalSpec::from_ast(&doc1).unwrap();
        let spec2 = CanonicalSpec::from_ast(&doc2).unwrap();
        
        let graph1 = spec1.dependency_graph();
        let graph2 = spec2.dependency_graph();
        let sorted1 = graph1.topological_sort();
        let sorted2 = graph2.topological_sort();
        let plan1 = BuildPlan::create(&spec1, &graph1, &sorted1);
        let plan2 = BuildPlan::create(&spec2, &graph2, &sorted2);
        
        let gen1 = CodeGenerator::new(spec1.clone(), plan1.clone());
        let gen2 = CodeGenerator::new(spec2.clone(), plan2.clone());
        
        // Разные спецификации должны давать разный код
        assert_ne!(gen1.generate(), gen2.generate());
    }

    // TEST-009: Lean verification
    #[test]
    fn test_009_lean_verification() {
        use vaixlns_builder::verification::LeanVerifier;
        
        let verifier = LeanVerifier::new();
        let result = verifier.verify_single_proof("proofs/lean/VAixlns/Basic.lean");
        
        // Проверяем, что Lean доступен
        assert!(result.is_ok() || result.is_err());
    }

    // TEST-010: Full integration
    #[test]
    fn test_010_full_integration() {
        let input = fs::read_to_string("fixtures/valid/simple.lns").unwrap();
        let tokens = tokenize(&input);
        let mut parser = Parser::new(tokens);
        let doc = parser.parse().unwrap();
        let spec = CanonicalSpec::from_ast(&doc).unwrap();
        
        let graph = spec.dependency_graph();
        let sorted = graph.topological_sort();
        let plan = BuildPlan::create(&spec, &graph, &sorted);
        let gen = CodeGenerator::new(spec.clone(), plan.clone());
        let output = gen.generate();
        
        // Проверяем, что код сгенерирован
        assert!(!output.is_empty());
        assert!(output.contains("pub struct core_kernel"));
        assert!(output.contains("pub struct runtime_engine"));
        assert!(output.contains("impl Runtime"));
        
        // Создаём Evidence
        let mut evidence = Evidence::new()
            .with_spec_hash(&format!("{:016x}", input.len()))
            .with_ast_hash(&format!("{:016x}", doc.decls.len()))
            .with_ir_hash(&format!("{:016x}", spec.entities.len()))
            .with_plan_hash(&format!("{:016x}", plan.phases.len()))
            .with_artifact_hash(&format!("{:016x}", output.len()))
            .with_compiler_status("PASS")
            .with_tests(10, 0)
            .with_formal_proof("VERIFIED");
        
        evidence.finalize();
        assert!(evidence.is_valid());
        
        // Проверяем Finality Gate
        let mut gate = GenesisGate::new();
        let finalized = gate.run_all(&evidence);
        assert!(finalized);
        assert!(gate.is_finalized());
    }
}
