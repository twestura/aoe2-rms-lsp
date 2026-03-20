```rms
avoid_actor_area ACTOR_AREA_ID
```

---

Prevents the object from being placed within the specified [actor_area](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.obv5ypy66a57) or [create_actor_area](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.u28jmnfojke3).

- An object can avoid multiple actor areas by using this attribute multiple times.
- An object can specify an [actor_area](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.obv5ypy66a57) and then avoid that same actor area within the same [create_object](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.2vz7nxt2afqo) statement, but only for ungrouped objects or those with [set_loose_grouping](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.umboa0q57v9y).

**Arguments**:
- `ACTOR_AREA_ID`: A number.

**Example**: Place a group of Berries and a group of Deer that avoids it.
```rms
<OBJECTS_GENERATION>
create_object FORAGE {
  number_of_objects 6
  set_tight_grouping
  actor_area 1234
  actor_area_radius 7
}
create_object DEER {
  number_of_objects 4
  set_tight_grouping
  avoid_actor_area 1234
}
```

**Example 2**: Place a barracks for Empire Wars while avoiding several other objects already placed.
```rms
<OBJECTS_GENERATION>
create_object BARRACKS {
  set_place_for_every_player
  min_distance_to_players 7
  max_distance_to_players 9
  avoid_actor_area 94
  avoid_actor_area 40
  avoid_actor_area 8
  avoid_actor_area 9
  avoid_actor_area 99
  actor_area 51
  actor_area_radius 5
}
```

**See more**: [avoid_actor_area](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.cgoa0e8x398u)
