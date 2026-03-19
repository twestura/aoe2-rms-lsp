```rms
start_random
  percent_chance % CODE
end_random
```

---

Randomly choose a block of code.

**Arguments**:
- `%`: A percentage, 0&#8288;&ndash;&#8288;100.
- `CODE`: Arbitrary map script code to be included if the corresponding percentage is chosen.

**Quirks**:
- If the percentages sum to less than&nbsp;99%, there is a chance that none of them gets chosen.
- If the sum exceeds&nbsp;99%, only the first&nbsp;99% have a chance of occurring.
- Random blocks cannot be nested.
- If the first branch is `percent_chance 0`, it still may be chosen.
- The 100th&nbsp;percent never is chosen.

**Example**: Randomly generate 5, 6, or 7&nbsp;Gold mines with different probabilities.
```rms
<OBJECTS_GENERATION>
create_object GOLD {
  start_random
    percent_chance 30 number_of_objects 5
    percent_chance 50 number_of_objects 6
    percent_chance 20 number_of_objects 7
  end_random
}
```

**Example 2**: Have a 10%&nbsp;chance of placing 5&nbsp;Gold mines.
```rms
<OBJECTS_GENERATION>
start_random
  percent_chance 10
    create_object GOLD {
      number_of_objects 5
    }
end_random
```

**See more**: [percent_chance](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.10ifrywpiesx)
