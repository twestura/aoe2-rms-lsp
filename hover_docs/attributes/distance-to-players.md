```rms
min_distance_to_players DISTANCE
max_distance_to_players DISTANCE
```

---

Minimum and maximum distance in tiles from the origin of player lands that the object can be placed.

- It is not necessary to specify both attributes.
- Distances use a square radius by default. Use [set_circular_placement](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.15fez3e52vqr) for circular (Euclidean) distance.
- If the objects are grouped, distance refers to the center of the group, not individual members.
- When used with [place_on_specific_land_id](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.34wzlujx4lbv), distances refer to that specific land.
- When used without [set_place_for_every_player](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.hch89ipgsvb) or [place_on_specific_land_id](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.34wzlujx4lbv), `max_distance_to_players` has no effect.
- **Bug**: if distances are very constrained (e.g. min = max), objects are noticeably biased toward being placed in the west.
- **Bug (pre-DE)**: `min_distance_to_players` always applies to all lands, not just player lands.

**Arguments**:
- `DISTANCE`: A number (default: no limits).
  - If `max_distance_to_players` is negative, no limits apply.
  - If `min_distance_to_players` exceeds `max_distance_to_players`, no objects are placed.

**Example**: Place the starting scout at a distance of 7&ndash;9 tiles.
```rms
<OBJECTS_GENERATION>
create_object SCOUT {
  set_place_for_every_player
  min_distance_to_players 7
  max_distance_to_players 9
}
```

**See more**: [min_distance_to_players](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.asuv2zzhmbsd), [max_distance_to_players](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.v2aq68odkdzl)
