```rms
set_tight_grouping
```

---

Objects belonging to the same group must be placed on adjacent tiles.

- Activates grouping behavior.
- Objects larger than one tile that cannot overlap (most buildings) will not be placed when using tight grouping.
- Most placement constraints (e.g. [avoid_forest_zone](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.ym7v1j9vbnle), [min_distance_to_map_edge](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.w2q69orjs3l3), [min_distance_group_placement](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.b2u5jna014lf), [avoid_actor_area](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.cgoa0e8x398u)) apply only to the center of the group, not individual group members. Use [set_loose_grouping](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.umboa0q57v9y) if constraints should apply to group members individually.

**Mutually exclusive with**: [set_loose_grouping](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.umboa0q57v9y)

**Example**: Far player stone.
```rms
<OBJECTS_GENERATION>
create_object STONE {
  number_of_objects 4
  group_placement_radius 2
  set_tight_grouping
  set_gaia_object_only
  set_place_for_every_player
  min_distance_to_players 20
  max_distance_to_players 27
}
```

**See more**: [set_tight_grouping](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.ksq6iglmnili)
