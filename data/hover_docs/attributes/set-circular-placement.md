```rms
set_circular_placement
```

---

Changes [min_distance_to_players](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.asuv2zzhmbsd) and [max_distance_to_players](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.v2aq68odkdzl) to use circular (Euclidean) distance rather than a square radius. This prevents resources on the diagonal from being very far away.

- If used for a group of objects with [set_loose_grouping](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.umboa0q57v9y), the area of [group_placement_radius](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.675xoyq748tw) becomes round.
- If used with [place_on_specific_land_id](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.34wzlujx4lbv), then [min_distance_to_players](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.asuv2zzhmbsd) does not work.

**Example**: Use circular placement for player sheep spawns.
```rms
<OBJECTS_GENERATION>
create_object SHEEP {
  number_of_objects 2
  number_of_groups 2
  set_gaia_object_only
  set_place_for_every_player
  min_distance_to_players 18
  max_distance_to_players 23
  set_circular_placement
}
```

**See more**: [set_circular_placement](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.15fez3e52vqr)
