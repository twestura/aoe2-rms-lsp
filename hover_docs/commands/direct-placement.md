```rms
direct_placement
```

---

Allows the [land_position](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit#heading=h.fnezpf6c85yf) attribute in [create_land](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit#heading=h.te4jg2upqvlx) to be used in combination with the [assign_to_player](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit#heading=h.mok24wym6kiz) or [assign_to](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit#heading=h.b6uul7n11c6g) attributes to position players at exact positions on the map.

- `direct_placement` does not necessarily require [land_position](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit#heading=h.fnezpf6c85yf). It is possible just to specify borders instead.
- If used with [create_player_lands](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit#heading=h.esy5dq29i0wr), these lands are positions entirely at random (ignoring the default circular positioning).

**Mutually exclusive with** [grouped_by_team](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.31ppsgfv9i7y), [random_placement](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.l4ykw7wa466e)

**Example**: Place player&nbsp;1 at the center of the map&nbsp;(50%,&nbsp;50%).

```rms
<PLAYER_SETUP>
direct_placement

<LAND_GENERATION>
create_land {
  terrain_type DESERT
  land_position 50 50
  land_percent 3
  assign_to_player 1
}
```

**See more**: [direct_placement](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.jbnjt99zhiqx)
