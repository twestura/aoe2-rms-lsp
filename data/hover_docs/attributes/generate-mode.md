```rms
generate_mode MODE
```

---

Controls whether a land can be positioned randomly in the corners of the map.

By default, lands are positioned in a cross-shaped area and never appear in the corners.
Setting `generate_mode` to 1 allows lands to be positioned anywhere, including the corners.

- No effect for [create_player_lands](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.esy5dq29i0wr).
- No effect when using [assign_to_player](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.mok24wym6kiz) or [assign_to](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.b6uul7n11c6g) for [create_land](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.te4jg2upqvlx), unless [direct_placement](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.jbnjt99zhiqx) is specified in [<PLAYER_SETUP>](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.1jv1nmqnml7h).

**Arguments**:
- `MODE`: 1 to allow corner placement, 0 to disallow (default 0).

**Example**: Create 4 ponds that can appear anywhere, including in the corners.
```rms
<LAND_GENERATION>
create_land { generate_mode 1 terrain_type WATER land_percent 1 }
create_land { generate_mode 1 terrain_type WATER land_percent 1 }
create_land { generate_mode 1 terrain_type WATER land_percent 1 }
create_land { generate_mode 1 terrain_type WATER land_percent 1 }
```

**See more**: [generate_mode](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.4xknxvg2r2hg)
