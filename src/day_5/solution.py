from pathlib import Path
from typing import Callable
import taichi as ti
import numpy as np
import re

ti.init(arch=ti.metal)

def load_data(file_path):
  with open(file_path) as f:
    data = f.read()
  data = data.split("\n\n")
  fresh_ranges = [line.strip() for line in data[0].split('\n')]
  fresh_ranges = [list(map(int, line.split('-'))) for line in fresh_ranges]
  ids = [int(line.strip()) for line in data[1].split('\n')]
  return fresh_ranges, ids


def star1(data: list):
  fresh_ranges = data[0]
  ids = data[1]
  ans = 0
  for id in ids:
    for range in fresh_ranges:
      if range[0] <= id <= range[1]:
        ans += 1
        break 
  return ans

def star2(data: list[list]):
  ranges = data[0]
  ranges.sort(key=lambda x: x[1])
  new_ranges = [ranges[0]]
  for i, range in enumerate(ranges[1:]):
    if range[0] > new_ranges[-1][0]:
      new_ranges.append(range)
    
    while len(new_ranges) and range[0] <= new_ranges[-1][0]:
      new_ranges.pop()
    if new_ranges and range[0] <= new_ranges[-1][1]:
      x = [new_ranges[-1][0], range[1]]
      new_ranges.pop()
      new_ranges.append(x)
    else:
      new_ranges.append(range)
  
  return sum(range[1] - range[0] + 1 for range in new_ranges) 

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