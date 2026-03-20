```rms
actor_area_to_place_in ACTOR_AREA_ID
```

---

Places the object only within the radius of the specified [actor_area](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.obv5ypy66a57) or [create_actor_area](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.u28jmnfojke3).

- An object can have only one `actor_area_to_place_in`.
- Actor areas have placement intricacies. Follow these guidelines to avoid issues:
  - Different objects can be assigned to the same actor area.
  - Do not place origin-referenced (player or land ID) objects in generic actor areas.
  - Placing generic objects into land ID-referenced actor areas always works.
  - Placing player objects into land ID-referenced actor areas always works.
  - Only player objects should be placed into player-referenced actor areas.
  - When placing generic objects in generic actor areas, minimize the number of [create_object](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.2vz7nxt2afqo) commands between the actor area creation and the object to be placed in it.
  - When none of the above rules can be satisfied, inverse actor areas can be used as a failsafe.

**Arguments**:
- `ACTOR_AREA_ID`: A number.

**Example**: Place a Lumber Camp on the nearest forest and place Villagers there too.
```rms
<OBJECTS_GENERATION>
create_object LUMBER_CAMP {
  set_place_for_every_player
  max_distance_to_players 67
  place_on_forest_zone
  find_closest
  actor_area 8
  actor_area_radius 4
}
create_object VILLAGER {
  set_place_for_every_player
  number_of_objects 4
  actor_area_to_place_in 8
  place_on_forest_zone
  find_closest
}
```

**See more**: [actor_area_to_place_in](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.d6d6k8uc57zx)
