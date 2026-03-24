```rms
terrain_to_place_on TERRAIN
```

---

Restricts object placement to the specified terrain.

**Arguments**:
- `TERRAIN`: A [`terrain constant`](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.3bdjnf7tryyk) (default: any valid terrain).

**Example**: Place decorative rocks on a central desert.
```rms
<LAND_GENERATION>
create_land {
  terrain_type DESERT
  number_of_tiles 500
  land_position 50 50
}

<OBJECTS_GENERATION>
create_object ROCK {
  number_of_objects 300
  terrain_to_place_on DESERT
}
```

**See more**: [terrain_to_place_on](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.t7m0dcvh1px)
