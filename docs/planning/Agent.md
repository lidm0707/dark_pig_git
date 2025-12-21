# Dark Pig Git - Project Plan

## Overview

Dark Pig Git is a Rust-based visualization tool for Git repositories that provides an intuitive graphical representation of commit history, branches, and merges using GPUI for the user interface.

## Project Goals

1. Parse Git repositories using the git2 crate
2. Create a comprehensive data model for Git commits, branches, and merges
3. Implement an efficient lane allocation algorithm for commit visualization
4. Build an interactive graphical interface using GPUI
5. Support filtering and focusing on specific commits, branches, or time ranges

## Architecture

### Core Components

#### 1. Entities (`src/entities/`)
- **CommitNode**: Represents a single commit with metadata
  - Commit ID (OID)
  - Message
  - Author information
  - Timestamp
  - Parent commit IDs
  - Lane position
  - Branch information

- **Lane**: Represents a visual lane in the graph
  - Commit placement and ordering
  - Lane connections between commits
  - Branch associations
  - Visual properties (color, width)

#### 2. Graph Processing (`src/graph/`)
- **CommitGraph**: Core data structure for organizing commits
- **LaneAllocator**: Algorithm to assign commits to appropriate lanes
- **GraphAnalyzer**: Extracts relationships between commits
- **HistoryTraverser**: Traverses commit history efficiently

#### 3. Rendering (`src/rendering/`)
- **GraphRenderer**: Renders the commit graph to GPUI elements
- **CommitRenderer**: Renders individual commit nodes
- **LaneRenderer**: Renders lane connections and branches
- **LayoutManager**: Manages the layout of elements in the GPUI view

#### 4. Configuration (`src/config/`)
- **Config**: Global configuration options
- **FilterOptions**: Commit filtering criteria
- **RenderOptions**: Output customization options
- **ThemeOptions**: Visual appearance customization

### Data Flow

1. Parse Git repository from provided path using git2
2. Extract commit history and build CommitGraph
3. Allocate commits to lanes using LaneAllocator
4. Apply any filters based on user options
5. Render the graph using the GPUI renderer
6. Display interactive visualization in the GUI

## Development Phases

### Phase 1: Core Data Structures (In Progress)
- [x] Basic CommitNode structure
- [x] Basic Lane structure
- [ ] Enhanced Lane with connections and branch associations
- [ ] Comprehensive CommitGraph
- [ ] Tests for all entities

### Phase 2: Graph Processing
- [ ] Implement efficient graph traversal algorithms
- [ ] Develop lane allocation algorithm for complex merge patterns
- [ ] Add support for branch detection and naming
- [ ] Handle merge commits correctly
- [ ] Implement filtering capabilities

### Phase 3: GPUI Interface
- [ ] Set up basic GPUI application structure
- [ ] Implement GraphRenderer for commit visualization
- [ ] Create interactive elements for commit inspection
- [ ] Add zoom and pan functionality for large graphs
- [ ] Implement commit filtering UI

### Phase 4: Configuration and Customization
- [ ] Implement configuration file support
- [ ] Add visual theming options
- [ ] Create keyboard shortcuts and navigation
- [ ] Add export functionality

## Technical Considerations

### Performance
- Efficient traversal of commit history
- Lazy loading for large repositories
- Memory optimization for graphs with many commits
- GPU-accelerated rendering through GPUI
- Incremental updates as the repository changes

### Usability
- Clear visualization of complex merge patterns
- Interactive exploration of commit history
- Intuitive navigation and search functionality
- Responsive layout for different screen sizes
- Intuitive color coding for different branches

## Testing Strategy

### Unit Tests
- Entity behavior validation
- Graph algorithm correctness
- Component isolation

### Integration Tests
- End-to-end visualization
- Real repository testing
- Performance benchmarking

## Dependencies
- **git2**: Git repository access
- **dotenv**: Environment variable management
- **GPUI**: Graphical User Interface for interactive visualization
- **serde**: Serialization for configuration
- **tokio**: Async operations (if needed)

## Documentation
- Inline code documentation (rustdoc)
- User manual with examples
- Developer guide for contributors
- Architecture decision records (ADRs)
