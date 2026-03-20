```rms
land_id LAND_ID_NUMBER
```

---

Assigns a numeric label to a land, which can be used to place objects specifically on that land with [place_on_specific_land_id](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.34wzlujx4lbv).
Unrelated to zone numbers.

- Multiple lands can have the same ID. In this case, objects will be placed on all of them.
- Must be used after [assign_to_player](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.mok24wym6kiz) or [assign_to](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.b6uul7n11c6g) since they reset the ID.
- Can theoretically be used for [create_player_lands](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.esy5dq29i0wr), but will disable the ability to use [set_place_for_every_player](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.hch89ipgsvb) for object placement.
- Note that objects may be placed on surrounding terrain rather than the land itself, if the surrounding terrain is one the object can be placed on.
- `land_id -9` assigns the land to be the player land of Gaia.

**Arguments**:
- `LAND_ID_NUMBER`: A number (minimum -10, default: no ID).

**Example**: Create a snowy island and place a gold mine on it.
```rms
<LAND_GENERATION>
base_terrain WATER
create_player_lands {
  terrain_type DIRT
  land_percent 0
}
create_land {
  terrain_type SNOW
  land_percent 0
  land_id 13
  land_position 50 50
}

<OBJECTS_GENERATION>
create_object GOLD {
  place_on_specific_land_id 13
}
```

**See more**: [land_id](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.c97q6t5q24lj)
