#[cfg(test)]
mod tests {
    use vaixlns_builder::lexer::tokenize;
    use vaixlns_builder::parser::Parser;

    #[test]
    fn test_parser_meta() {
        let input = r#"
        meta {
            id "VAIXLNS://TEST/1.0"
            name "Test Root"
            version "1.0.0"
            authority "TEST"
        }
        "#;
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let doc = parser.parse().unwrap();
        assert_eq!(doc.decls.len(), 1);
    }

    #[test]
    fn test_parser_entity() {
        let input = r#"
        vaixlns_root {
            ontology {
                entities {
                    core.kernel {
                        domain core
                        kind "kernel"
                        provides ["identity"]
                        requires []
                    }
                }
            }
        }
        "#;
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let doc = parser.parse().unwrap();
        // Проверяем, что есть хотя бы одна Entity
        let entities = doc.find_entities();
        assert!(!entities.is_empty());
        assert_eq!(entities[0].id, "core.kernel");
    }

    #[test]
    fn test_parser_relation() {
        let input = "link core.kernel -> runtime.engine";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let doc = parser.parse().unwrap();
        let relations = doc.find_relations();
        assert_eq!(relations.len(), 1);
    }
}
