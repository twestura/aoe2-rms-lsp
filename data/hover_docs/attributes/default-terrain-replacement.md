```rms
default_terrain_replacement TERRAIN
```

---

Replaces all terrain along the connection path with the specified terrain.

- Overrides any previously specified [replace_terrain](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.5h9ggnuativl) attributes. Does not override [replace_terrain](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.5h9ggnuativl) attributes that follow it.
- Useful for debugging to visualize all connection paths.

**Arguments**:
- `TERRAIN`: A [terrain constant](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit#heading=h.3bdjnf7tryyk).

**Example**: Replace all connecting terrain with road, but replace water with shallows instead.
```rms
<LAND_GENERATION>
base_terrain WATER
create_player_lands {
  land_percent 100
  other_zone_avoidance_distance 10
}

<CONNECTION_GENERATION>
create_connect_all_players_land {
  default_terrain_replacement ROAD
  replace_terrain WATER SHALLOW
}
```

**Example 2**: Replace everything with ice to visualize connection routes.
```rms
<CONNECTION_GENERATION>
create_connect_all_lands {
  default_terrain_replacement ICE
}
```

**See more**: [default_terrain_replacement](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.p16vd5cszmhm)
