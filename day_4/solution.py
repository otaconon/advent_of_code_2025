from pathlib import Path
from typing import Callable
import taichi as ti
import numpy as np
import re

ti.init(arch=ti.metal)


def load_data(file_path):
  with open(file_path) as f:
    data = f.readlines()
  return [line.strip() for line in data]


def star1(data: list):
  n, m = len(data), len(data[0])
  data = ['.' + line + '.' for line in data]
  data.insert(0, ['.'] * (m+2))
  data.append(['.'] * (m+2))
  ans = 0
  for r in range(1, n+1):
    for c in range(1, m+1):
      if data[r][c] != '@':
        continue
      d = [1, -1, 0]
      ans += sum(data[r+x][c+y] == '@' for x in d for y in d) < 5
  return ans


@ti.kernel
def helper_kernel(data: ti.types.ndarray(ndim=2)):
  N, M = data.shape
  for r, c in ti.ndrange((1, N - 1), (1, M - 1)):
    if data[r, c] == 0:
      continue

    neighbor_sum = 0
    for i in range(-1, 2):
      for j in range(-1, 2):
        neighbor_sum += data[r + i, c + j]

    if neighbor_sum < 5:
      data[r, c] = 0
    else:
      data[r, c] = 1


def star2(raw_input: list):
  rows = len(raw_input)
  cols = len(raw_input[0])

  host_data = np.zeros((rows + 2, cols + 2), dtype=np.int32)

  for r in range(rows):
    for c in range(cols):
      if raw_input[r][c] == '@':
        host_data[r + 1, c + 1] = 1

  gpu_data = ti.ndarray(dtype=ti.i32, shape=(rows + 2, cols + 2))
  gpu_data.from_numpy(host_data)

  for _ in range(1000):
    helper_kernel(gpu_data)

  result_data = gpu_data.to_numpy()

  ans = 0
  for r in range(1, rows + 1):
    for c in range(1, cols + 1):
      if host_data[r][c] != result_data[r, c]:
        ans += result_data[r-1:r+2, c-1:c+2].sum() < 5
  return ans


def main():
  datas = {}
  for file in Path(__file__).resolve().parent.iterdir():
    if file.is_file() and file.suffix == '.in':
      datas[file.name] = (load_data(file.absolute()))

  print(f"""star 1:
        example: {star1(datas['example.in'])}
        answer: {star1(datas['input.in'])}
        """)

  print(f"""star 2:
        example: {star2(datas['example.in'])}
        answer: {star2(datas['input.in'])}
        """
        )


if __name__ == "__main__":
  main()
