#include <stdlib.h>
#include <string.h>

extern "C"
char * hello_world(char* var1, char* var2){
    int newSize = strlen(var1)  + strlen(var2) + 1;
    char * newChar = (char *)malloc(newSize);
    strcpy(newChar, var1);
    strcat(newChar, " ");
    strcat(newChar, var2);
    return newChar;
}