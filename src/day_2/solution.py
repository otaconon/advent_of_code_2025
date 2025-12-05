import os
from typing import Callable
import re

script_dir = os.path.dirname(os.path.abspath(__file__))
file_path = os.path.join(script_dir, 'input.in')

def load_data():
  with open(file_path) as f:
    data = f.read()
  return [tuple(map(int, range.split("-"))) for range in data.split(",")]

def bruteforce(rule: Callable[[str, int, str], bool]):
  ranges = load_data()
  ans = 0
  for start, end in ranges:
    for i in range(start, end+1):
      s = str(i); n = len(s)
      for j in range(1, n//2+1):
        if n % j != 0: continue
        if rule(s[:j], n // j, s):
          ans += i
          break
  return ans

def regex(expr: str):
  ranges = load_data()
  ans = 0
  for start, end in ranges:
    ans += sum(i if bool(re.match(expr, str(i))) else 0 for i in range(start, end+1))
  return ans

def kmp(rule: Callable[[int, int], bool]):
  ranges = load_data()
  ans = 0
  for start, end in ranges:
    for i in range(start, end+1):
      s = str(i); n = len(s)
      pi = [0] * n
      for j in range(1, n):
        k = pi[j-1]
        while k > 0 and s[j] != s[k]: k = pi[k-1]
        if s[j] == s[k]: k += 1
        pi[j] = k
      l = pi[n-1]
      ans += i if l > 0 and rule(n, l) else 0
  return ans

def main():
  print(f"""star 1:
        bruteforce: {bruteforce(lambda sub, rep, s: sub * 2 == s)}
        regex:      {regex(r'^(.+)\1$')}
        kmp:        {kmp(lambda n, l: n % (n-l) == 0 and (n/(n-l)) % 2 == 0)}
        """)
  print(f"""star 2:
        bruteforce: {bruteforce(lambda sub, rep, s: sub * rep == s)}
        regex:      {regex(r'^(.+)\1+$')}
        kmp:        {kmp(lambda n, l: n % (n-l) == 0)}
        """)

if __name__ == "__main__":
  main()
