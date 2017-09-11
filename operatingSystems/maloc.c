#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#define HEAPSIZE 320000

struct page
{
  unsigned int *p_size;
  char *p_start;
};

char *heap;
struct page *ledger;

void init();
int search(int spot, int size);
void mov(int* point);
char* maloc(int size);
void fre(int *spot);

int main()
{
  init();
  int action;
  int size;
  printf("usage for maloc: 1 size\nusage for fre: 2 locationInledger\nfre should be given odd numbers\n"); 
  scanf("%d %d", &action, &size);

  while(1)
  {
    if (action == 1)
    {
      printf("Here is your pointer: %p\n", maloc (size));
    }
    else
    {
      fre(ledger + size);
      printf("freed.\n");
    }

    for (int i = 0; i < 16; i++)
      {
        printf("%d\n", ledger[i]);
      }
    printf("\n\n");
    scanf("%d %d", &action, &size);
    printf("\n");
  }
}

void init()
{
  heap = malloc(HEAPSIZE);
  ledger = malloc(3200);
  *(ledger->p_size) = HEAPSIZE;
  ledger->p_start = heap;
  *((ledger+1)->p_size) = 0;
}

// return pointer to space, or -1 on failure.
char* maloc (int size)
{
  char *a;
  struct page *bMark;

  for (bMark = ledger; bMark->p_size; bMark++)
  {
    if (*(bMark->p_size) > size)
    {
      a = bMark->p_start;
      bMark->p_start += size;
      if ( (*(bMark->p_size) -= size) == 0 )
      {
        do 
        {
          bMark++;
          (bMark-1)->p_start = bMark->p_start;
        } while ((bMark-1)->p_size = bMark->p_size);
      }
    }
  }

  //search for open space
  int s =  (search(1, size));
  if (s == -1) return (int*) -1;
  return (heap + s);
}

//-1 means end. -2 means open space.
int search (int spot, int size)
{
  if((spot+size) > HEAPSIZE) return -1;
 
  //if -1...
  if (ledger[spot] == -1)
  {
    ledger[spot] = 1 + ledger[spot-1]; //make a note of where the block starts
    ledger[spot+1] = ledger[spot] + size - 1; //make a note of where the block ends
    ledger[spot+2] = -1;
    return ledger[spot]; //return the start of the memory
  }

  //if -2...
  if (ledger[spot] == -2)
  {
    //Find the next filled chunk of memory.
    int i = 2;
    while(ledger[spot+i] == -2)
    {
      i += 2;
    }

    //If there is enough space between these blocks for the new chunk of memory...
    if ((ledger[spot+i] - ledger[spot-1]) > size)
    {
      ledger[spot] = 1 + ledger[spot-1]; //make a note of where the block starts
      ledger[spot+1] = ledger[spot] + size - 1; //make a note of where the block ends
      return spot;
    }
    else return search(spot+i, size); //else resume search at the next filled chunk of memory.
  }

//If there is a space between this one and the next.
  if ((ledger[spot+2] - ledger[spot+1]) > size)
  {
    mov(ledger + spot + 2);
    ledger[spot+2] = 1 + ledger[spot+1]; //make a note of where the block starts
    ledger[spot+3] = ledger[spot+2] + size - 1; //make a note of where the block ends
    return ledger[spot+2];
  }

  return search(spot+2, size);
}

void mov(int* point)
{
  int c,k,l;
  for (int i = 0; i < 16 && *point != -1; i += 2)
  {
    l = *(point+i);
    *(point+i) = c;
    c = l;
    l = *(point+i+1);
    *(point+i+1) = k;
    k = l;
  }
}

void fre(int *spot)
{
  *spot = -2;
}
