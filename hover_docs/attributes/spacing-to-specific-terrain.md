```rms
spacing_to_specific_terrain TERRAIN DISTANCE
```

---

Minimum distance that this terrain stays away from a specific terrain.

- Can be used multiple times to avoid multiple terrain types.
- Only considers terrains existing at the time of generation. Terrains generated later need their own spacing.
- Cannot be used to avoid the terrain currently being placed. Doing so prevents the terrain from being placed at all.
- [spacing_to_other_terrain_types](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.dzdzen1yp2wx) takes precedence if larger.

**Arguments**:
- `TERRAIN`: A [terrain constant](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit#heading=h.3bdjnf7tryyk).
- `DISTANCE`: A number (default 0).

**Example**: Generate forest that stays away from various terrains.
```rms
<TERRAIN_GENERATION>
create_terrain FOREST {
  base_terrain GRASS
  land_percent 20
  number_of_clumps 30
  spacing_to_specific_terrain WATER 15
  spacing_to_specific_terrain SHALLOW 8
  spacing_to_specific_terrain ICE 6
  spacing_to_specific_terrain DESERT 3
}
```

**See more**: [spacing_to_specific_terrain](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.bbf5cnk59hw)
