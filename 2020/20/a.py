# pyright: strict

import collections
import math
import re
import sys
from typing import Dict, List, Optional, Set, Tuple

Tile = List[str]

tile_re = re.compile(r'Tile (\d+):')

monster_patterns = [
    r'..................#.',
    r'#....##....##....###',
    r'.#..#..#..#..#..#...',
]

monster_res = [re.compile(x) for x in monster_patterns]

monster_subs = [
    r'..................O.',
    r'O....OO....OO....OOO',
    r'.O..O..O..O..O..O...',
]

N = 0
S = 1
E = 2
W = 3


def get_west_edge(tile: Tile) -> str:
    return ''.join([tile[i][0] for i in range(len(tile))])


def get_east_edge(tile: Tile) -> str:
    return ''.join([tile[i][-1] for i in range(len(tile))])


def edges(tile: Tile) -> List[str]:
    return [tile[0], tile[-1], get_east_edge(tile), get_west_edge(tile)]


def cw(tile: Tile) -> Tile:
    new_tile: Tile = []
    for col in range(len(tile)):
        new_tile.append(''.join([row[col] for row in tile[::-1]]))
    return new_tile


def ccw(tile: Tile) -> Tile:
    return cw(cw(cw(tile)))


def flip_x(tile: Tile) -> Tile:
    return [r[::-1] for r in tile]


def flip_y(tile: Tile) -> Tile:
    return ccw(flip_x(cw(tile)))


def orientations(tile: Tile):
    for _ in range(4):
        tile = cw(tile)
        yield tile
    tile = flip_x(tile)
    for _ in range(4):
        tile = cw(tile)
        yield tile
    tile = flip_y(tile)
    for _ in range(4):
        tile = cw(tile)
        yield tile
    tile = flip_x(tile)
    for _ in range(4):
        tile = cw(tile)
        yield tile


def print_tile_row(tiles: List[Tile]):
    return '\n'.join([''.join([''.join(tile[row]) for tile in tiles])
                      for row in range(len(tiles[0]))])


tiles: Dict[int, Tile] = {}
cur_tile: Tile = []
tile_id = None
for line in sys.stdin:
    line = line.rstrip()
    m = tile_re.match(line)
    if m is not None:
        if tile_id is not None and cur_tile:
            tiles[tile_id] = cur_tile
        tile_id = int(m[1])
        cur_tile = []
    elif line:
        cur_tile.append(line)
if tile_id is not None and cur_tile:
    tiles[tile_id] = cur_tile

n = int(math.sqrt(len(tiles)))
assert n * n == len(tiles)

edge_to_tiles: Dict[str, List[int]] = collections.defaultdict(list)
for k, tile in tiles.items():
    for i, edge in enumerate(edges(tile)):
        edge_to_tiles[edge].append(k)
        edge_to_tiles[edge[::-1]].append(k)


def can_place(tile: Tile, places: Tuple[Optional[Tile]], pos: int):
    if pos >= n:
        north_tile = places[pos - n]
        if north_tile is not None:
            if north_tile[-1] != tile[0]:
                return False
    if pos + n < len(tiles):
        south_tile = places[pos + n]
        if south_tile is not None:
            if south_tile[0] != tile[-1]:
                return False
    if pos % n + 1 < n:
        east_tile = places[pos + 1]
        if east_tile is not None:
            if get_west_edge(east_tile) != get_east_edge(tile):
                return False
    if pos % n > 0:
        west_tile = places[pos - 1]
        if west_tile is not None:
            if get_east_edge(west_tile) != get_west_edge(tile):
                return False
    return True


for t in tiles.values():
    assert t == ccw(cw(t))
    assert t == flip_x(flip_x(t))
    assert t == flip_y(flip_y(t))

unique_edges_per_tile: Dict[int, List[str]] = collections.defaultdict(list)
for tile_key, tile in tiles.items():
    for edge in edges(tile):
        if len(edge_to_tiles[edge]) == 1:
            unique_edges_per_tile[tile_key].append(edge)
            unique_edges_per_tile[tile_key].append(edge[::-1])

