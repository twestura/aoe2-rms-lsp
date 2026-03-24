```rms
spacing_to_other_terrain_types DISTANCE
```

---

Minimum distance that this terrain stays away from other terrain types.

- Only considers terrains existing at the time of generation. Terrains generated later need their own spacing.
- Terrains do not stay away from the same terrain type created previously. Use an intermediate placeholder terrain to achieve this.
- Also affects the distance the terrain stays away from cliffs, since cliffs generate their own terrain underneath them.
- When used with [set_flat_terrain_only](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.4300mvgw1xz7), also affects the distance the terrain stays away from slopes.

**Arguments**:
- `DISTANCE`: A number (default 0).

**Example**: Create a lake, then fill the rest of the map with forest that stays 10 tiles away from the water.
```rms
<TERRAIN_GENERATION>
create_terrain WATER {
  base_terrain GRASS
  land_percent 10
}
create_terrain FOREST {
  base_terrain GRASS
  spacing_to_other_terrain_types 10
  land_percent 100
}
```

**See more**: [spacing_to_other_terrain_types](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.dzdzen1yp2wx)
