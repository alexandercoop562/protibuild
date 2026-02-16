# Controls

Protibuild uses an intuitive control scheme that allows you to navigate the 3D space and interact with molecular structures efficiently.

## Cursor Capture

Before you can look around or move, you need to capture the mouse cursor:

| Key | Action |
|-----|--------|
| **C** | Capture/lock cursor to enable camera movement |
| **Escape** | Release cursor back to normal mode |

> **Important**: You must capture the cursor (press C) before you can use WASD movement or look around with the mouse. Release the cursor (Escape) when you need to interact with UI elements like the File menu.

## Camera Movement

### Mouse Look
- **Mouse Movement** - Look around (only when cursor is captured)
- Movement is relative to your current viewing direction

### Navigation Keys

| Key | Action |
|-----|--------|
| **W** | Move forward |
| **S** | Move backward |
| **A** | Move left |
| **D** | Move right |
| **Space** | Move up |
| **Shift** | Move down |

Movement is relative to your current viewing direction:
- **W/S** moves along the horizontal plane (ignoring vertical tilt)
- **A/D** moves sideways
- **Space/Shift** moves vertically

## Object Interaction

### Selecting and Moving Objects

> **Note**: Cursor must be captured to interact with objects.

| Action | Description |
|--------|-------------|
| **Hover** | Move cursor over an object to highlight it with a white outline |
| **Left Click + Hold** | Grab and hold the highlighted object |
| **Release Left Click** | Drop the object at its current position |

When you grab an object, it follows your cursor at a fixed distance. You can move around while holding the object.

### Adjusting Distance

| Action | Description |
|--------|-------------|
| **Scroll Up** | Pull the held object closer to your camera |
| **Scroll Down** | Push the held object farther from your camera |

Distance range: 0.5 to 50 units

## Reference Grid

The 3D grid provides spatial reference:

| Axis | Color |
|------|-------|
| X-axis | Red |
| Y-axis | Green |
| Z-axis | Blue |

The grid automatically updates as you move through the space, always centering on your current position.

## Quick Reference

| Context | Keys |
|---------|------|
| Enable camera controls | C |
| Release cursor / Exit mode | Escape |
| Move forward/backward | W / S |
| Strafe left/right | A / D |
| Move up/down | Space / Shift |
| Grab object | Left Click (while hovering) |
| Drop object | Release Left Click |
| Adjust object distance | Scroll Wheel |

## Tips

- **Capture the cursor** (C) when you want to look around freely and interact with objects
- **Release the cursor** (Escape) when you need to interact with UI elements like the File menu
- Objects can be picked up from a distance - you don't need to be right next to them
- Use the scroll wheel to position objects precisely at your desired distance
- The Dev Cube rotates automatically - try picking it up and moving around!
