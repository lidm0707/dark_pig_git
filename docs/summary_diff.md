# Git Diff Management: Directory and Inline Diffs

## Overview

Git diff management is essential for understanding changes between commits, branches, or working directory states. This document covers how to manage two primary types of diffs using the git2 library:

- **Directory Diffs**: File-level changes (added, modified, deleted, renamed files)
- **Inline Diffs**: Line-by-line content changes within files

## Table of Contents

1. [Core Concepts](#core-concepts)
2. [Directory Diff Management](#directory-diff-management)
3. [Inline Diff Management](#inline-diff-management)
4. [Practical Examples](#practical-examples)
5. [Best Practices](#best-practices)
6. [API Reference](#api-reference)

---

## Core Concepts

### Diff Delta Types

The git2 library represents file changes using `Delta` enum:

| Delta Type | Description | Example |
|------------|-------------|---------|
| `Added` | New file created | `src/new_file.rs` |
| `Modified` | Existing file changed | `src/main.rs` |
| `Deleted` | File removed | `old_file.rs` |
| `Renamed` | File moved/renamed | `old.rs` → `new.rs` |
| `Copied` | File copied | `original.rs` → `copy.rs` |
| `Ignored` | Changes to ignored files | `.gitignore` changes |
| `Unmodified` | No changes | N/A |
| `Typechange` | File type changed | File → Symlink |

### Diff Callback Structure

The git2 `diff_foreach` method uses four callbacks:

1. **File Callback**: Called once per file that changed
2. **Binary Callback**: Called when a file is detected as binary
3. **Hunk Callback**: Called for each section of changes (group of lines)
4. **Line Callback**: Called for each changed line within a hunk

---

## Directory Diff Management

### Purpose

Directory diffs provide a high-level overview of which files have changed, without showing the actual content differences. This is useful for:

- Generating change summaries
- Filtering files for processing
- Understanding project structure changes
- Building commit history visualizations

### Implementation

```rust
use git2::{DiffOptions, Oid, Repository};

pub fn get_directory_diff(
    repo: &Repository,
    old_commit: &str,
    new_commit: &str,
) -> Result<Vec<DirChange>, git2::Error> {
    let old_oid = Oid::from_str(old_commit)?;
    let new_oid = Oid::from_str(new_commit)?;
    
    let old_commit_obj = repo.find_commit(old_oid)?;
    let new_commit_obj = repo.find_commit(new_oid)?;
    
    let old_tree = old_commit_obj.tree()?;
    let new_tree = new_commit_obj.tree()?;
    
    let mut diff_opts = DiffOptions::new();
    diff_opts.include_unmodified(false);
    diff_opts.recurse_untracked_dirs(true);
    
    let diff = repo.diff_tree_to_tree(
        Some(&old_tree), 
        Some(&new_tree), 
        Some(&mut diff_opts)
    )?;
    
    let mut changes = Vec::new();
    
    diff.foreach(
        &mut |delta, _progress| {
            changes.push(DirChange {
                status: delta.status(),
                old_path: delta.old_file().path()
                    .map(|p| p.to_path_buf()),
                new_path: delta.new_file().path()
                    .map(|p| p.to_path_buf()),
                similarity: delta.similarity(),
            });
            true
        },
        None, None, None,
    )?;
    
    Ok(changes)
}

pub struct DirChange {
    pub status: git2::Delta,
    pub old_path: Option<std::path::PathBuf>,
    pub new_path: Option<std::path::PathBuf>,
    pub similarity: Option<i32>,
}
```

### Directory Diff Statistics

```rust
pub struct DirDiffStats {
    pub total_files: usize,
    pub added: usize,
    pub modified: usize,
    pub deleted: usize,
    pub renamed: usize,
    pub copied: usize,
}

pub fn calculate_dir_stats(changes: &[DirChange]) -> DirDiffStats {
    let mut stats = DirDiffStats {
        total_files: changes.len(),
        added: 0,
        modified: 0,
        deleted: 0,
        renamed: 0,
        copied: 0,
    };
    
    for change in changes {
        match change.status {
            git2::Delta::Added => stats.added += 1,
            git2::Delta::Modified => stats.modified += 1,
            git2::Delta::Deleted => stats.deleted += 1,
            git2::Delta::Renamed => stats.renamed += 1,
            git2::Delta::Copied => stats.copied += 1,
            _ => {}
        }
    }
    
    stats
}
```

---

## Inline Diff Management

### Purpose

Inline diffs show the exact line-by-line changes within files, including:

- Added lines
- Removed lines
- Context lines (unchanged lines around changes)
- Hunk boundaries (change sections)

This is essential for:

- Code review interfaces
- Merge conflict resolution
- Change visualization
- Patch generation

### Implementation

```rust
use git2::DiffOptions;

pub fn get_inline_diff(
    repo: &Repository,
    old_commit: &str,
    new_commit: &str,
    context_lines: u32,
) -> Result<Vec<FileDiff>, git2::Error> {
    let old_oid = Oid::from_str(old_commit)?;
    let new_oid = Oid::from_str(new_commit)?;
    
    let old_commit_obj = repo.find_commit(old_oid)?;
    let new_commit_obj = repo.find_commit(new_oid)?;
    
    let old_tree = old_commit_obj.tree()?;
    let new_tree = new_commit_obj.tree()?;
    
    let mut diff_opts = DiffOptions::new();
    diff_opts.context_lines(context_lines);
    diff_opts.indent_heuristic(true);
    diff_opts.ignore_whitespace(false);
    
    let diff = repo.diff_tree_to_tree(
        Some(&old_tree), 
        Some(&new_tree), 
        Some(&mut diff_opts)
    )?;
    
    let mut file_diffs = Vec::new();
    let mut current_diff: Option<FileDiff> = None;
    
    diff.foreach(
        &mut |delta, _progress| {
            // Start new file diff
            let path = delta.new_file().path()
                .or_else(|| delta.old_file().path())
                .map(|p| p.to_path_buf())
                .unwrap_or_else(|| std::path::PathBuf::from("(binary)"));
            
            current_diff = Some(FileDiff {
                path,
                hunks: Vec::new(),
                status: delta.status(),
            });
            true
        },
        Some(&mut |_delta, _binary| {
            // Handle binary files
            if let Some(diff) = &mut current_diff {
                diff.is_binary = true;
            }
            true
        }),
        &mut |hunk| {
            // Process hunk
            if let Some(diff) = &mut current_diff {
                let header = hunk.header()
                    .map(|h| String::from_utf8_lossy(h).to_string());
                
                diff.hunks.push(Hunk {
                    old_start: hunk.old_start(),
                    old_lines: hunk.old_lines(),
                    new_start: hunk.new_start(),
                    new_lines: hunk.new_lines(),
                    header,
                    lines: Vec::new(),
                });
            }
            true
        },
        &mut |line, hunk| {
            // Process line
            if let Some(diff) = &mut current_diff {
                if let Some(last_hunk) = diff.hunks.last_mut() {
                    let content = String::from_utf8_lossy(line.content())
                        .to_string();
                    
                    last_hunk.lines.push(Line {
                        origin: line.origin(),
                        content: content.trim_end_matches('\n').to_string(),
                        old_lineno: line.old_lineno(),
                        new_lineno: line.new_lineno(),
                    });
                }
            }
            true
        },
    )?;
    
    // Collect all file diffs
    // (In practice, you'd accumulate them properly)
    Ok(file_diffs)
}

pub struct FileDiff {
    pub path: std::path::PathBuf,
    pub hunks: Vec<Hunk>,
    pub status: git2::Delta,
    pub is_binary: bool,
}

pub struct Hunk {
    pub old_start: usize,
    pub old_lines: usize,
    pub new_start: usize,
    pub new_lines: usize,
    pub header: Option<String>,
    pub lines: Vec<Line>,
}

pub struct Line {
    pub origin: char,  // '+', '-', ' ', etc.
    pub content: String,
    pub old_lineno: Option<usize>,
    pub new_lineno: Option<usize>,
}
```

### Inline Diff Visualization

```rust
pub fn format_inline_diff(diff: &FileDiff) -> String {
    let mut output = String::new();
    
    output.push_str(&format!("File: {}\n", diff.path.display()));
    output.push_str(&format!("Status: {:?}\n", diff.status));
    output.push_str(&format!("{}\n", "=".repeat(60)));
    
    for hunk in &diff.hunks {
        output.push_str(&format!(
            "\n@@ -{},{} +{},{} @@",
            hunk.old_start, hunk.old_lines,
            hunk.new_start, hunk.new_lines
        ));
        
        if let Some(header) = &hunk.header {
            let trimmed = header.trim();
            if !trimmed.is_empty() {
                output.push_str(&format!(" {}", trimmed));
            }
        }
        output.push('\n');
        
        for line in &hunk.lines {
            output.push_str(line.origin);
            output.push_str(&line.content);
            output.push('\n');
        }
    }
    
    output
}
```

---

## Practical Examples

### Example 1: Summary Report Generator

```rust
pub fn generate_diff_report(
    repo: &Repository,
    old_commit: &str,
    new_commit: &str,
) -> Result<String, git2::Error> {
    let dir_changes = get_directory_diff(repo, old_commit, new_commit)?;
    let stats = calculate_dir_stats(&dir_changes);
    
    let mut report = String::new();
    
    report.push_str("=== Diff Summary Report ===\n\n");
    report.push_str(&format!("Comparing: {} → {}\n", old_commit, new_commit));
    report.push_str(&format!("Total files changed: {}\n\n", stats.total_files));
    
    report.push_str("=== File Changes ===\n");
    for change in &dir_changes {
        let status_symbol = match change.status {
            git2::Delta::Added => "+",
            git2::Delta::Modified => "M",
            git2::Delta::Deleted => "-",
            git2::Delta::Renamed => "R",
            _ => "?",
        };
        
        if let Some(new_path) = &change.new_path {
            report.push_str(&format!("{} {}\n", status_symbol, new_path.display()));
        } else if let Some(old_path) = &change.old_path {
            report.push_str(&format!("{} {}\n", status_symbol, old_path.display()));
        }
    }
    
    report.push_str("\n=== Statistics ===\n");
    report.push_str(&format!("Added: {}\n", stats.added));
    report.push_str(&format!("Modified: {}\n", stats.modified));
    report.push_str(&format!("Deleted: {}\n", stats.deleted));
    report.push_str(&format!("Renamed: {}\n", stats.renamed));
    
    Ok(report)
}
```

### Example 2: Working Directory Diff

```rust
pub fn get_working_dir_diff(repo: &Repository) -> Result<Vec<DirChange>, git2::Error> {
    let head = repo.head()?;
    let head_commit = head.peel_to_commit()?;
    let head_tree = head_commit.tree()?;
    
    let mut diff_opts = DiffOptions::new();
    diff_opts.include_untracked(true);
    diff_opts.recurse_untracked_dirs(true);
    
    let diff = repo.diff_tree_to_workdir(Some(&head_tree), Some(&mut diff_opts))?;
    
    let mut changes = Vec::new();
    
    diff.foreach(
        &mut |delta, _progress| {
            changes.push(DirChange {
                status: delta.status(),
                old_path: delta.old_file().path().map(|p| p.to_path_buf()),
                new_path: delta.new_file().path().map(|p| p.to_path_buf()),
                similarity: delta.similarity(),
            });
            true
        },
        None, None, None,
    )?;
    
    Ok(changes)
}
```

### Example 3: Combined Analysis

```rust
pub fn analyze_changes(
    repo: &Repository,
    old_commit: &str,
    new_commit: &str,
) -> Result<DiffAnalysis, git2::Error> {
    // Get directory-level changes
    let dir_changes = get_directory_diff(repo, old_commit, new_commit)?;
    let dir_stats = calculate_dir_stats(&dir_changes);
    
    // Get inline diff for modified files
    let mut file_diffs = Vec::new();
    
    for change in &dir_changes {
        if change.status == git2::Delta::Modified {
            if let Some(new_path) = &change.new_path {
                // Get inline diff for this specific file
                let inline_diff = get_inline_diff_for_file(
                    repo,
                    old_commit,
                    new_commit,
                    new_path,
                    3 // context lines
                )?;
                
                if let Some(diff) = inline_diff {
                    file_diffs.push(diff);
                }
            }
        }
    }
    
    // Calculate impact metrics
    let total_additions: usize = file_diffs.iter()
        .flat_map(|d| d.hunks.iter())
        .flat_map(|h| h.lines.iter())
        .filter(|l| l.origin == '+')
        .count();
    
    let total_deletions: usize = file_diffs.iter()
        .flat_map(|d| d.hunks.iter())
        .flat_map(|h| h.lines.iter())
        .filter(|l| l.origin == '-')
        .count();
    
    Ok(DiffAnalysis {
        directory_stats: dir_stats,
        file_changes: dir_changes,
        inline_diffs: file_diffs,
        total_additions,
        total_deletions,
    })
}

pub struct DiffAnalysis {
    pub directory_stats: DirDiffStats,
    pub file_changes: Vec<DirChange>,
    pub inline_diffs: Vec<FileDiff>,
    pub total_additions: usize,
    pub total_deletions: usize,
}
```

---

## Best Practices

### 1. Use Appropriate Context

```rust
// For summary views: minimal context
diff_opts.context_lines(0);

// For code review: standard context
diff_opts.context_lines(3);

// For detailed analysis: more context
diff_opts.context_lines(10);
```

### 2. Handle Binary Files

Always provide a binary callback to avoid crashes:

```rust
diff.foreach(
    &mut file_callback,
    Some(&mut |_delta, _binary| {
        // Binary file detected
        true // Continue processing
    }),
    &mut hunk_callback,
    &mut line_callback,
)?;
```

### 3. Optimize Performance

For large repositories, use filters to reduce processing:

```rust
diff_opts.pathspec(vec!["src/**/*.rs".to_string()]);
diff_opts.skip_binary_check(true);
```

### 4. Error Handling

```rust
match repo.diff_tree_to_tree(Some(&old_tree), Some(&new_tree), Some(&mut opts)) {
    Ok(diff) => {
        // Process diff
    }
    Err(e) => {
        eprintln!("Failed to create diff: {}", e);
        // Handle error appropriately
    }
}
```

### 5. Memory Management

For very large diffs, consider processing incrementally:

```rust
diff.foreach(
    &mut |delta, _| {
        // Process immediately, don't accumulate
        process_delta(delta);
        true
    },
    None, None, None,
)?;
```

---

## API Reference

### Key Functions

#### `Repository::diff_tree_to_tree`

```rust
pub fn diff_tree_to_tree(
    &self,
    old_tree: Option<&Tree>,
    new_tree: Option<&Tree>,
    opts: Option<&mut DiffOptions>,
) -> Result<Diff<'_>, Error>
```

**Parameters:**
- `old_tree`: Optional old tree to compare from
- `new_tree`: Optional new tree to compare to
- `opts`: Optional diff options

**Returns:** A `Diff` object representing the changes

#### `Repository::diff_tree_to_workdir`

```rust
pub fn diff_tree_to_workdir(
    &self,
    old_tree: Option<&Tree>,
    opts: Option<&mut DiffOptions>,
) -> Result<Diff<'_>, Error>
```

Compares a tree to the working directory.

#### `Diff::foreach`

```rust
pub fn foreach(
    &mut self,
    file_cb: &mut impl FnMut(&DiffDelta, Option<&DiffHunk>) -> bool,
    binary_cb: Option<&mut impl FnMut(&DiffDelta, &DiffBinary) -> bool>,
    hunk_cb: Option<&mut impl FnMut(&DiffDelta, &DiffHunk) -> bool>,
    line_cb: Option<&mut impl FnMut(&DiffDelta, Option<&DiffHunk>, &DiffLine) -> bool>,
) -> Result<(), Error>
```

Iterates over all changes in the diff.

### DiffOptions Configuration

```rust
let mut opts = DiffOptions::new();

// Context lines around changes
opts.context_lines(3);

// Ignore whitespace changes
opts.ignore_whitespace(true);

// Include untracked files
opts.include_untracked(true);

// Recurse into untracked directories
opts.recurse_untracked_dirs(true);

// Include unmodified files
opts.include_unmodified(false);

// Pathspec filtering
opts.pathspec(vec!["src/**/*".to_string()]);

// Indent heuristic for better diff detection
opts.indent_heuristic(true);
```

### Delta Status Values

```rust
pub enum Delta {
    Added,
    Deleted,
    Modified,
    Renamed,
    Copied,
    Ignored,
    Untracked,
    Typechange,
    Unmodified,
}
```

### Line Origin Characters

```rust
// '+' - Added line
// '-' - Removed line
// ' ' - Context line (unchanged)
// '=' - Location indicator (file headers)
// '>' - Git-specific markers
// '<' - Git-specific markers
// 'F' - Fuzz marker
```

---

## Common Use Cases

### 1. Commit Message Generation

Use directory diff to automatically generate commit message summaries.

### 2. Code Review UI

Use inline diffs to display changes in a user-friendly interface.

### 3. Build System Optimization

Use directory diff to determine which files need to be rebuilt.

### 4. Documentation Updates

Track changes to documentation files separately from code changes.

### 5. Security Auditing

Monitor changes to security-sensitive files using directory diff filters.

### 6. Merge Conflict Detection

Use inline diffs to identify potential merge conflicts before they occur.

---

## Conclusion

Effective diff management requires understanding both directory-level and inline-level changes:

- **Directory diffs** provide the "what" - which files changed
- **Inline diffs** provide the "how" - what specifically changed within files

The git2 library offers comprehensive tools for both types of analysis. By combining them appropriately and following best practices, you can build powerful tools for understanding and visualizing code changes.

For more detailed information, refer to the [git2-rs documentation](https://docs.rs/git2/) and the [libgit2 API reference](https://libgit2.org/docs/).