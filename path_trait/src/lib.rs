pub trait PathResolver {
    fn resolve_path(&self, vars_map: std::collections::HashMap<String,String>) -> String;
}

pub fn extract_values(template: &str) -> Vec<String> {
    let re = regex::Regex::new(r"\{\{([^}]+)\}\}").unwrap();
    re.captures_iter(template)
        .map(|cap| cap[1].trim().to_string())
        .collect()
}