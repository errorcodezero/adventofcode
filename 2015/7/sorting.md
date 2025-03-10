# Sorting Instructions

Since instructions are in the wrong order, they need to be sorted. Instructions will depend on 0-3 values depending on their type. For example:

```
x AND y -> z
^     ^
|     |
```

This instruction depends on `x` and `y` having their values assigned meaning all the instructions need to be in the right order. We can do this by making sure that all instructions are put after their dependencies are declared.

# Actually implemented

I instead chose to use a recursive function since I couldn't get sorting to work.
