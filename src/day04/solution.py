from pathlib import Path
from typing import Callable
import taichi as ti
import numpy as np
import time
import re

ti.init(arch=ti.metal)
res_x, res_y = 1024, 1024
MAX_SIZE = (1024, 1024)
pixels = ti.field(dtype=ti.float32, shape=(res_x, res_y))
grid = ti.field(dtype=ti.i32, shape=MAX_SIZE)
next_grid = ti.field(dtype=ti.i32, shape=MAX_SIZE)
initial_snapshot = ti.field(dtype=ti.i32, shape=MAX_SIZE)
gui = ti.GUI("vis", res=(res_x, res_y))  # type: ignore


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
def upscale_nearest(rows: int, cols: int):
  for i, j in pixels:
    src_i = int(i * (rows / res_x))
    src_j = int(j * (cols / res_y))

    src_i = ti.min(src_i, rows - 1)
    src_j = ti.min(src_j, cols - 1)

    pixels[i, j] = grid[src_i, src_j]


@ti.kernel
def init_grid(input_arr: ti.types.ndarray(), rows: int, cols: int):
  for i, j in ti.ndrange(rows + 2, cols + 2):
    val = input_arr[i, j]
    grid[i, j] = val
    next_grid[i, j] = val
    initial_snapshot[i, j] = val


@ti.kernel
def step_kernel(rows: int, cols: int):
  for r, c in ti.ndrange((1, rows + 1), (1, cols + 1)):
    if grid[r, c] == 0:
      next_grid[r, c] = 0
      continue

    neighbor_sum = 0
    for i in range(-1, 2):
      for j in range(-1, 2):
        neighbor_sum += grid[r + i, c + j]

    if neighbor_sum < 5:
      next_grid[r, c] = 0
    else:
      next_grid[r, c] = 1


@ti.kernel
def copy_back(rows: int, cols: int):
  for I in ti.grouped(ti.ndrange((1, rows + 1), (1, cols + 1))):
    grid[I] = next_grid[I]


@ti.kernel
def compute_ans(rows: int, cols: int) -> ti.i32:
  ans = 0
  for r, c in ti.ndrange((1, rows + 1), (1, cols + 1)):
    if initial_snapshot[r, c] != grid[r, c]:
      neighbor_sum = 0
      for i in range(-1, 2):
        for j in range(-1, 2):
          neighbor_sum += grid[r + i, c + j]

      if neighbor_sum < 5:
        ans += 1
  return ans


def star2(raw_input: list):
  rows = len(raw_input)
  cols = len(raw_input[0])

  host_data = np.zeros((rows + 2, cols + 2), dtype=np.int32)
  for r in range(rows):
    for c in range(cols):
      if raw_input[r][c] == '@':
        host_data[r + 1, c + 1] = 1

  init_grid(host_data, rows, cols)

  step_count = 0
  last_update_time = time.time()

  while step_count < 1000 and gui.running:
    current_time = time.time()
    if current_time - last_update_time >= 0.1:
      step_kernel(rows, cols)
      copy_back(rows, cols)
      upscale_nearest(rows, cols)

      last_update_time = current_time
      step_count += 1

    gui.set_image(pixels)
    gui.show()
    time.sleep(0.01)

  while gui.running:
    gui.set_image(pixels)
    gui.show()
    time.sleep(0.1)

  return compute_ans(rows, cols)


def main():
  datas = {}
  for file in Path("input/day04/").resolve().iterdir():
    if file.is_file() and file.suffix == '.in':
      datas[file.name] = (load_data(file.absolute()))

  print(f"""star 1:
        example: {star1(datas['example.in'])}
        answer: {star1(datas['input.in'])}
        """)

  print(f"""star 2:
        answer: {star2(datas['input.in'])}
        """
        )


if __name__ == "__main__":
  main()
