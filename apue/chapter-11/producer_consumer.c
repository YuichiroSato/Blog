#include <stdlib.h>
#include <pthread.h>
#include <stdio.h>
#include <sys/types.h>

struct Buffer {
  int buf[32];
  int head;
  int tail;
  int start;
  int end;
};

static struct Buffer buffer;

static pthread_mutex_t f_lock;

static pthread_cond_t writable = PTHREAD_COND_INITIALIZER;

static pthread_cond_t readable = PTHREAD_COND_INITIALIZER;

void *
consumer(void *arg)
{
  pthread_t thr_id;
  thr_id = pthread_self();

  printf("%p: consumer thread start\n", thr_id);

  while(buffer.end == 0 || (buffer.tail + 1) % 32 != buffer.head) {
    pthread_mutex_lock(&f_lock);
    if ((buffer.tail + 1) % 32 == buffer.head) {
      printf("%p: consumer waiting\n", thr_id);
      pthread_cond_wait(&readable, &f_lock);
    }
    buffer.tail++;
    buffer.tail %= 32;
    int i = buffer.buf[buffer.tail];
    pthread_mutex_unlock(&f_lock);
    pthread_cond_signal(&writable);
  
    if (buffer.start == 0) {
      continue;
    }
    
    if (i % 3 == 0 && i % 5 == 0) {
      printf("%p: %d,fizzbuzz\n", thr_id, i);
    } else if (i % 3 == 0) {
      printf("%p: %d,fizz\n", thr_id, i);
    }
    else if (i % 5 == 0) {
      printf("%p: %d,buzz\n", thr_id, i);
    }
    sleep(1);
  }

  printf("%p: consumer thread stoped\n", thr_id);

  return ((void *) 0);
}

void *
producer(void *arg)
{
  int i;
  i = 0;
  buffer.start = 1;
  
  printf("producer thread start\n");
  while(i < 100) {
    pthread_mutex_lock(&f_lock);
    if ((buffer.head + 1) % 32 == buffer.tail) {
      printf("producer waiting\n");
      pthread_cond_wait(&writable, &f_lock);
    }
    buffer.buf[buffer.head] = i;
    buffer.head++;
    buffer.head %= 32;
    pthread_mutex_unlock(&f_lock);
    pthread_cond_signal(&readable);
    i++;
    
    // make consumer wait
    if (i == 50) {
      sleep(4);
    }
  }
  
  pthread_cond_signal(&readable);
  buffer.end = 1;
  printf("producer thread stoped\n"); 
 
  return ((void *) 0);
}

int
main(void)
{
  int err;
  pthread_t pro_thr;
  pthread_t con_thr[10];
  
  buffer.head = 1;
  buffer.tail = 0;
  buffer.start = 0;
  buffer.end = 0;
  
  pthread_mutex_init(&f_lock, NULL);

  err = pthread_create(&pro_thr, NULL, producer, NULL);
  
  sleep(1);
  
  int i;
  for (i = 0; i < 10; i++) {
    err = pthread_create(&con_thr[i], NULL, consumer, NULL);
  }
  
  void * code;
//  err = pthread_join(pro_thr, &code);
  for (i = 0; i < 10; i++) {
    err = pthread_join(con_thr[i], &code);
  }

  sleep(1);
  fflush(stdout);
  exit(0);
}
