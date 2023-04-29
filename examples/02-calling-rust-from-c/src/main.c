#include <nusys.h>
#include <ultra64.h>
#include "main.h"

u64	bootStack[STACKSIZE/sizeof(u64)];

int call_rust();

void mainproc()
{
    int i;
        
    i = call_rust();		
    osInitialize();
    
    while (1) {
        ;;
    }
}
