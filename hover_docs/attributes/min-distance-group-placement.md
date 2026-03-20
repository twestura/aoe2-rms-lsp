```rms
min_distance_group_placement DISTANCE
```

---

Minimum distance in tiles that individual objects of the same [create_object](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.2vz7nxt2afqo) command, and all future objects, must stay away from each placed object.

- Best used with small values to keep different resources from being directly adjacent to each other.
- To scatter objects from the same command far apart without affecting future objects, use [temp_min_distance_group_placement](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.a5n8aue3ffi4).
- If the objects are grouped, distance refers to the center of the group, not individual members.

**Arguments**:
- `DISTANCE`: A number (default 0).

**Example**: Give each player two sets of forages that avoid each other and keep all future objects 4 tiles away.
```rms
<OBJECTS_GENERATION>
create_object FORAGE {
  number_of_objects 7
  number_of_groups 2
  set_tight_grouping
  set_place_for_every_player
  set_gaia_object_only
  min_distance_to_players 8
  max_distance_to_players 10
  min_distance_group_placement 4
}
```

**See more**: [min_distance_group_placement](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.b2u5jna014lf)
