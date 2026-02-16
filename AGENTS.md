# AGENTS.md - Protibuild Development Guide

This document provides guidance for agents working on the Protibuild project.

## Project Overview

Protibuild is a protein simulator and builder built in Rust using the Bevy game engine. It features a project system for managing different scene templates (DevCube, AminoAcids), with ball-and-stick visualization and CPK coloring for atoms.

## Build & Development Commands

### Running the Application

```bash
make run              # Run the application (cargo run -p protibuild)
```

### Code Quality

```bash
make qa               # Run all QA checks: fmt, clippy, audit, deny
cargo fmt             # Format code
cargo clippy --all-targets --all-features -- -D warnings  # Lint with warnings as errors
cargo audit           # Check for security vulnerabilities
cargo deny check      # Check licenses and dependencies
```

### Testing

```bash
# Currently no tests exist, but when added:
cargo test            # Run all tests
cargo test <name>     # Run a specific test
```

## Code Style Guidelines

### Formatting

- **Rust edition**: 2024 (see `Cargo.toml`)
- **Indentation**: 4 spaces, no tabs
- **Line width**: Max 100 characters
- **Heuristics**: Default (rustfmt default settings)
- **Import sorting**: Enabled (reorder_imports, reorder_modules)
- **Remove nested parens**: Enabled

Run `cargo fmt` before committing.

### Naming Conventions

- **Modules**: `snake_case` (e.g., `tab_bar`, `amino_acids`)
- **Types/Structs**: `PascalCase` (e.g., `ActiveTab`, `ProjectTemplate`)
- **Enums variants**: `PascalCase` (e.g., `TabType::File`, `Element::Hydrogen`)
- **Functions/Methods**: `snake_case` (e.g., `handle_tab_clicks`)
- **Constants**: `SCREAMING_SNAKE_CASE` for true constants
- **Fields/Variables**: `snake_case` (e.g., `camera_position`)
- **Files**: One file per concept. Each file should have a single responsibility (e.g., `atoms.rs` for element/atom types, `tab_bar.rs` for tab UI). Avoid bundling unrelated types in the same file.

### Imports

- Use `use bevy::prelude::*;` for Bevy types
- Use absolute imports within crate: `use crate::projects::{SwitchProjectMessage, templates::ProjectTemplates};`
- Use `pub(crate)` for module visibility when not exposing publicly

### Bevy 0.18 Patterns

#### Messages (not Events)
```rust
#[derive(Message, Clone, Debug)]
pub struct SwitchProjectMessage {
    pub project: Project,
}

// In plugin setup:
app.add_message::<SwitchProjectMessage>()

// Sending:
messages.write(SwitchProjectMessage { project });

// Reading:
for message in switch_messages.read() { ... }
```

#### Query Conflicts
When multiple queries access the same component, use `ParamSet`:
```rust
fn system(
    mut param_set: ParamSet<(
        Query<(&TabButton, &mut BackgroundColor, &mut Node)>,
        Query<(&ProjectSelectionButton, &mut BackgroundColor)>,
    )>,
) {
    for item in &mut param_set.p0() { ... }
    for item in &mut param_set.p1() { ... }
}
```

#### Resource Initialization
Don't rely on `is_added()` for resources in Update systems. Use a flag instead:
```rust
#[derive(Resource, Default)]
pub struct ProjectEntities {
    pub roots: Vec<Entity>,
    pub initialized: bool,  // Use this flag
}
```

#### UI Interactions
Use `Interaction` component with `Changed<Interaction>`:
```rust
fn handle_clicks(
    interaction_query: Query<(&Interaction, &TabButton), Changed<Interaction>>,
) {
    for (interaction, button) in &interaction_query {
        match *interaction {
            Interaction::Pressed => { ... }
            Interaction::Hovered => { ... }
            Interaction::None => { ... }
        }
    }
}
```

#### Colors
Use Bevy 0.18 sRGB functions:
```rust
BackgroundColor(Color::srgb(0.30, 0.30, 0.32))
TextColor(Color::srgb(0.95, 0.95, 0.95))
BorderColor::all(Color::srgba(0.25, 0.25, 0.28, 1.0))  // Note: .all() required
```

#### UI Node Builders
```rust
Node {
    width: Val::Percent(100.0),
    height: Val::Px(32.0),
    position_type: PositionType::Absolute,
    flex_direction: FlexDirection::Row,
    align_items: AlignItems::Center,
    border_radius: BorderRadius::all(Val::Px(6.0)),
    ..default()  // Always use ..default() to fill rest
}
```

### Error Handling

