# 0009 A picture of the board

**Status:** implemented
**Date:** 2026-11-03

## Goal

The frame tessera puts on its readme is held against a picture of what that
frame should be.

## Why it belongs here

Specs 0001 to 0008 are arithmetic: where a piece sits, how it turns, when a
row clears, which twelve shapes the bag holds. None of them can see the
board.

**The engine's first harness could not either.** tessera pushes every quad
into `Geometry` and never touches `Scene`, so the 3D call photographs a
black frame of it. The engine's spec 0078 is the call that takes the
overlay, and the tighter share that goes with a game drawn flat.

## Behavior

**`TesseraGame::new` takes `staged` rather than reading the arguments.** A
picture test asks for the photographed state without `--screenshot` in its
own command line.

**A staged run deals from `Bag::in_order`.** Spec 0008's bag shuffles, which
is right for playing and wrong for photographing: the picture is of the
board the run leaves behind, so a fresh shuffle means no two staged runs
agree and every regenerated screenshot is a different board. `in_order`
deals the same twelve in the order `SHAPES` lists them and never shuffles.
Only the staged path uses it, so nothing about playing changes. This does
change what `refresh-screenshots` photographs, so tessera's committed
screenshot moves the next time it is taken.

**One `update`, at no time at all.** The pose plays the whole staged run on
its own fixed step inside the first update and then holds, so a second call
would find `played` set and move nothing.

**It is the engine's size**, 320 by 240, and the engine's `SPARSE` share,
which the overlay call picks on its own.

## Acceptance criteria

- The board after a staged run matches its
  picture. — `tessera_game::tests::the_board_after_a_staged_run_looks_right`

### Verified by hand

- **The picture itself.** The well with a piece settled at the bottom and
  one falling into it, the hold and next panels either side, the score,
  level and rows down the left. It reads at this size, which not every
  game's does.
- **A control.** The staged run cut from eighteen seconds to sixteen point
  two failed it, at 0.3%.
- **That it is repeatable.** With the bag in order, a fresh run comes back
  identical to the reference byte for byte. Before this it could not have
  been: the board was a different shuffle every time.

## Out of scope

**The menu, the pause screen and the game over screen**, each of which has
its own state and its own tests.

**Seeding the shuffle.** A seeded bag would be a second way to deal pieces
and a number to keep. Dealing them in the order they are already written
down needs neither.
