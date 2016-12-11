/*  main.c  - main */

#include <xinu.h>

extern int double_input(int input);

process	main(void)
{
	recvclr();
	while (TRUE) {
		kprintf("main()\n");
		sleepms(2000);
		int x = double_input(5);
		kprintf("Double: %d", x);
	}
	return OK;
    
}
