from pathlib import Path
from typing import Callable
import re

def load_data(file_path):
  with open(file_path) as f:
    data = f.readlines()
  return [list(map(int, list(line.strip()))) for line in data]

def star2(data):
  ans = 0
  for line in data:
    st = []
    for i, x in enumerate(line):
      if i < 12:
        st.append(x) 
        continue
      
      st.append(x)
      for j in range(len(st)-1):
        if st[j] < st[j+1]:
          st[j:] = st[j+1:]
          break
      if len(st) > 12:
        st.pop()
    
    ans += int("".join(list(map(str, st))))
  return ans
      

def brute_force(data):
  ans = 0
  for line in data:
    cnt = 0
    for i, x in enumerate(line[:-1]):
      cnt = max(cnt, 10*x + max(line[i+1:]))
    ans += cnt
  return ans

def main():
  datas = {}
  for file in Path(__file__).resolve().parent.iterdir():
    if file.is_file() and file.suffix == '.in':
      datas[file.name] = (load_data(file.absolute()))
  
   
  print(f"""star 1:
        example: {brute_force(datas['example.in'])}
        answer: {brute_force(datas['input.in'])}
        """)
  
  print(f"""star 2:
        example: {star2(datas['example.in'])}
        answer: {star2(datas['input.in'])}
        """
        )

if __name__ == "__main__":
  main()

