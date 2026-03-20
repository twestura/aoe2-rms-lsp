```rms
actor_area_radius RADIUS
```

---

Specifies the size of the actor area created by [actor_area](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.obv5ypy66a57).

- Requires [actor_area](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.obv5ypy66a57).
- If several objects share the same [actor_area](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.obv5ypy66a57) identifier, all of them will share the radius of the first object successfully created.

**Arguments**:
- `RADIUS`: A number (default 1, a 3x3 area).

**Example**: Give each player a Mill with 4&nbsp;Deer within a 7-tile radius.
```rms
<OBJECTS_GENERATION>
create_object MILL {
  set_place_for_every_player
  min_distance_to_players 16
  max_distance_to_players 20
  actor_area 61
  actor_area_radius 7
}
create_object DEER {
  number_of_objects 4
  set_place_for_every_player
  set_gaia_object_only
  actor_area_to_place_in 61
}
```

**See more**: [actor_area_radius](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.z9i7h4jrjeaf)
