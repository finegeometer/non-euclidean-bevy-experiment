# Non-Euclidean Bevy Experiment

What does it take to implement a non-euclidean game in Bevy 5.0? This experiment aims to find out.

`bevy_euclidean_example` is a trivial project, meant to act as a control in this experiment.
It just shows a cubical arrangement of cubes.

`bevy_non_euclidean_example` is the same thing, but in positively curved space.
This time, the cubes are arranged into a tetrahedron with all right angles; a structure that only makes sense in curved space.

The rest of the crates are modifications of Bevy's crates, to make them work with non-Euclidean geometry.

## Controls

WASD, Space, Shift to move. Move the mouse to look around. Press Escape to quit.

## Non-Euclidean effects

In the non-Euclidean version, cubes in the distance often seem to be too large, or even inverted.
This happens because the curved space acts like a lens, and focuses the light toward you.

## Problems

These crate modifications should not be used for real projects.
- The parts not used in my example project have not been not tested.
- I do not expect my non-Euclidean crates to work if you are also using the Euclidean ones, because some types and systems will be registered twice.
    - A real project would need that; the menu should be Euclidean!
- I only support curvature +1.
    - This means the radius of the universe is 1, which is awkward, because that's roughly the size of Bevy's default objects.
    - This also means I don't support negatively curved (aka hyperbolic) spaces.
- There's no physics or collision detection. That would require cloning `bevy-rapier` as well!


