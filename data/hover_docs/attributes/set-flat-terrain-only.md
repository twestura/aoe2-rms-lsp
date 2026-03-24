```rms
set_flat_terrain_only
```

---

Prevents the terrain from being placed on sloped tiles.

- The distance from slopes is determined by [spacing_to_other_terrain_types](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.dzdzen1yp2wx).
- Only works when [spacing_to_other_terrain_types](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.dzdzen1yp2wx) is at least 1.

**Example**: Create a hill where the flat top and bottom are desert, but the slope remains grass.
```rms
<ELEVATION_GENERATION>
create_elevation 7 {
  base_terrain GRASS
  number_of_tiles 3000
  number_of_clumps 1
}

<TERRAIN_GENERATION>
create_terrain DESERT {
  base_terrain GRASS
  land_percent 100
  number_of_clumps 9320
  spacing_to_other_terrain_types 1
  set_flat_terrain_only
}
```

**See more**: [set_flat_terrain_only](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.4300mvgw1xz7)
