```rms
ignore_terrain_restrictions
```

---

Allows objects to be placed on terrains from which they normally are restricted, and prevents those terrains from acting as a border on placement.

- Requires [set_place_for_every_player](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.hch89ipgsvb) or [place_on_specific_land_id](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.34wzlujx4lbv).
- Can be used in combination with [terrain_to_place_on](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.t7m0dcvh1px).
- Terrain restrictions alternatively can be modified with [effect_amount](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.1niw1edwqhy5) using `ATTR_TERRAIN_ID`, or bypassed using [second_object](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.cr71z3stu8pd) with a placeholder.

**Example**: Place Salmon on the land near each player's Town Center.
```rms
<OBJECTS_GENERATION>
create_object SALMON {
  number_of_objects 4
  set_place_for_every_player
  min_distance_to_players 3
  set_gaia_object_only
  find_closest
  ignore_terrain_restrictions
}
```

**See more**: [ignore_terrain_restrictions](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.fx1jh8glz0tl)
