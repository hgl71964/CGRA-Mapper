// Source is: /home/alex/.local/share/compy-Learn/1.0/DarkNet/content/./src/image.c

#include <stdint.h>
#include <stdio.h>




typedef uint16_t stbi__uint16;

int
fn (unsigned int x, stbi__uint16 * src, stbi__uint16 * dest, int i)
{
  for (i = x - 1; i >= 0; --i, src += 2, dest += 1)
    {
      dest[0] = src[0];
    }
}
