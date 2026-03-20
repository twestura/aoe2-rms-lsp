```rms
number_of_objects NUMBER
```

---

Number of objects to create.


**Arguments**:
- `NUMBER`: A number (default 1).
  - A maximum of 9,320 should be used when also specifying [set_scaling_to_map_size](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.ctsq8l5z99u6).

**Example**: Place 10 individual gold mines on the map.
```rms
<OBJECTS_GENERATION>
create_object GOLD {
  number_of_objects 10
}
```

**See more**: [number_of_objects](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.btcx5h61er65)