- Use `?` operator for fallible operations
- Use `Result` types for functions that can fail
- Log errors with appropriate level: `error!("Failed to spawn atom: {}", e)`

### Documentation

- Add doc comments (`///`) for public types and functions
- Add inline comments (`//`) only when code is non-obvious
- Comment the "why", not the "what"

### Architecture

- **Plugin structure**: Each feature is a plugin implementing `Plugin`
- **Systems**: Use `Startup` for setup, `Update` for runtime logic
- **Resources**: Use `#[derive(Resource)]` for global state
- **Components**: Use `#[derive(Component)]` for entity data
- **Messages**: Use `#[derive(Message)]` for event-like communication

### Project Structure

```
protibuild/src/
├── lib.rs              # Root module exports
├── main.rs             # Entry point
├── chemistry/          # Chemistry simulation (atoms, amino acids)
│   ├── atoms/          # Element, Atom, Bond types
│   ├── amino_acids/    # Amino acid structures
│   └── rendering/      # Visual rendering
├── config/             # Configuration (amino_acids.json)
├── objects/            # 3D objects (dev_cube, grid)
├── projects/           # Project system (templates, switching)
├── ui/                 # UI components (tab_bar, crosshair)
├── user/               # User input (camera, interaction)
└── world.rs            # World initialization
```

### Dependencies

- **bevy 0.18**: Game engine (with dynamic_linking feature for dev)
- **rapier3d**: Physics
- **serde/serde_json**: Serialization
- **log**: Logging
- **once_cell**: Static initialization

### Known Bevy 0.18 Gotchas

1. `BorderColor` requires `.all()`: `BorderColor::all(...)` not `BorderColor(...)`
2. Use `ParamSet` to avoid B0001 query conflicts
3. Use flags instead of `is_added()` for resource initialization in Update
4. Use `Message` trait + `MessageWriter`/`MessageReader` for project switching events

### Entity Management Patterns

#### Naming and Cleanup
Always spawn top-level entities with `Name` and a cleanup marker component:
```rust
commands.spawn((
    Name::new("Player"),
    CleanupOnGameExit,
    Transform::default(),
    // ... other components
));
```

Use `StateScoped` (Bevy 0.14+) for automatic cleanup on state transitions:
```rust
#[derive(Default, States)]
enum AppState { #[default] Menu, Game }

commands.spawn((
    StateScoped(AppState::Game),
    // ... components
));
```

#### Marker Components
Use Zero-Sized Types (ZST) as marker components for tagging entities:
```rust
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

// Query easily: Query<&Transform, With<Player>>
```

#### Bundles
Group common component sets into bundles for reusable entity templates:
```rust
#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    health: Health,
    transform: Transform,
    sprite: Sprite,
}

commands.spawn(PlayerBundle {
    player: Player,
    health: Health(100),
    ..default()
});
```

#### Commands Patterns
```rust
// Batch spawn many entities at once
commands.spawn_batch(vec![
    (Name::new("Enemy1"), Enemy, Transform::default()),
    (Name::new("Enemy2"), Enemy, Transform::default()),
]);

// Queue custom commands
commands.queue(|world: &mut World| {
    // arbitrary world manipulation
});
```

### System Scheduling Patterns

#### State-Bounded Systems
Bind systems to specific states to avoid unnecessary computation:
```rust
#[derive(Default, States)]
enum GameState { #[default] Menu, Playing, Paused }

app.add_systems(Update, player_movement.run_if(in_state(GameState::Playing)));
```

#### SystemSets and Ordering
Group related systems and enforce ordering:
```rust
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum UpdateSet { Input, Physics, Rendering }

app.configure_sets(Update, (
    UpdateSet::Input.before(UpdateSet::Physics),
    UpdateSet::Physics.before(UpdateSet::Rendering),
));
```

Use `.chain()` for strict sequential ordering:
```rust
.add_systems(Update, (system_a, system_b, system_c).chain())
```

#### Setup/Cleanup Systems
Co-locate state transition handlers:
```rust
.add_systems(OnEnter(GameState::Playing), setup_game)
.add_systems(OnExit(GameState::Playing), cleanup_game)
```

### Project Organization Patterns

#### Prelude Module
Create a prelude for easy imports:
```rust
// src/prelude.rs
pub use crate::chemistry::atoms::*;
pub use crate::projects::*;
pub use crate::ui::*;

pub use bevy::prelude::*;
```

#### Plugin Grouping
Organize features into plugins:
```rust
pub struct ChemistryPlugin;

impl Plugin for ChemistryPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AminoAcidPlugin)
           .add_plugins(RenderingPlugin);
    }
}
```

