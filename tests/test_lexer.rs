#[cfg(test)]
mod tests {
    use vaixlns_builder::lexer::tokenize;

    #[test]
    fn test_lexer_simple() {
        let tokens = tokenize("domain { core }");
        assert_eq!(tokens.len(), 5);
    }
}
