```rms
second_object OBJECT
```

---

Specifies an object to be placed on top of the main object.

- If placing multiple objects, each receives the specified second object.
- `second_object` can be used to bypass terrain restrictions by using an invisible placeholder object as the main object.
  - Alternatively, terrain restrictions of an object can be changed with [effect_amount](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.1niw1edwqhy5) or removed entirely with [ignore_terrain_restrictions](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.fx1jh8glz0tl).
- Unrescuable status from [set_gaia_unconvertible](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.g4mzdyb4izbo) does not apply to `second_object`.

**Arguments**:
- `OBJECT`: An [`object constant`](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.nvxriamulybh).

**Example**: Players start with a cow underneath their town center.
```rms
<OBJECTS_GENERATION>
create_object TOWN_CENTER {
  set_place_for_every_player
  max_distance_to_players 0
  second_object DLC_COW
}
```

**See more**: [second_object](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.cr71z3stu8pd)
