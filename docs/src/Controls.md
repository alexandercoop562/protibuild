# Controls

Protibuild uses an intuitive control scheme that allows you to navigate the 3D space and interact with protein structures efficiently.

## Camera Movement

### Mouse Look
- **Left Click** - Capture mouse cursor and enable camera movement
- **Mouse Movement** - Look around (when mouse is captured)
- **Escape** - Release mouse cursor

### Navigation Keys

| Key | Action |
|-----|--------|
| **W** | Move forward |
| **S** | Move backward |
| **A** | Move left |
| **D** | Move right |
| **Space** | Move up |
| **Shift** | Move down |

Movement is relative to your current viewing direction. The W and S keys move you along the horizontal plane (ignoring vertical tilt), while A and D move you sideways.

## Object Interaction

### Selecting and Moving Objects
- **Hover** over an object to highlight it
- **Left Click + Hold** on an object to grab it
- **Release Left Click** to drop the object

When you grab an object, it will follow your cursor at a fixed distance. You can move around while holding the object.

### Adjusting Distance
- **Scroll Wheel** - Adjust the distance of the held object from your camera
  - Scroll up to pull the object closer
  - Scroll down to push the object farther away
  - Distance range: 0.5 to 50 units

## Grid Reference

The 3D grid provides spatial reference:
- **Red line** - X-axis
- **Green line** - Y-axis  
- **Blue line** - Z-axis

The grid automatically updates as you move through the space, always centering on your current position.

## Tips

- **Capture the mouse** when you want to look around freely
- **Release the mouse** (Escape) when you need to interact with UI elements
- Objects can be picked up from a distance - you don't need to be right next to them
- Use the scroll wheel to position objects precisely at your desired distance
