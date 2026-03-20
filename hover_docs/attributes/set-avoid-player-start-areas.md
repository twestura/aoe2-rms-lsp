```rms
set_avoid_player_start_areas DISTANCE
```

---

The terrain avoids the origins of player lands by the specified number of tiles (with some variance).

- Useful to prevent forests or water terrain from generating directly under the Town Center.
- In DE, the distance can be specified. In pre-DE versions, the distance is fixed.

**Arguments**:
- `DISTANCE`: A number.
  - If the argument name is not used, there is no avoidance.
  - If the argument name is used without a value, defaults to 13.

**Example**: Forest Nothing with small clearings around player starts.
```rms
<TERRAIN_GENERATION>
create_terrain FOREST {
  base_terrain GRASS
  land_percent 100
  number_of_clumps 999
  set_avoid_player_start_areas 12
}
```

**See more**: [set_avoid_player_start_areas](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.ijxhxwahit2u)
