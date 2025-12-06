import os
import torch
import triton
import triton.testing
import triton.language as tl
import numpy as np
import pytest

device = 'cuda'

script_dir = os.path.dirname(os.path.abspath(__file__))
file_path = os.path.join(script_dir, 'input.in')

@triton.jit
def brute_kernel(starts_ptr, steps_ptr, output_ptr, n_elements, BLOCK_SIZE: tl.constexpr):
    pid = tl.program_id(axis=0)
    block_start = pid * BLOCK_SIZE
    offsets = block_start + tl.arange(0, BLOCK_SIZE)
    mask = offsets < n_elements
    
    starts = tl.load(starts_ptr + offsets, mask=mask, other=0.0)
    steps = tl.load(steps_ptr + offsets, mask=mask)
    sgns = tl.where(steps >= 0, 1, -1)
    steps = tl.abs(steps)
    max_loops = tl.max(steps, axis=0)
    res = tl.zeros([BLOCK_SIZE], dtype=tl.int32)
    
    for i in range(max_loops):
      active_lanes = mask & (i < steps)
      starts = tl.where(active_lanes, starts+sgns, starts)
      is_mod_100 = (starts % 100 == 0)
      increment = (active_lanes & is_mod_100).to(tl.int32)
      res += increment
    
    block_total = tl.sum(res, axis=0) 
    tl.atomic_add(output_ptr, block_total)


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

def gpu_brute_2():
  data = load_data()
  n_elements = len(data)
  steps = np.array([steps if rot == 'R' else -steps for rot, steps in data], dtype=np.int32)
  cumsum = np.cumsum(steps).astype(np.int32)
  starts = np.zeros_like(cumsum)
  starts[1:] = cumsum[:-1]
  starts[0] = 0
  starts += 50
  
  steps_tensor = torch.from_numpy(steps).to(device)
  starts_tensor = torch.from_numpy(starts).to(device)
  output_tensor = torch.zeros(1, device=device, dtype=torch.int32)
  grid = lambda meta: (triton.cdiv(n_elements, meta['BLOCK_SIZE']), )
  brute_kernel[grid](
    starts_tensor,
    steps_tensor,
    output_tensor,
    n_elements,
    BLOCK_SIZE=1024 # type: ignore
  ) 
  
  return output_tensor.item() 


def part_2():
  data = load_data()
  current_pos = 50
  answer = 0

  for direction, rotation in data:
    if direction == 'R':
      next_pos = current_pos + rotation
      answer += (next_pos // 100) - (current_pos // 100)
      current_pos = next_pos
    else:
      next_pos = current_pos - rotation
      answer += (current_pos - 1) // 100 - (next_pos - 1) // 100
      current_pos = next_pos

  return answer

def benchmark_comparison():
  import time
  start = time.perf_counter()
  cpu_result = brute_2()
  end = time.perf_counter()
  cpu_ms = (end - start) * 1000
  print(f"CPU Time brute force: {cpu_ms:.3f} ms")
  
  start = time.perf_counter()
  cpu_result = part_2()
  end = time.perf_counter()
  cpu_ms = (end - start) * 1000
  print(f"CPU Time algorithmic: {cpu_ms:.3f} ms")

  gpu_ms = triton.testing.do_bench(gpu_brute_2, warmup=25, rep=100)
  print(f"GPU Time brute force: {gpu_ms:.3f} ms")

  speedup = cpu_ms / gpu_ms # type: ignore
  print(f"Speedup: {speedup:.1f}x")
  

def main():
  print(f"star 1: {part_1()}")
  print(f"star 2:\n\tbrute force on cpu: {brute_2()}")
  print(f"\tbrute force on gpu: {gpu_brute_2()}")
  print(f"\talgorithmic on cpu: {part_2()}")

  benchmark_comparison()

if __name__ == "__main__":
  main()
