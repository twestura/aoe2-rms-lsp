```rms
min_terrain_distance DISTANCE
```

---

Minimum distance in cliff units that cliffs stay away from water terrains.

- The unit is cliff units, not tiles. 0 is 0&nbsp;tiles, 1 is 3&nbsp;tiles, 2` is 6&nbsp;tiles, etc.
- Only considers terrains from [LAND_GENERATION](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.15g4dj26anlp). Terrains placed in [TERRAIN_GENERATION](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.2mwmqwe7m0vw) are not yet present when cliffs are placed.

**Arguments**:
- `DISTANCE`: The number of cliff units (default 2).

**Example**: Fill the map with cliffs that stay only 3&nbsp;tiles from water and 0&nbsp;tiles from each other
```rms
<LAND_GENERATION>
base_terrain WATER
create_player_lands {
  terrain_type GRASS
  other_zone_avoidance_distance 10
}

<CLIFF_GENERATION>
min_number_of_cliffs 9999
max_number_of_cliffs 9999
min_distance_cliffs 0
min_terrain_distance 1
```

**See more**: [min_terrain_distance](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.gorf7iar00tm)
