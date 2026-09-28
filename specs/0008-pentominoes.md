# 0008 Pentominoes

**Status:** implemented
**Date:** 2026-09-28

## Goal

Pieces of five cells instead of four. Twelve shapes instead of seven, a wider
board to fit them, and clears that go up to five rows.

## Behavior

This replaces spec 0002, which stays on file describing what the game was.

**Twelve pieces, the free pentominoes**, named by the letters they look like:
F, I, L, N, P, T, U, V, W, X, Y, Z. Free means a shape and its mirror are the
same piece, so F and its mirror are one piece and not two. Six of the twelve
are chiral, F L N P Y Z, and taking their mirrors as well would give eighteen,
which is more
than the bag wants to deal and more than a player can hold in their head.

**Each keeps its own colour**, the way the tetrominoes did. Twelve colours have
to stay apart at a glance under the lighting the game already uses, which is
the part of this that cannot be settled by a test.

**A cell is a tessera.** The game is named for the single tile in a mosaic, and
a piece is five of them. Worth using in the code: `Tessera` reads better than
`Cell` and says where the name went.

**Rotation stays what it is**, a quarter turn of each cell about the piece's
origin, four rotations, clockwise and counterclockwise. X does not change when
rotated at all, the way O did not. I and Z have two distinct rotations rather
than four. The other nine have four.

**Wall kicks widen.** The current try is one left, one right, two left, two
right. A pentomino is up to five cells across, so the try goes out to three
each way: one left, one right, two left, two right, three left, three right.
First that fits is taken; none fit and the rotation does not happen.

**The board goes to twelve wide.** Ten was right for a four-cell piece. The I
pentomino is five long, and a ten wide board leaves too little room either side
of it. Twelve keeps the well feeling narrow while letting a piece turn near the
edge. Height stays at twenty visible.

**Clears go to five.** One row 100, two 300, three 500, four 800, five 1200,
each times the level. The jump from four to five is bigger than the step before
it, because clearing five at once needs a five-tall well kept clean.

**A bag of twelve.** All twelve shuffled, dealt one at a time, a fresh bag
after. So a piece cannot be missing for more than twenty-two in a row.

## Acceptance criteria

- Every piece has five cells. — `piece::tests::every_piece_has_five_cells`
- There are twelve pieces. — `piece::tests::there_are_twelve_pieces`
- No two pieces are the same shape, under rotation. — `piece::tests::no_two_pieces_are_the_same_shape`
- No piece is another's mirror. — `piece::tests::no_piece_is_another_s_mirror`
- Each piece has its own colour. — `piece::tests::every_piece_has_its_own_color`
- Rotating four times returns a piece to where it started. — `piece::tests::four_rotations_come_back_around`
- X does not change when rotated. — `piece::tests::x_does_not_change_when_rotated`
- I and Z have two distinct rotations, and the other nine have four. — `piece::tests::each_piece_has_the_rotations_its_symmetry_allows`
- Clockwise and counterclockwise undo each other. — `piece::tests::rotations_undo_each_other`
- A rotation into a wall kicks up to three cells away from it. — `system::tests::rotation_kicks_three_off_the_wall`
- A rotation with nowhere to go does not happen. — `system::tests::blocked_rotation_does_not_happen`
- Every piece fits the board at spawn, in every rotation. — `system::tests::every_piece_spawns_in_every_rotation`
- A bag deals all twelve before repeating. — `bag::tests::a_bag_deals_all_twelve`
- A new bag follows the old one. — `bag::tests::bags_keep_coming`
- Five rows score 1200 times the level. — `util::tests::five_rows_score_twelve_hundred`
- Five full rows really do go at once. — `board::tests::five_rows_clear_at_once`
- An I dropped into a five deep notch clears five and scores for it. — `system::tests::a_five_row_clear_scores_twelve_hundred_in_play`
- The board is twelve wide. — `board::tests::the_board_is_twelve_wide`

### Verified by hand

- Twelve colours stay apart at a glance. — run tessera and play a full bag.
- A piece can be turned against either wall without feeling stuck. — run
  tessera, take a piece to each wall and rotate it there.
- The well still reads as narrow at twelve wide. — run tessera.
- Clearing five rows at once feels earned rather than lucky. — run tessera and
  try for one. Whether the game can reach 1200 at all is settled by
  `a_five_row_clear_scores_twelve_hundred_in_play`, because setting a five deep
  notch up by hand takes long enough that nobody would check it twice.

## Out of scope

The eighteen one-sided pentominoes. Mixing pentominoes with smaller pieces.
Any change to falling speed, hold, or the next queue, which spec 0003 and 0004
leave as they are.
