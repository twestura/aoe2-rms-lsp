```rms
create_connect_all_lands {
  /* Attributes */
}
```

---

Generates connections between the origins of all lands, including neutral lands.

**Attributes**:
- [default_terrain_replacement](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.p16vd5cszmhm)
- [replace_terrain](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.5h9ggnuativl)
- [terrain_cost](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.pw0ckpmic7kh)
- [terrain_size](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.o6c6lz9vb2w7)

**Example**: Connect all player and neutral islands with shallows.
```rms
<LAND_GENERATION>
base_terrain WATER
create_player_lands {
  terrain_type DESERT
  number_of_tiles 100
}
create_land {
  terrain_type FOREST
  number_of_tiles 100
  land_position 99 1
}
create_land {
  terrain_type PINE_FOREST
  number_of_tiles 100
  land_position 50 50
}

<CONNECTION_GENERATION>
create_connect_all_lands {
  replace_terrain WATER SHALLOW
}
```

**See more**: [create_connect_all_lands](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.wm3xy9f5nbd9)
