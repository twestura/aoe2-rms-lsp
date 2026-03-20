```rms
set_scale_by_size
```

---

Scales [number_of_tiles](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.efs6lqjf3k0x) to the map size. The unscaled value is relative to a 100x100 map. See the [map sizes reference](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.qannz915qgy5) for scaling ratios.

- **Bug (AoC/UP/HD)**: the behavior of `set_scale_by_size` and [set_scale_by_groups](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.3vkc0lxd4r4a) is inverted in [create_elevation](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.h85o0pyielaj) — each scales as the other should. Fixed in DE.
- If both `set_scale_by_size` and [set_scale_by_groups](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.3vkc0lxd4r4a) are specified, only the final one applies.
- To scale by both groups and size, use [conditionals](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.vs551a7tyxet) manually.

**Mutually exclusive with**: [set_scale_by_groups](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.3vkc0lxd4r4a)

**Example**: Create 4 hills that grow larger on larger maps.
```rms
<ELEVATION_GENERATION>
create_elevation 3 {
  base_terrain GRASS
  number_of_tiles 400
  number_of_clumps 4
  set_scale_by_size
}
```

**Example 2**: Create 4 lakes that become larger on larger maps. On a small map this command generates 4 clumps with a total of 400 \* 2.1 = 840 tiles.
```rms
<TERRAIN_GENERATION>
  create_terrain WATER {
  base_terrain GRASS
  number_of_clumps 4
  number_of_tiles 400
  set_scale_by_size
}
```

**See more**: [set_scale_by_size (create_elevation)](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.8vbd2ko0sw7f), [set_scale_by_size (create_terrain)](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.g4zvtvsbcm29)
