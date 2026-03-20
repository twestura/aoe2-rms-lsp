```rms
create_connect_teams_lands {
  /* Attributes */
}
```

---

Generates connections between the origins of player lands belonging to members of the same team.

 Players are on their own team by default in the scenario editor.
 Use the diplomacy tab to simulate team setups when testing.

**Attributes**:
- [default_terrain_replacement](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.p16vd5cszmhm)
- [replace_terrain](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.5h9ggnuativl)
- [terrain_cost](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.pw0ckpmic7kh)
- [terrain_size](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.o6c6lz9vb2w7)

**Example**: Connect teammates with a road.
```rms
<LAND_GENERATION>
create_player_lands {
  terrain_type DESERT
  number_of_tiles 100
}

<CONNECTION_GENERATION>
create_connect_teams_lands {
  replace_terrain DESERT ROAD
  replace_terrain GRASS ROAD
}
```

**See more**: [create_connect_teams_lands](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.sbxghl4uf1bm)
