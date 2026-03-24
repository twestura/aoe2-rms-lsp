```rms
assign_to_player PLAYER_NUMBER
```

---

Assigns a land created with [create_land](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.te4jg2upqvlx) to one specific player, allowing starting objects to be placed on that land for that player.

- Refers to lobby order: the first person in the lobby is player 1, even if they are not blue.
- To support 8 players, lands must be assigned individually to all 8 players.
- Lands assigned to players who are not playing not created.
- All lands belonging to players will be in a circle and [land_position](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.fnezpf6c85yf) will be ignored, unless [direct_placement](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.jbnjt99zhiqx) is specified in [<PLAYER_SETUP>](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.1jv1nmqnml7h).
- `assign_to_player 0` assigns the land to the Gaia player (not recommended).
- Negative values create the land without assigning it to anyone.

**Mutually exclusive with**: [assign_to](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.b6uul7n11c6g)

**Arguments**:
- `PLAYER_NUMBER`: A number 1&ndash;8 (default: not assigned to any player).

**Example**: Assign a desert land to player 1.
```rms
<LAND_GENERATION>
create_land {
  terrain_type DESERT
  land_percent 3
  assign_to_player 1
}
```

**See more**: [assign_to_player](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.mok24wym6kiz)
