```rms
clumping_factor FACTOR
```

---

The extent to which tiles prefer to clump together near existing tiles of the same land or terrain patch.

- In [create_player_lands](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.esy5dq29i0wr) and [create_land](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.te4jg2upqvlx): moderate values (11&ndash;40) create rounder lands, low values (0&ndash;10) create more irregular lands, and high values (40+) create lands that extend in one direction away from the origin. Negative values create extremely snakey lands and are generally not recommended.
- In [create_terrain](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.acnibljecbfg): moderate values (5&ndash;25) create rounder terrain patches, low values (0&ndash;5) create more irregular patches. Negative values create extremely snakey terrains.

**Arguments**:
- `FACTOR`: A number, lower values clump less, higher values clump more.
  - 0&ndash;40 in land generation (default 8).
  - 0&ndash;25 in terrain generation (default 20).

**Example**: Create an irregularly shaped lake.
```rms
<LAND_GENERATION>
create_land {
  terrain_type WATER
  land_percent 10
  clumping_factor 2
}
```

**Example 2**: Create a regularly shape bamboo forest.
```rms
<TERRAIN_GENERATION>
create_terrain BAMBOO {
  base_terrain GRASS
  number_of_tiles 500
  clumping_factor 20
}
```

**See more**: [clumping_factor (create_player_lands / create_land)](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.7e3knrokkcni), [clumping_factor (create_terrain)](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.mztseaf6qfgt)
