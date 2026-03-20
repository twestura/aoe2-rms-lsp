```rms
set_scaling_to_map_size
```

---

Scales [number_of_groups](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.t7eg3wg2xm3w) to the map size. The unscaled value is relative to a 100x100 map.
See the [map sizes reference](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.qannz915qgy5) for scaling ratios.

If no grouping is present, scaling applies to [number_of_objects](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.btcx5h61er65) instead.

**Mutually exclusive with**: [set_scaling_to_player_number](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.l48a16uing0q)

**Example**: Create clumps of 10 gold and scale the number of groups to map size.
```rms
<OBJECTS_GENERATION>
create_object GOLD {
  number_of_objects 10
  number_of_groups 3
  set_scaling_to_map_size
  set_tight_grouping
}
```

**See more**: [set_scaling_to_map_size](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.ctsq8l5z99u6)
