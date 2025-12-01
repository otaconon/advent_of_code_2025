import os

script_dir = os.path.dirname(os.path.abspath(__file__))
file_path = os.path.join(script_dir, 'input.in')


def load_data():
  with open(file_path) as f:
    data = f.readlines()
  return [(line[0], int(line[1:])) for line in data]


def part_1():
  data = load_data()
  total_rotation = 50
  answer = 0
  decode_direction = {'R': 1, 'L': -1}
  for direction, rotation in data:
    total_rotation += decode_direction[direction] * rotation
    total_rotation %= 100
    answer += total_rotation == 0

  return answer


def brute_2():
  data = load_data()
  total_rotation = 50
  answer = 0
  decode_direction = {'R': 1, 'L': -1}
  for direction, rotation in data:
    for _ in range(rotation):
      total_rotation += decode_direction[direction]
      total_rotation %= 100
      answer += total_rotation == 0

  return answer


def part_2():
  data = load_data()
  current_pos = 50
  answer = 0

  for direction, rotation in data:
    if rotation == 0:
      continue

    if direction == 'R':
      next_pos = current_pos + rotation
      answer += (next_pos // 100) - (current_pos // 100)
      current_pos = next_pos
    else:
      next_pos = current_pos - rotation
      answer += (current_pos - 1) // 100 - (next_pos - 1) // 100
      current_pos = next_pos

  return answer


def main():
  print(part_1())
  print(brute_2())
  print(part_2())


if __name__ == "__main__":
  main()
