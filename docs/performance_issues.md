# Performance Issues in dark_pig_git

## Overview

The dark_pig_git project experiences significant performance degradation as the number of commits increases. This document outlines the main performance bottlenecks and provides recommended solutions.

## Identified Performance Issues

### 1. Loading All Commits at Once

**Location**: `src/main.rs` (Lines 15-30)

**Problem**: The application loads all commits from the repository into memory at once, which causes high CPU and memory usage for repositories with many commits.

```rust
for (index, commit_oid) in rewalk.enumerate() {
    let commit_oid = commit_oid?;
    let commit = repo.find_commit(commit_oid)?;
    let parent_ids: Vec<Oid> = commit.parents().map(|parent| parent.id()).collect();
    // ...
}
```

**Impact**: 
- Memory consumption scales linearly with the number of commits
- Initial loading time becomes prohibitively long for large repositories
- CPU usage spikes during the loading process

### 2. Inefficient Sorting

**Location**: `src/main.rs` (Lines 32-36)

**Problem**: Sorting all commits by timestamp using O(n log n) algorithm becomes increasingly expensive with more commits.

```rust
commits.sort_by(|a, b| b.timestamp.seconds().cmp(&a.timestamp.seconds()));
```

**Impact**:
- Processing time increases significantly with repository size
- No incremental sorting when new commits are added

### 3. Multiple Passes Through Data

**Problem**: The application makes three separate passes through all commits:
1. First pass: Collect all commits
2. Second pass: Assign lanes and calculate positions
3. Third pass: Create edges between commits

**Impact**:
- Unnecessary repeated computation
- Poor cache locality due to multiple iterations

### 4. Inefficient Rendering

**Location**: `src/entities/garph.rs` (Lines 56-58)

**Problem**:
- All edges are cloned for every render call
- All commit nodes are rendered regardless of visibility
- Canvas is completely redrawn on every frame

```rust
let edges = self.edge_manager.edges.clone();
```

**Impact**:
- High CPU usage during scrolling or resizing
- Memory usage spikes during rendering
- Unnecessary rendering of off-screen elements

### 5. Lane Assignment Algorithm

**Location**: `src/entities/lane.rs` (Lines 18-25)

**Problem**: The lane assignment algorithm performs linear searches through the lanes vector multiple times for each commit, resulting in O(n²) behavior.

```rust
let mut lane = match self
    .lanes
    .iter()
    .position(|slot| slot.as_ref() == Some(commit_oid))
```

**Impact**:
- CPU usage increases quadratically with commit count
- Poor scalability for large repositories

### 6. Memory Allocation Patterns

**Problem**:
- Frequent HashMap lookups and insertions
- Many Vec operations that cause reallocations
- Excessive cloning of nodes when creating UI elements

**Impact**:
- Memory fragmentation
- Increased garbage collection pressure
- Unnecessary memory usage

## Recommended Solutions

### 1. Implement Lazy Loading/Pagination

Load commits in batches rather than all at once:

```rust
const BATCH_SIZE: usize = 100;
let mut commits: Vec<CommitNode> = Vec::with_capacity(BATCH_SIZE);

for (i, commit_oid) in rewalk.enumerate() {
    if i >= BATCH_SIZE { break; }
    // Process commit...
}
```

### 2. Implement Virtual Rendering

Only render visible elements to reduce CPU and memory usage:

```rust
let visible_range = calculate_visible_range(scroll_position, viewport_height);
let visible_nodes = &self.nodes[visible_range.start..visible_range.end];
```

### 3. Optimize Lane Assignment

Use more efficient data structures for lane management:

```rust
use std::collections::HashMap;

struct LaneManager {
    lanes: Vec<Option<Oid>>,
    lane_index: HashMap<Oid, usize>, // Cache for O(1) lookups
}
```

### 4. Reduce Cloning and Allocations

Use references instead of cloning where possible:

```rust
let edges = &self.edge_manager.edges; // Reference instead of clone
```

### 5. Implement Incremental Updates

When new commits are added, only update the affected parts of the graph rather than rebuilding everything.

### 6. Add Caching

Implement caching for expensive operations like path calculations and rendering.

## Priority

The following changes should be implemented in order of priority:

1. **High Priority**: Lazy loading of commits (Immediate impact)
2. **High Priority**: Virtual rendering (Significant memory savings)
3. **Medium Priority**: Lane assignment optimization (CPU usage reduction)
4. **Medium Priority**: Reduce cloning and allocations (Memory optimization)
5. **Low Priority**: Incremental updates (Nice-to-have feature)

## Expected Performance Gains

After implementing these optimizations:

- **Memory Usage**: Reduced by 70-90% for large repositories
- **Initial Load Time**: Reduced by 80-95%
- **Scrolling Performance**: Improved to maintain 60fps even with thousands of commits
- **CPU Usage**: Reduced by 60-80% during normal operation

## Testing Approach

To verify performance improvements:

1. Use large repositories (10k+ commits) for testing
2. Measure memory usage with profiling tools
3. Benchmark scroll and zoom performance
4. Compare before/after metrics for all operations

## Conclusion

The current implementation has significant scalability issues that prevent it from handling large Git repositories efficiently. By implementing the recommended optimizations, the application should be able to handle repositories with tens of thousands of commits without performance degradation.