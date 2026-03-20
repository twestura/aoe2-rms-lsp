```rms
place_on_specific_land_id LAND_ID
```

---

Places the object on each land with the specified [land_id](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.c97q6t5q24lj).

- Objects are placed only where they are not separated from the origin of their land by a terrain they are restricted on, unless [ignore_terrain_restrictions](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.fx1jh8glz0tl) is used. Road terrains do not form a separation even though resources cannot be placed on them.
- If [avoid_other_land_zones](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.axswyohsolzw) is specified, the object is placed only on tiles belonging to the land.
- If multiple lands share the same ID, the object is placed on all of them.
- A value of -11 (or -12 in pre-DE versions) places the object at a random position on the map. Smaller values do not place the object at all.

**Mutually exclusive with**: [set_place_for_every_player](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.hch89ipgsvb)

**Arguments**:
- `LAND_ID`: A number (minimum -11).

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
  find_closest
}
```

**See more**: [place_on_specific_land_id](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.34wzlujx4lbv)
