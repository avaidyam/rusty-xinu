/*  main.c  - main */

#include <xinu.h>

extern int double_input(int input);
extern int quadruple_input(int input);

process	main(void)
{
	recvclr();
	while (TRUE) {
		kprintf("main()\n");
		kprintf("Double 5: %d\n", double_input(5));
		kprintf("Quadruple 10: %d\n", quadruple_input(10));
		sleepms(500);
	}
	return OK;
    
}
