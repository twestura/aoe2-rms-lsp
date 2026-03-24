```rms
terrain_cost TERRAIN COST
```

---

Sets the cost of routing a connection through the specified terrain. The connection algorithm prefers lower cost routes, even if they are longer than a more direct higher cost route.

- Can be used multiple times for different terrains.
- If all costs are equal, connections follow straight lines.
- A cost of 0 (or any negative value) prevents the connection from passing through that terrain at all. If a land origin is on such a terrain, or such a terrain must be crossed, the connection to that land is not generated. This can be used to restrict which lands are connected.
- Excessive use of cost 0 can slow down map generation time.
- Accepts floating-point values in DE.

**Arguments**:
- `TERRAIN`: A [terrain constant](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit#heading=h.3bdjnf7tryyk).
- `COST`: A number 0&ndash;4,294,967,296 (default 1).
  - A cost of 0 (or any negative value) prevents the connection from passing through the terrain.
  - Accepts floating-point values in DE.

**Example**: Connections that prefer grass and shy away from forests and deeper water.
```rms
<CONNECTION_GENERATION>
create_connect_all_players_land {
  replace_terrain GRASS ROAD
  replace_terrain FOREST LEAVES
  replace_terrain WATER SHALLOW
  replace_terrain MED_WATER SHALLOW
  replace_terrain DEEP_WATER SHALLOW
  terrain_cost GRASS 1
  terrain_cost FOREST 7
  terrain_cost WATER 7
  terrain_cost MED_WATER 12
  terrain_cost DEEP_WATER 15
}
```

**See more**: [terrain_cost](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.pw0ckpmic7kh)
