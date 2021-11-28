// gcc -dynamiclib -fPIC -o foo.o -c foo.c
// gcc -shared -fPIC -o foo.so foo.o

#include <stdio.h>
#include <stdlib.h>

struct foo
{
  int a;
  int b;
};

int foo_create(struct foo** result);
void set_a(struct foo *f, int param);
int get_a(struct foo* f);
void foo_destroy(struct foo* f);

int foo_create(struct foo** result) {
  struct foo* f = malloc(sizeof(struct foo));
  if (f == NULL) {
    return -1;
  }
  f->a = 7; 
  f->b = 13;
  *result = f;
  printf("foo_create in c\n");
  return 0;
}

void set_a(struct foo* f, int param) {
  f->a = param;
  printf("set_a in c\n");
}
int get_a(struct foo* f) {
  printf("get_a in c\n");
  return f->a;
}

void foo_destroy(struct foo* f) {
  free(f);
  printf("foo_destroy in c\n");
}

