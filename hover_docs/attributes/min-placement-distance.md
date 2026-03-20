```rms
min_placement_distance TILES
```

---

Number of tiles to stay away from the origins of previously created lands when randomly selecting an origin for this land.

- If not specified, land origins will be positioned such that there is at least [other_zone_avoidance_distance](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.2oebaei6j04a) worth of space between the edges of the square origins.
- `min_placement_distance` uses the center of the origins, so for a large [base_size](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.b9qafcmyygh), origins may end up closer than an equivalent [other_zone_avoidance_distance](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.2oebaei6j04a).
- If too large a value is specified and the land cannot find a valid position, it will be placed at the center regardless of other lands already there.
- No effect when [land_position](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.fnezpf6c85yf) is specified.
- No effect on player lands, unless [direct_placement](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.jbnjt99zhiqx) is active in DE.

**Arguments**:
- `TILES`: A number (default: value of [other_zone_avoidance_distance](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.2oebaei6j04a), which itself has default 0).

**Example**: Create three deserts with origins at least 25 tiles apart.
```rms
<LAND_GENERATION>
create_land {
  terrain_type DESERT
  land_percent 1
  min_placement_distance 25
}
create_land {
  terrain_type DESERT
  land_percent 1
  min_placement_distance 25
}
create_land {
  terrain_type DESERT
  land_percent 1
  min_placement_distance 25
}
```

**See more**: [min_placement_distance](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.6d0lk4yitd95)
