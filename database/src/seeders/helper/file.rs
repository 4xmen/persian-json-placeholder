use std::{fs, io, path::Path};
fn matches_pattern(name: &str, pattern: &str) -> bool {
    let parts: Vec<&str> = pattern.split('*').collect();
    let mut rest = name;

    for (i, part) in parts.iter().enumerate() {
        if part.is_empty() {
            continue;
        }

        if i == 0 && !pattern.starts_with('*') {
            let Some(after_prefix) = rest.strip_prefix(part) else {
                return false;
            };
            rest = after_prefix;
        } else if let Some(pos) = rest.find(part) {
            rest = &rest[pos + part.len()..];
        } else {
            return false;
        }
    }

    pattern.ends_with('*') || parts.last().is_none_or(|last| rest.ends_with(last))
}

pub fn find_file(folder: impl AsRef<Path>, pattern: &str) -> io::Result<Vec<String>> {
    let mut found = Vec::new();
    let mut dirs = vec![folder.as_ref().to_path_buf()];

    while let Some(dir) = dirs.pop() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let file_type = entry.file_type()?;

            if file_type.is_dir() {
                // dirs.push(path);// add recusive
            } else if file_type.is_file()
                && entry
                    .file_name()
                    .to_str()
                    .is_some_and(|name| matches_pattern(name, pattern))
            {
                found.push(path.display().to_string());
            }
        }
    }

    Ok(found)
}
