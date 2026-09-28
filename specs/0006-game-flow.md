# 0006 Game flow

**Status:** implemented
**Date:** 2026-09-20

## Goal

Getting in and out of a game works the way pong and snake do, so all three feel
like the same family.

## Behavior

**Menu.** TESSERA with Play and Quit. Up and Down move between them, Enter
chooses, and Escape quits.

**Playing.** The game, per specs 0002 through 0005. Escape returns to the menu
and throws the board away.

**Paused.** Losing window focus during a game pauses it and shows Paused with
Resume. Enter resumes, and Escape leaves for the menu the way it does while
playing, so a pause is not the one state the key stops working in. Nothing
falls while paused. The hold label is hidden, because the overlay writes Paused
into the corner it sits in. Losing focus anywhere else changes nothing.

**Game over.** The final score is shown for five seconds, then the game returns
to the menu. Escape quits from here.

Starting a game clears the board, the score, the level, the hold slot, and the
bag.

Escape is acted on once per press, so holding it doesn't carry from a game into
the menu and quit.

## Acceptance criteria

- Escape during a game returns to the menu. — `system::tests::escape_returns_to_the_menu`
- Escape from the menu quits. — `system::tests::escape_quits_from_the_menu`
- Starting a game clears everything. — `system::tests::starting_a_game_clears_the_board`
- Losing focus while playing pauses. — `tessera_game::tests::losing_focus_while_playing_pauses`
- Losing focus on the menu changes nothing. — `tessera_game::tests::losing_focus_on_the_menu_does_nothing`
- Nothing falls while paused. — `tessera_game::tests::nothing_falls_while_paused`
- Escape leaves a paused game. — `system::tests::escape_leaves_a_paused_game`
- The menu it arrives at reads like the menu, not like the pause it came from.
  — `tessera_game::tests::escaping_out_of_a_pause_arrives_at_a_real_menu`
- The hold label goes away while paused. — `system::tests::pausing_hides_the_hold_label`
- Escape is ignored on key repeat. — `input::tests::escape_ignores_key_repeat`

The menu one is not decoration. Pong had the same gap and its menu came up
still reading "Paused" and "Resume", because nothing could reach the menu
before from a state that had overwritten them.

### Verified by hand

- The five second game over wait feels right. — run tessera and top out.
- Escape out of a pause, 2026-09-28.

## Out of scope

A pause key, and restarting without going through the menu. Sound is spec 0007.
