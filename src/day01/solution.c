#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int floor_div(int a, int b) {
  return a / b - (a % b != 0 && (a ^ b) < 0);
}

char* read_file(const char* filename) {
  FILE* file = fopen(filename, "rb");
  if (file == NULL) {
    perror("Error opening file");
    return NULL;
  }

  fseek(file, 0, SEEK_END);
  long length = ftell(file);
  fseek(file, 0, SEEK_SET);

  char* buffer = malloc(length + 1);

  size_t read_size = fread(buffer, 1, length, file);
  buffer[length] = '\0';

  fclose(file);
  return buffer;
}

int solve_1(char* data) {
  int total_rotation = 50;
  int answer = 0;
  char* line = strtok(data, "\r\n");
  while (line != NULL) {
    int dir = line[0] == 'R' ? 1 : -1;
    int rot = atoi(&line[1]) * dir;
    total_rotation += rot;
    total_rotation %= 100;
    answer += total_rotation == 0;
    line = strtok(NULL, "\r\n");
  }

  return answer;
}

int solve_2(char* data) {
  int pos = 50;
  int answer = 0;

  char* line = strtok(data, "\r\n");
  while (line != NULL) {
    int rot = atoi(&line[1]);
    int next_pos = pos + (line[0] == 'R' ? rot : -rot);

    if (line[0] == 'R') 
      answer += floor_div(next_pos, 100) - floor_div(pos, 100);
    else
      answer += floor_div(pos - 1, 100) - floor_div(next_pos - 1, 100);

    pos = next_pos;
    line = strtok(NULL, "\r\n");
  }

  return answer;
}

int main() {
  char* data = read_file("input.in");
  char* data_copy = strdup(data);

  printf("star 1:\t%d\n", solve_1(data));
  printf("star 2:\t%d\n", solve_2(data_copy));

  free(data);
  free(data_copy);
}