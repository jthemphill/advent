import collections
import typing as ty
import sys

deck1 = collections.deque()
for line in sys.stdin:
    line = line.rstrip()
    if line == 'Player 1:':
        continue
    elif not line:
        break
    deck1.append(int(line))

deck2 = collections.deque()
for line in sys.stdin:
    line = line.rstrip()
    if line == 'Player 2:':
        continue
    deck2.append(int(line))


def score(deck):
    return sum([(len(deck) - i) * card for i, card in enumerate(deck)])


def combat_game(deck1: ty.Deque[int], deck2: ty.Deque[int]) -> ty.Tuple[int, int]:
    seen_scores = set()
    while deck1 and deck2:
        new_scores = (score(deck1), score(deck2))
        if new_scores in seen_scores:
            return 1, new_scores[0]
        seen_scores.add(new_scores)

        card1 = deck1.popleft()
        card2 = deck2.popleft()
        if len(deck1) >= card1 and len(deck2) >= card2:
            winner, _ = combat_game(collections.deque(
                list(deck1)[:card1]), collections.deque(list(deck2)[:card2]))
        else:
            if card1 > card2:
                winner = 1
            else:
                winner = 2
        if winner == 1:
            deck1.append(card1)
            deck1.append(card2)
        else:
            assert winner == 2
            deck2.append(card2)
            deck2.append(card1)

    winner = 1
    winning_deck = deck1
    if not winning_deck:
        assert deck2
        winner = 2
        winning_deck = deck2
    return winner, score(winning_deck)


print(combat_game(deck1, deck2))