placed_tile_keys: List[int] = []
unplaced_tiles: Set[int] = set([t for t in tiles.keys()])
placed_tiles: List[Tile] = []
for tile_key, unique_edges in unique_edges_per_tile.items():
    print(f'{tile_key} has {unique_edges} as its unique edges')
    if len(unique_edges) == 4:
        tile = tiles[tile_key]
        for new_tile in orientations(tile):
            print_tile_row([new_tile])
            if new_tile[0] in unique_edges and get_west_edge(new_tile) in unique_edges:
                placed_tiles.append(new_tile)
                placed_tile_keys.append(tile_key)
                unplaced_tiles.remove(tile_key)
                break
        assert len(placed_tiles) == len(placed_tile_keys) == 1
        break

while unplaced_tiles:
    assert len(tiles) - len(placed_tiles) == len(unplaced_tiles)
    if len(placed_tiles) % n != 0:
        east_edge = get_east_edge(placed_tiles[-1])
        candidates = [t for t in edge_to_tiles[east_edge]
                      if t != placed_tile_keys[-1]]
        assert len(candidates) == 1
        to_place_key = candidates[0]
        for tile_to_place in orientations(tiles[to_place_key]):
            if get_west_edge(tile_to_place) == east_edge:
                placed_tiles.append(tile_to_place)
                unplaced_tiles.remove(to_place_key)
                placed_tile_keys.append(to_place_key)
                break
    else:
        south_edge = placed_tiles[-n][-1]
        candidates = [t for t in edge_to_tiles[south_edge]
                      if t != placed_tile_keys[-n]]
        assert len(candidates) == 1
        to_place_key = candidates[0]
        for tile_to_place in orientations(tiles[to_place_key]):
            if tile_to_place[0] == south_edge:
                placed_tiles.append(tile_to_place)
                unplaced_tiles.remove(to_place_key)
                placed_tile_keys.append(to_place_key)
                break


for i in range(n):
    print(print_tile_row(placed_tiles[n * i:n * i + n]))
    print(placed_tile_keys[n * i:n * i + n])
print("This is the full thing.")

print(
    f"Checksum: {placed_tile_keys[0]} * {placed_tile_keys[n-1]} * {placed_tile_keys[-n]} * {placed_tile_keys[-1]} = {placed_tile_keys[0] * placed_tile_keys[n - 1] * placed_tile_keys[-n] * placed_tile_keys[-1]}")

borderless_tiles = []
for tile in placed_tiles:
    borderless_tiles.append([row[1:-1] for row in tile[1:-1]])

print('\n\n'.join(['\n'.join(t) for t in borderless_tiles]))
print('ugh...')

borderless_image = '\n'.join(
    [print_tile_row(borderless_tiles[n*i:n*i+n]) for i in range(n)]).split('\n')

print('\n'.join(ccw(flip_x(borderless_image))))
print('finding...')


def substitute(match: str, replacement: str):
    ans = ''
    for i in range(len(match)):
        if replacement[i] == 'O':
            assert match[i] == '#'
            ans += 'O'
        else:
            ans += match[i]
    return ans


finished_image = None
nfound = 0
for image in orientations(borderless_image):
    for row in range(len(image) - 2):
        for col in range(len(image[row])):
            matches = [monster_res[i].search(
                image[row + i], col, col + len(monster_patterns[i])) for i in range(3)]
            if all(matches):
                nfound += 1
                for i, match in enumerate(matches):
                    assert match is not None
                    # image[row + i] = (image[row + i][:col + match.start()] + substitute(
                    #     match.group(0), monster_subs[i]) + image[row + i][col + match.end():])
                    assert len(image[row + i]) == len(borderless_image)
    if nfound > 0:
        finished_image = '\n'.join(image)
        break

assert finished_image is not None
print(finished_image)
print(f"Found {nfound} monsters, roughness = {finished_image.count('#') - 15 * nfound}")
