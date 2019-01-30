#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

int
main(void)
{
  printf("PID %d\n", (long)getpid());
  exit(0);
}
