```rms
circle_radius RADIUS VARIANCE
```

---

Positions player lands in a circle with equal distance to the center. Used in [create_player_lands](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.esy5dq29i0wr).

- Circle radius ignores [borders](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.xrncn5cs75or) when placing land origins, but land growth still is constrained by borders.
- If used for multiple [create_player_lands](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.esy5dq29i0wr) commands, only the final radius applies.
- **Bug**: if used for multiple player lands while also using [grouped_by_team](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.31ppsgfv9i7y), the additional land positions do not generate properly.
- If used for [create_land](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.te4jg2upqvlx), it still applies to player lands normally.
- See [Circle Radius Comparison](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit#heading=h.2pk5zk5824si) for a comparison of not using `circle_radius` to a radius with no variance.

**Arguments**:
- `RADIUS`: A number (1&ndash;50). Percentage of map width.
  - 0 disables `circle_radius`.
  - The standard radius for unconstrained lands is around 40.
  - Values larger than 50 tend to force players toward the extreme edges and corners of the map.
- `VARIANCE`: A number (default 0). Distance players can spawn away from the circle.
  - 0 is a perfect circle with no variance.
  - 20 is close to the standard amount of variance when not using `circle_radius`.
  - Very large values tend to force players toward the corners.
  - Each player varies independently.

**Example**: Place player lands in a perfect circle close to the center.
```rms
<LAND_GENERATION>
create_player_lands {
  terrain_type DIRT
  number_of_tiles 100
  circle_radius 20 0
}
```

**See more**: [circle_radius](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.lc3eipzhfx0z)
