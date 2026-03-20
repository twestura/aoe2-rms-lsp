```rms
land_position X Y
```

---

Specifies the exact origin point of a land as a percentage of total map dimensions.

- `50 50` is the center of the map.
- `0 0` is the west corner, `99 0` is the north corner, `99 99` is the east corner, `0 99` is the south corner.
- Ignores border restrictions. If placed outside specified borders, the land will not grow beyond its [base_size](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.b9qafcmyygh).
- Positions outside the map bounds are valid as long as crash conditions are avoided.
- Accepts floating-point values in DE.
- Disabled for [create_player_lands](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.esy5dq29i0wr), unless [direct_placement](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.jbnjt99zhiqx) is active.
- Disabled when using [assign_to_player](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.mok24wym6kiz) or [assign_to](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.b6uul7n11c6g), unless [direct_placement](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.jbnjt99zhiqx) is active.

**Arguments**:
- `X`: A number 0&ndash;99. Percentage of map width.
- `Y`: A number 0&ndash;99. Percentage of map height.

**Example**: Create a lake at the center of the map.
```rms
<LAND_GENERATION>
create_land {
  terrain_type WATER
  land_percent 10
  land_position 50 50
}
```

**See more**: [land_position](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.fnezpf6c85yf)
