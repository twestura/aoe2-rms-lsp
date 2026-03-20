```rms
actor_area ACTOR_AREA_ID
```

---

Assigns an actor area to the placed object with the specified id number.

- This area can be referenced in future objects using [actor_area_to_place_in](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.d6d6k8uc57zx) or [avoid_actor_area](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.cgoa0e8x398u).
- Use along with [actor_area_radius](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.z9i7h4jrjeaf) to control the size of the area.

**Arguments**:
- `ACTOR_AREA_ID`: A number (default 0, no actor area).

**Example**: Spawn a Wolf next to each Relic.
```rms
<OBJECTS_GENERATION>
create_object RELIC {
  number_of_objects 5
  set_gaia_object_only
  temp_min_distance_group_placement 35
  actor_area 1234
}
create_object WOLF {
  number_of_objects 9320
  set_gaia_object_only
  actor_area_to_place_in 1234
  temp_min_distance_group_placement 25
}
```

**See more**: [actor_area](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.obv5ypy66a57)
