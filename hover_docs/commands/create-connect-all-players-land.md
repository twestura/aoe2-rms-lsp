```rms
create_connect_all_players_land {
  /* Attributes */
}
```

---

Generates connections between the origins of all player lands (including all lands assigned to players).

**Attributes**:
- [default_terrain_replacement](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.p16vd5cszmhm)
- [replace_terrain](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.5h9ggnuativl)
- [terrain_cost](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.pw0ckpmic7kh)
- [terrain_size](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.o6c6lz9vb2w7)

**Example**: Connect all players with a dirt path.
```rms
<LAND_GENERATION>
create_player_lands {
  terrain_type DESERT
  number_of_tiles 100
}

<CONNECTION_GENERATION>
create_connect_all_players_land {
  default_terrain_replacement DIRT
}
```

**See more**: [create_connect_all_players_land](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.br9ypglw81m2)
