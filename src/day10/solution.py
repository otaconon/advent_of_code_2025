from typing import List, Tuple
import numpy as np
import sympy as sp
from scipy.optimize import linprog


def solve_min_sum(schemes, jolt) -> int:
  A = np.array(schemes).T
  b = np.array(jolt)

  c = np.ones(A.shape[1])

  res = linprog(c, A_eq=A, b_eq=b, bounds=(0, None), integrality=1)

  if res.success:
    solution = np.round(res.x).astype(int)
    return solution.sum()
    print(f"Found minimal integer solution: {solution}")
    print(f"Minimal Sum: {solution.sum()}")
  else:
    print("No integer solution found.")
    return 0


def star2():
  with open("input/day10/input.in") as f:
    raw_data = f.read()
  data = parse_input(raw_data)

  ans = 0
  for _, schemes, jolt in data:
    ans += solve_min_sum(schemes, jolt)
  print(ans)


def parse_input(raw_data: str) -> List[Tuple[int, List[List[int]], List[int]]]:
  data = []

  for line in raw_data.strip().splitlines():
    parts = line.strip().split()

    if not parts:
      continue

    target_str = parts[0].strip("[]")
    target = 0

    for i, char in enumerate(target_str):
      if char == '#':
        target |= (1 << i)

    schematics = []

    for part in parts[1:-1]:
      clean_part = part.strip("()")

      scheme = []
      if clean_part:
        indices = clean_part.split(",")
        for i in range(len(target_str)):
          if str(i) in indices:
            scheme.append(1)
          else:
            scheme.append(0)

      schematics.append(scheme)

    jolt = [int(x) for x in parts[-1].strip("{}").split(",")]
    data.append((target, schematics, jolt))

  return data


star2()
