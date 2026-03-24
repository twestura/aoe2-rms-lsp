```rms
temp_min_distance_group_placement DISTANCE
```

---

Like [min_distance_group_placement](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.b2u5jna014lf), but only applies to the current [create_object](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.2vz7nxt2afqo) command. Future objects are unaffected.

- Useful for scattering neutral resources and Relics evenly across the map.
- Can be used together with [min_distance_group_placement](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.b2u5jna014lf).
- If the objects are grouped, distance refers to the center of the group, not individual members.

**Arguments**:
- `DISTANCE`: A number (default 0).

**Example**: Scatter neutral gold evenly across the map.
```rms
<OBJECTS_GENERATION>
create_object GOLD {
  number_of_groups 9320
  number_of_objects 4
  set_gaia_object_only
  set_tight_grouping
  min_distance_group_placement 4
  temp_min_distance_group_placement 46
}
```

**See more**: [temp_min_distance_group_placement](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.a5n8aue3ffi4)
