```rms
group_variance VARIANCE
```

---

Randomly varies [number_of_objects](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.btcx5h61er65) by up to the specified amount. The maximum variance in the positive direction is reduced by 1.

- A minimum of 1 object will always be generated, even if the variance would make the count 0 or negative.
- Each group varies independently, so this is not suitable for ensuring fair player starting resources. Consider [random code](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.87mv66lnefdm) for such cases.

**Arguments**:
- `VARIANCE`: A number (default 0).

**Example**: Create 10 patches of 2&ndash;7 forage bushes each.
```rms
<OBJECTS_GENERATION>
create_object FORAGE {
  number_of_objects 5
  number_of_groups 10
  group_variance 3
  set_tight_grouping
}
```

**See more**: [group_variance](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.u5a62pn5z0hm)
