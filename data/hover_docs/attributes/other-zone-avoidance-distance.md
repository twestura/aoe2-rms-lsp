```rms
other_zone_avoidance_distance TILES
```

---

Number of tiles away from a land with a different [zone](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.li85mvsiskop) at which land growth stops.
Used to create river maps and island maps.

- To keep two lands separated, both lands must have this attribute.
- When different values are used for two lands, the smaller one applies.
- Also keeps randomly positioned land origins the specified distance apart regardless of zone, but can be overridden by [min_placement_distance](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.6d0lk4yitd95).
- Land origins may end up closer together or even touching if [land_position](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.fnezpf6c85yf) is specified, or if there are too many players for the map size.

**Arguments**:
- `TILES`: A number (default 0).

**Example**: A rivers map where all players are separated by water with a neutral island in the center.
```rms
<LAND_GENERATION>
base_terrain WATER
create_player_lands {
  terrain_type GRASS
  land_percent 100
  other_zone_avoidance_distance 10
}
create_land {
  terrain_type DIRT
  land_percent 100
  land_position 50 50
  zone 1
  other_zone_avoidance_distance 10
}
```

**See more**: [other_zone_avoidance_distance](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.2oebaei6j04a)
