```rms
generate_for_first_land_only
```

---

When multiple [create_player_lands](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.esy5dq29i0wr) commands or multiple lands sharing a [land_id](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.c97q6t5q24lj) exist, this object is placed only on the first applicable land instead of all of them.

Requires [set_place_for_every_player](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.hch89ipgsvb) or [place_on_specific_land_id](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.34wzlujx4lbv).

**Example**: Generate two lands for each player, give each player a house on both lands but a king only on the first.
```rms
<LAND_GENERATION>
base_terrain WATER
create_player_lands {
  land_percent 10
}
create_player_lands {
  land_percent 10
}

<OBJECTS_GENERATION>
create_object HOUSE {
  set_place_for_every_player
}
create_object KING {
  set_place_for_every_player
  generate_for_first_land_only
}
```

**See more**: [generate_for_first_land_only](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.c3yp8xxihcnd)
