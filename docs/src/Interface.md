# Interface

Protibuild features a minimalist interface designed to keep the focus on molecular visualization.

## Main UI Elements

### Tab Bar

Located at the top of the window, the tab bar provides access to the File menu:

- **File Tab** - Opens the project selection menu (active by default)
- **Default Tab** - A placeholder tab for future features

### File Menu

The File menu appears when you click the "File" tab. It contains:

1. **Application Title** - "Protibuild" displayed prominently
2. **Templates Section** - List of available project templates
3. **Project Buttons** - Click to switch between projects

#### Project Buttons

| Button | Description |
|--------|-------------|
| **Dev Cube** | Switch to development cube project |
| **Amino Acids** | Switch to amino acids display |

The buttons highlight on hover and trigger a project switch when clicked.

## Visual Feedback

### Object Highlighting

When you hover over an interactive object:

- A **white wireframe cube** appears around the object
- This indicates the object can be picked up

When holding an object:

- The white highlight persists
- The object follows your cursor

### Cursor States

| State | Behavior |
|-------|----------|
| **Normal** | Cursor visible, can interact with UI |
| **Captured** | Cursor hidden/locked, can look around and interact with objects |

## Layout

```
+--------------------------------------------------+
|  [File]  [Default]                    (Tab Bar) |
+--------------------------------------------------+
|                                                  |
|                                                  |
|              3D Viewport                         |
|         (Molecular Visualization)                |
|                                                  |
|                                                  |
+--------------------------------------------------+
```

The 3D viewport occupies the entire window background. The UI overlays are minimal and non-intrusive.

## Design

The interface uses a dark theme:

- **Background**: Dark gray (#121212)
- **Text**: Light gray (#f2f2f2)
- **Buttons**: Blue-gray with hover highlights
- **Active elements**: Lighter background for selected items

This ensures good contrast for viewing molecular structures while providing a modern, clean aesthetic.
