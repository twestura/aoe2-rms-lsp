```rms
assign_to ASSIGN_TARGET NUMBER MODE FLAGS
```

---

A more powerful version of [assign_to_player](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.mok24wym6kiz).
Assigns a land created with [create_land](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.te4jg2upqvlx) to one specific player.

- To support 8 players, lands must be assigned individually for all players.
- Lands assigned to players who are not playing are not created.
- All lands belonging to players will be in a circle and [land_position](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.fnezpf6c85yf) will be ignored, unless [direct_placement](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.jbnjt99zhiqx) is specified in [<PLAYER_SETUP>](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.1jv1nmqnml7h).
- In DE and WK the `AssignTarget` constants are predefined, but in vanilla UP they must be defined manually.

**Mutually exclusive with**: [assign_to_player](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.mok24wym6kiz)

**Arguments**:
- `ASSIGN_TARGET`: An [assign target constant](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit#heading=h.n9ppujqraas6) (default: not assigned to any player).
  - `AT_PLAYER`: assign by player number (lobby order).
  - `AT_COLOR`: assign by player color.
  - `AT_TEAM`: assign by team (lobby order). Teams containing only 1 player do not count as teams.
- `NUMBER`: Varies depending on `ASSIGN_TARGET`.
  - `AT_PLAYER`: 1&ndash;8, player number.
  - `AT_COLOR`: 1&ndash;8, player color.
  - `AT_TEAM`: (-10, -4, -3, -2, -1, 0, 1, 2, 3, 4). Gives the land to a player from the specified team.
    - Team number in lobby order, not the number chosen by the team.
    - 0 is for un-teamed players.
    - Negative values target a player NOT on the specified team.
    - -10 gives the land to any player.
- `MODE`: -1 or 0.
  - 0 is random selection (only matters for `AT_TEAM`).
  - -1 is ordered selection based on lobby order (only matters for `AT_TEAM`).
- `FLAGS`: A number 0&ndash;3 (default 0). Add values to apply multiple flags.
  - 0: no flags.
  - 1: reset players who have already been assigned before starting.
  - 2: do not remember assigning this player.
  - 3: apply the effects of both 1 and 2.

**Example**: Assign a land at the map center to a random player on team 1.
```rms
<PLAYER_SETUP>
direct_placement

<LAND_GENERATION>
create_land {
  terrain_type DIRT
  number_of_tiles 128
  land_position 50 50
  assign_to AT_TEAM 1 0 0
}
```

**See more**: [assign_to](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.b6uul7n11c6g)
