use td_cache::CacheDb;

/// Resolve a project reference to an ID.
/// Chain: exact ID → URL parsing → exact name (case-insensitive) → fuzzy suggestion.
pub fn resolve_project_id(cache: &CacheDb, input: &str) -> anyhow::Result<String> {
    // 1. Try as direct ID
    if cache.get_project(input).is_ok() {
        return Ok(input.to_string());
    }

    // 2. Try exact name match (case-insensitive)
    if let Some(project) = cache.find_project_by_name(input)? {
        return Ok(project.id);
    }

    // 3. Fuzzy suggestion
    let all_projects = cache.get_all_projects()?;
    let input_lower = input.to_lowercase();
    let mut best_match: Option<(String, String, usize)> = None;

    for p in &all_projects {
        let dist = levenshtein(&input_lower, &p.name.to_lowercase());
        if dist <= 3 && (best_match.is_none() || dist < best_match.as_ref().unwrap().2) {
            best_match = Some((p.id.clone(), p.name.clone(), dist));
        }
    }

    if let Some((id, name, _)) = best_match {
        anyhow::bail!("Project \"{input}\" not found. Did you mean \"{name}\" ({id})?");
    }

    anyhow::bail!("Project \"{input}\" not found. Run `td sync` to refresh.");
}

/// Resolve a label reference to a name (labels are referenced by name in the API).
pub fn resolve_label_name(cache: &CacheDb, input: &str) -> anyhow::Result<String> {
    if let Some(label) = cache.find_label_by_name(input)? {
        return Ok(label.name);
    }

    let all_labels = cache.get_all_labels()?;
    let input_lower = input.to_lowercase();

    for l in &all_labels {
        let dist = levenshtein(&input_lower, &l.name.to_lowercase());
        if dist <= 2 {
            anyhow::bail!("Label \"{input}\" not found. Did you mean \"{}\"?", l.name);
        }
    }

    anyhow::bail!("Label \"{input}\" not found. Run `td sync` to refresh.");
}

/// Simple Levenshtein distance implementation.
fn levenshtein(a: &str, b: &str) -> usize {
    let a_len = a.len();
    let b_len = b.len();

    if a_len == 0 {
        return b_len;
    }
    if b_len == 0 {
        return a_len;
    }

    let mut prev_row: Vec<usize> = (0..=b_len).collect();
    let mut curr_row = vec![0; b_len + 1];

    for (i, ca) in a.chars().enumerate() {
        curr_row[0] = i + 1;
        for (j, cb) in b.chars().enumerate() {
            let cost = if ca == cb { 0 } else { 1 };
            curr_row[j + 1] = (prev_row[j + 1] + 1)
                .min(curr_row[j] + 1)
                .min(prev_row[j] + cost);
        }
        std::mem::swap(&mut prev_row, &mut curr_row);
    }

    prev_row[b_len]
}

#[cfg(test)]
mod tests {
    use super::*;
    use td_api::models::Project;

    fn setup_db_with_projects() -> CacheDb {
        let db = CacheDb::open_in_memory().unwrap();
        db.upsert_project(&Project {
            id: "p1".to_string(),
            name: "Work".to_string(),
            color: None,
            parent_id: None,
            order: Some(1),
            is_favorite: false,
            is_inbox_project: false,
            is_team_inbox: false,
            view_style: None,
            url: None,
        })
        .unwrap();
        db.upsert_project(&Project {
            id: "p2".to_string(),
            name: "Personal".to_string(),
            color: None,
            parent_id: None,
            order: Some(2),
            is_favorite: false,
            is_inbox_project: false,
            is_team_inbox: false,
            view_style: None,
            url: None,
        })
        .unwrap();
        db
    }

    #[test]
    fn test_resolve_by_id() {
        let db = setup_db_with_projects();
        let result = resolve_project_id(&db, "p1").unwrap();
        assert_eq!(result, "p1");
    }

    #[test]
    fn test_resolve_by_name() {
        let db = setup_db_with_projects();
        let result = resolve_project_id(&db, "work").unwrap();
        assert_eq!(result, "p1");
    }

    #[test]
    fn test_resolve_fuzzy_suggestion() {
        let db = setup_db_with_projects();
        let err = resolve_project_id(&db, "Wrk").unwrap_err();
        assert!(err.to_string().contains("Did you mean"));
    }

    #[test]
    fn test_resolve_not_found() {
        let db = setup_db_with_projects();
        let err = resolve_project_id(&db, "zzzzzzzzz").unwrap_err();
        assert!(err.to_string().contains("not found"));
    }

    #[test]
    fn test_levenshtein() {
        assert_eq!(levenshtein("kitten", "sitting"), 3);
        assert_eq!(levenshtein("", "abc"), 3);
        assert_eq!(levenshtein("same", "same"), 0);
        assert_eq!(levenshtein("work", "wrk"), 1);
    }
}
