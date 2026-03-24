```rms
set_loose_grouping
```

---

Objects belonging to the same group can be placed anywhere within the confines of [group_placement_radius](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.675xoyq748tw).

- Activates grouping behavior. Loose grouping is the default type of grouping, so this attribute can be omitted if [group_placement_radius](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.675xoyq748tw) is specified.
- When used with [set_circular_placement](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.15fez3e52vqr), the [group_placement_radius](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.675xoyq748tw) becomes round.
- Most placement constraints (e.g. [avoid_forest_zone](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.ym7v1j9vbnle), [min_distance_to_map_edge](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.w2q69orjs3l3), [min_distance_group_placement](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.b2u5jna014lf), [avoid_actor_area](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.cgoa0e8x398u)) apply to each group member individually. However, the game does not check if there is enough room for the whole group when choosing a location, which can cause some group members to fail to spawn. For important objects with many placement constraints, consider using [set_tight_grouping](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.ksq6iglmnili) or placeholder objects and [actor areas](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.obv5ypy66a57) instead.

**Mutually exclusive with**: [set_tight_grouping](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.ksq6iglmnili)

**Example**: Give players a group of 7 deer.
```rms
<OBJECTS_GENERATION>
create_object DEER {
  number_of_objects 7
  number_of_groups 1
  group_placement_radius 5
  set_loose_grouping
  set_gaia_object_only
  set_place_for_every_player
  min_distance_to_players 14
  max_distance_to_players 22
}
```

**See more**: [set_loose_grouping](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.umboa0q57v9y)
