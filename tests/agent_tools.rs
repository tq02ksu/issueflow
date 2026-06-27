use issueflow::agent::gitlab_tools;

#[test]
fn tool_definitions_expose_expected_names() {
    let defs = gitlab_tools::tool_definitions();
    let names: Vec<&str> = defs
        .iter()
        .map(|v| v["function"]["name"].as_str().unwrap())
        .collect();
    assert!(names.contains(&"create_issue"));
    assert!(names.contains(&"list_issues"));
    assert!(names.contains(&"get_repo_file"));
    assert!(names.contains(&"list_wiki_pages"));
    assert!(names.contains(&"get_wiki_page"));
    assert!(names.contains(&"update_issue"));
    assert!(names.contains(&"delete_issue"));
    assert!(names.contains(&"get_issue"));
    assert_eq!(names.len(), 8);
}

#[test]
fn get_repo_file_allowlist_works() {
    assert!(!gitlab_tools::is_allowed_repo_path("src/secrets.txt"));
    assert!(gitlab_tools::is_allowed_repo_path("AGENTS.md"));
    assert!(gitlab_tools::is_allowed_repo_path("README.md"));
    assert!(gitlab_tools::is_allowed_repo_path("docs/CONFIG.md"));
    assert!(!gitlab_tools::is_allowed_repo_path(".env"));
}