### Performance Patterns

#### Query Optimization
- Use `With`/`Without` filters to narrow queries
- Prefer `Option<&T>` over `Or` queries when possible
- Cache rarely-changing component access

#### Spawn Batching
Use `spawn_batch` for creating entities:
```rust many similar
commands.spawn_batch((0..1000).map(|i| (
    Name::new(format!("Particle {}", i)),
    Particle,
    Transform::from_translation(Vec3::new(i as f32 * 0.1, 0.0, 0.0)),
)));
```

### Debugging Patterns

#### Named Entities
Always add `Name` for debugging:
```rust
commands.spawn((
    Name::new("Player"),
    // ... components
));
```

This enables better debugging in bevy_inspector_egui and logs.

## Design Principles

### General OOP/FP Principles (Adapted for Rust)

- **Single Responsibility**: Each type, function, and system should do one thing well. A plugin should handle one feature domain. A system should process one concern.
- **Open/Closed**: Extend behavior by adding new types/modules, not modifying existing code. Use traits and generics for extensibility.
- **Liskov Substitution**: Derived types must be substitutable for their base types. Implement trait methods consistently with the trait's contract.
- **Interface Segregation**: Prefer small, focused traits over large, general ones. Bevy's systems naturally encourage this.
- **Dependency Inversion**: Depend on abstractions (traits), not concretions. Use generics in systems where appropriate.

### Rust-Specific Guidelines

- **Prefer composition over inheritance**: Rust has no inheritance. Use traits, composition, and delegation.
- **Make invalid states unrepresentable**: Use types to encode invariants at compile time.
- **Zero-cost abstractions**: Bevy ECS provides performance without sacrificing ergonomics. Don't add unnecessary runtime overhead.
- **RAII**: Use Rust's ownership model. Let destructors clean up resources (Bevy handles this via `Despawn`).
- **Clone sparingly**: Prefer `&T`, `&mut T`, or `Option<&T>` over cloning. Only clone when necessary for ownership transfer.

### Clean Code Practices

- **Small functions**: Systems should be focused and composable. Break complex logic into helper functions.
- **Meaningful names**: Names should reveal intent. `spawn_molecule` is better than `do_it`.
- **DRY (Don't Repeat Yourself)**: Extract common patterns into utility functions or macros.
- **YAGNI (You Aren't Gonna Need It)**: Don't add flexibility until it's actually needed.
- **Fail fast**: Validate inputs early. Return `Result` for recoverable errors, panic only for programmer errors.

### Bevy-Specific Patterns

- **Entity-component composition**: Model data as Components, behavior as Systems. Avoid putting complex logic in Components.
- **Resource as singleton state**: Use Resources for global state that persists across frames.
- **Event-driven for cross-system communication**: Use Messages for decoupled communication between systems.
- **Immutable data in queries**: Bevy queries provide read access. Use `Res` for read-only, `ResMut` for mutable state.
- **Command buffer for structural changes**: Use `Commands` to spawn/despawn entities during system execution.

### Testing

- **Test behavior, not implementation**: Test public interfaces and observable outcomes.
- **Unit test pure functions**: Extract pure logic into free functions for easy testing.
- **Integration test systems**: Verify systems work correctly together.
- **Benchmark hot paths**: Use `criterion` or `bencher` for performance-critical code.

## Research & Documentation

### Using Web Search

When you need to look up documentation or examples on the internet:

- **Use `websearch` for general queries**: Search for patterns, tutorials, or how-to guides
- **Use `codesearch` for code examples**: Find specific API usage, patterns, or implementation details
- **Use `webfetch` for specific URLs**: Grab content from specific documentation pages or articles

### When to Research

- **Bevy API questions**: Always prefer searching for Bevy 0.18 docs first
- **Library usage**: Look up crates.io docs or GitHub READMEs
- **Pattern verification**: Confirm idiomatic patterns before implementing
- **Error resolution**: Search for error messages or warning explanations

### Example Research Tasks

```bash
# Find Bevy 0.18 UI button patterns
codesearch: "Bevy 0.18 button spawning UI example"

# Search for parametricEQ usage
websearch: "Bevy 0.18 ParamSet query conflict example"

# Get specific docs
webfetch: "https://docs.rs/bevy/0.18.0/bevy/prelude/struct.App.html"
```

### Research Best Practices

- Always specify the Bevy version (0.18) in searches
- Prefer official Bevy book and API docs over third-party tutorials
- If search results are stale, try adding "2025" or "2026" to queries
- Verify code examples match Bevy 0.18 API (not 0.13 or 0.14)
