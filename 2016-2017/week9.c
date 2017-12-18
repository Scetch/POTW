#include <stdio.h>

#define MAX_PROGRAM_SIZE 4096
#define MAX_DATA_SIZE 2048

char prog[MAX_PROGRAM_SIZE];
char* insn_ptr = prog;

char data[MAX_DATA_SIZE];
char* data_ptr = data;

void eval() {
	// Keep track of the jump in case we encounter a loop
	// and must jump back.
	char* start_insn = insn_ptr;
	char c;

	while(c = *(insn_ptr++)) {
		if(c == '>')	  data_ptr++;
		else if(c == '<') data_ptr--;
		else if(c == '+') (*data_ptr)++;
		else if(c == '-') (*data_ptr)--;
		else if(c == '.') putchar(*data_ptr);
		else if(c == ',') (*data_ptr) = getchar();
		else if(c == '[') {
			if((*data_ptr) == 0) {
				int nest_level = 0;

				while((c = *(insn_ptr++)) != ']' || nest_level != 0) {
					if(c == '[') nest_level++;
					else if(c == ']') nest_level--;
				}
			} else {
				// Recursively eval the inner loop.
				eval();
			}
		} 
		else if(c == ']') {
			if((*data_ptr) != 0) {
				// Jump back to the start of this loop.
				insn_ptr = start_insn;
			} else {
				// Break out of this inner loop and return.
				break;
			}
		}
	}
}

int main() {
	scanf("%s", prog);
	eval();
	return 0;
}