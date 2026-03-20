```rms
create_connect_to_nonplayer_land {
  /* Attributes */
}
```

---

Connects all player lands to all neutral lands, without generating direct connections between individual players.

- **Bug**: Blocks all future connection generation.
- **Bug**: Blocks all team connection generation (except those involving player&nbsp;1) when used after [create_connect_teams_lands](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.sbxghl4uf1bm).

**Attributes**:
- [default_terrain_replacement](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.p16vd5cszmhm)
- [replace_terrain](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.5h9ggnuativl)
- [terrain_cost](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.pw0ckpmic7kh)
- [terrain_size](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.o6c6lz9vb2w7)

**Example**: Connect players to a central desert without connecting them to each other.
```rms
<LAND_GENERATION>
create_player_lands {
  terrain_type DIRT2
  number_of_tiles 100
}
create_land {
  terrain_type DESERT
  number_of_tiles 500
  land_position 50 50
}

<CONNECTION_GENERATION>
create_connect_to_nonplayer_land {
  replace_terrain GRASS ROAD2
}
```

**See more**: [create_connect_to_nonplayer_land](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.ypcxynpvljp0)
