```rms
group_placement_radius RADIUS
```

---

Specifies the number of tiles out from the central tile that objects belonging to the same group may spawn.

- Activates grouping behavior.
- If [number_of_objects](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.btcx5h61er65) exceeds the available number of tiles, a perfect square of objects is filled.
- If used with a group of objects alongside [set_loose_grouping](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.umboa0q57v9y) and [set_circular_placement](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.15fez3e52vqr), the area is a circle instead of a square.

**Arguments**:
- `RADIUS`: A number (default 3, a 7x7 area).

**Example**: Give each player forage bushes that must stay in a 3x3 area.
```rms
<OBJECTS_GENERATION>
create_object FORAGE {
  number_of_objects 7
  set_tight_grouping
  group_placement_radius 1
  set_gaia_object_only
  set_place_for_every_player
  min_distance_to_players 7
  max_distance_to_players 8
}
```

**See more**: [group_placement_radius](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.675xoyq748tw)
