```rms
create_elevation MAX_HEIGHT {
  /* Attributes */
}
```

---

Creates one or more hills of random height up to `MAX_HEIGHT` (inclusive).

- When creating a single hill, it always attempts to reach `MAX_HEIGHT`.
- Hills with a small number of base tiles cannot reach as high.
- In DE, if terrain is elevated by a land's [base_elevation](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit#heading=h.ppqecxdcopxb), elevation is generated relative to that height. In UP, hills use an absolute height.
- Always include [enable_balanced_elevation](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.izx21xcrwjlg) in DE to reduce the southward bias of hill placement.
- Higher elevations may cause buggy behavior, such as Town Center projectiles missing their targets.
- In versions before DE, the behavior of [set_scale_by_size](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit#heading=h.8vbd2ko0sw7f) and [set_scale_by_groups](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit#heading=h.3vkc0lxd4r4a) is inverted. This bug is fixed in DE.

**Arguments**:
- `MAX_HEIGHT`: A number in 0&#8288;&ndash;&#8288;16 (default 0). Maximum 7 in pre-DE versions.

**Attributes**:
- [base_terrain](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.y6zq54jntrkn)
- [base_layer](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.my6smlk88r2j)
- [number_of_tiles](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.efs6lqjf3k0x)
- [number_of_clumps](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.7u0osqxg1v3m)
- [set_scale_by_size](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.8vbd2ko0sw7f) / [set_scale_by_groups](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.3vkc0lxd4r4a)
- [spacing](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.hqjpcx4o099o)
- [enable_balanced_elevation](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.izx21xcrwjlg)

**Example**: Create one hill on grass terrain.
```rms
<ELEVATION_GENERATION>
create_elevation 7 {
  base_terrain GRASS
  number_of_tiles 600
}
```

**See more**: [create_elevation](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.h85o0pyielaj)
