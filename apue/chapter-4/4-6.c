#include <stdio.h>
#include <unistd.h>
#include <fcntl.h>
#include <sys/mman.h>

char buf1[] = "abcdefghij";
char buf2[] = "ABCDEFGHIJ";

int
main(void)
{
  int fd;
  if ((fd = creat("file.hole", 777)) < 0)
    fprintf(stderr, "Creat Error");

  if (write(fd, buf1, 10) != 10)
    fprintf(stderr, "write error");

  if (lseek(fd, 16384, SEEK_SET) == -1)
    fprintf(stderr, "lseek error");

  if (write(fd, buf2, 10) != 10)
    fprintf(stderr, "write error");
}
