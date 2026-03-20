```rms
terrain_size TERRAIN RADIUS VARIANCE
```

---

When a connection passes through a tile of the specified terrain, the area within `RADIUS` ± `VARIANCE` tiles is subject to [replace_terrain](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.5h9ggnuativl) and [default_terrain_replacement](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.p16vd5cszmhm).

- Can be used multiple times for different terrains.
- A radius of 0 still replaces a single-tile width path.
- Variance is selected randomly for each tile crossed.
- If variance is larger than radius, it can reduce the effective radius to a negative value, in which case no terrain will be replaced around those specific tiles.

**Arguments**:
- `TERRAIN`: A [terrain constant](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit#heading=h.3bdjnf7tryyk).
- `RADIUS`: A number (default 1).
- `VARIANCE`: A number (default 0).

**Example**: Connect players with a variable ragged-looking road, and with shallows that are slightly wider.
```rms
<CONNECTION_GENERATION>
create_connect_all_players_land {
  replace_terrain GRASS ROAD
  replace_terrain WATER SHALLOW
  terrain_size GRASS 1 1
  terrain_size WATER 3 1
}
```

**See more**: [terrain_size](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.o6c6lz9vb2w7)
