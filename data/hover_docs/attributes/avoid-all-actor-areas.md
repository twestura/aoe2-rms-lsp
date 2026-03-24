```rms
avoid_all_actor_areas
```

---

Prevents the object from being placed within any existing [actor_area](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.obv5ypy66a57) or [create_actor_area](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.u28jmnfojke3).

**Example**: Place Wolves that avoid all actor areas.
```rms
<OBJECTS_GENERATION>
create_object TOWN_CENTER {
  set_place_for_every_player
  max_distance_to_players 0
  actor_area 100
  actor_area_radius 60
}
create_object WOLF {
  number_of_objects 9320
  temp_min_distance_group_placement 52
  avoid_all_actor_areas
}
```

**See more**: [avoid_all_actor_areas](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.5c5n6srms81p)
