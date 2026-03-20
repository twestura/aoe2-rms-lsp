```rms
number_of_tiles NUMBER
```

---

Fixed number of tiles allocated to this command.

- In [create_player_lands](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.esy5dq29i0wr) and [create_land](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.te4jg2upqvlx): the growth size of the land beyond the [base_size](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.b9qafcmyygh).
  - With [behavior_version](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.seeuqpcozayb) 0: the square origin is not included in the total.
  - With [behavior_version](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.seeuqpcozayb) 1 or 2: the square origin is included in the total.
  - For [create_player_lands](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.esy5dq29i0wr), each player is given the specified number of tiles.
- In [create_elevation](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.h85o0pyielaj): total number of tiles elevated by this command. If [number_of_clumps](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.7u0osqxg1v3m) is specified, this value is divided equally among the clumps.
- In [create_terrain](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.acnibljecbfg): total number of terrain tiles painted in this command. If [number_of_clumps](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.1tzwe1brcw80) is specified, this value is divided equally among the clumps.

**Mutually exclusive with**: [land_percent](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.ipg3ruf70nn4)

**Arguments**:
- `NUMBER`: A number (default: [land_percent](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.ipg3ruf70nn4) 100 in land generation; approximately 120 on a tiny map in elevation and terrain generation).

**Example**: Give every player 300 tiles.
```rms
<LAND_GENERATION>
create_player_lands {
  terrain_type DIRT
  number_of_tiles 300
}
```

**Example 2**: Create one hill on grassy terrain.
```rms
<ELEVATION_GENERATION>
create_elevation 7 {
  base_terrain GRASS
  number_of_tiles 600
}
```

**Example 3**: Create a 500-tile lake.
```rms
<TERRAIN_GENERATION>
create_terrain WATER {
  base_terrain GRASS
  number_of_tiles 500
  set_avoid_player_start_areas
}
```

**See more**: [number_of_tiles (create_player_lands / create_land)](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.bzvr6x5i10na), [number_of_tiles (create_elevation)](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.efs6lqjf3k0x), [number_of_tiles (create_terrain)](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.qdz0o9mtb2hi)
