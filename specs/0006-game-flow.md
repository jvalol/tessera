# 0006 Game flow

**Status:** implemented
**Date:** 2026-09-20

## Goal

Getting in and out of a game works the way pong and snake do, so all three feel
like the same family.

## Behavior

**No menu.** It opens on a falling piece. There used to be a title with Play
and Quit in front of it, which made sense when a game was the only thing you
could have launched and makes none now: the arcade is the menu, and a splash
screen in front of a game you walked up to and started is a second front door.

**Playing.** The game, per specs 0002 through 0005. Escape quits.

**Paused.** Losing window focus during a game pauses it and shows one line on a
panel. Enter carries on, and Escape quits the way it does while playing, so a
pause is not the one state the key stops working in. Nothing falls while
paused. Losing focus anywhere else changes nothing.

**Game over.** The final score sits on a panel until Enter deals another game.
It used to count five seconds down to the menu; with no menu to return to, it
waits to be asked, the way cascada and carom already do. Escape quits from here.

Starting a game clears the board, the score, the level, the hold slot, and the
bag.

Escape is acted on once per press, so holding it cannot quit twice.

## Acceptance criteria

- A new state is already playing. — `state::tests::a_new_state_is_already_playing`
- Escape quits a game in play. — `system::tests::escape_quits_a_game_in_play`
- Starting a game clears everything. — `system::tests::starting_a_game_clears_the_board`
- Losing focus while playing pauses. — `tessera_game::tests::losing_focus_while_playing_pauses`
- Losing focus while already over changes nothing. — `tessera_game::tests::losing_focus_while_already_over_does_nothing`
- Nothing falls while paused. — `tessera_game::tests::nothing_falls_while_paused`
- Escape leaves a paused game. — `system::tests::escape_leaves_a_paused_game`
- And quits rather than backing out. — `tessera_game::tests::escaping_out_of_a_pause_quits`
- Resuming carries on where it stopped. — `system::tests::resuming_carries_on_where_it_stopped`
- An ended game waits to be asked. — `system::tests::an_ended_game_waits_to_be_asked`
- The pause panel covers nothing the panel needs. — `system::tests::pausing_leaves_the_panel_alone`
- Escape is ignored on key repeat. — `input::tests::escape_ignores_key_repeat`

### Verified by hand

- It opens on a falling piece with nothing in front of it.
- Escape out of a pause, 2026-09-28.

## Out of scope

A pause key. Sound is spec 0007.
