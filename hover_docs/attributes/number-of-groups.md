```rms
number_of_groups NUMBER
```

---

Places the specified number of groups, each consisting of the number of individual objects specified by [number_of_objects](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.btcx5h61er65).

Total objects placed = [number_of_objects](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.btcx5h61er65) \* `number_of_groups`.


**Arguments**:
- `NUMBER`: A number (default: individual objects, no groups).
  - A maximum of 9,320 should be used when also specifying [set_scaling_to_map_size](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.ctsq8l5z99u6).

**Example**: Place 20 groups of 5 Boars each.
```rms
<OBJECTS_GENERATION>
create_object BOAR {
  number_of_objects 5
  number_of_groups 20
}
```

**See more**: [number_of_groups](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.t7eg3wg2xm3w)
