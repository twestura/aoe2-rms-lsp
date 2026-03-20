```rms
height_limits MIN MAX
```

---

Restricts terrain placement to tiles within a specified elevation range (inclusive).

**Arguments**:
- `MIN`: The minimum height, inclusive.
- `MAX`: The maximum height, inclusive.

**Example**: Create a hill and place desert only on the slopes, leaving the top and bottom as grass.
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
  height_limits 1 6
}
```

**See more**: [height_limits](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.oezholffksgg)
