```rms
enable_tile_shuffling
```

---

Increases randomness of object positions by shuffling the list of candidate tiles rather than using the first entry.

- When using both [find_closest](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.kzkb7o2yhtk9) and [set_circular_placement](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.15fez3e52vqr), add this attribute to prevent objects from being in predictable positions.
- Does not prevent the bias toward the west when [min_distance_to_players](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.asuv2zzhmbsd) and [max_distance_to_players](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.v2aq68odkdzl) are close or equal.
- Should not be used when placing objects at a specific precise location, such as herdables or Villagers under the town center.

**Example**: Create 4 Gold mines randomly positioned in a circle around players, with no positional bias.
```rms
<OBJECTS_GENERATION>
create_object GOLD {
  set_place_for_every_player
  set_gaia_object_only
  number_of_objects 4
  min_distance_to_players 12
  set_circular_placement
  find_closest
  enable_tile_shuffling
}
```

**See more**: [enable_tile_shuffling](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.ewsg05tiznb0)
