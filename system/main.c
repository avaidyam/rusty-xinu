/*  main.c  - main */

#include <xinu.h>

process main(void) {
    recvclr();
    while (1) {
        kprintf("main()\n");
        kprintf("getpid: %d\n", getpid());
        sleepms(500);
    }
    return OK;
}
