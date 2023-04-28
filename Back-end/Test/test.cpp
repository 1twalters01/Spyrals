#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <math.h>

extern "C"
void test1() {
    printf("test \n");
}

extern "C"
void test2(char* var1, int var2) {
    printf("This is a %s and it is number %d \n", var1, var2);
}

extern "C"
const char * test3() {
    char const *test = "test 3";
    return test;
}

extern "C"
char * test4(char * var1, char* var2) {
    int newSize = strlen(var1)  + strlen(var2) + 1;
    char * newChar = (char *)malloc(newSize);
    strcpy(newChar, var1);
    strcat(newChar, " ");
    strcat(newChar, var2);
    return newChar;
}

extern "C"
char * test5(char* var1, int var2) {
    char * newvar2 = (char *)malloc(round(var2/10 + 1));
    sprintf(newvar2, "%d", var2);
    int newSize = strlen(var1)  + strlen(newvar2) + 1;
    char *newChar = (char *)malloc(newSize);
    strcpy(newChar, var1);
    strcat(newChar, " ");
    strcat(newChar, newvar2);
    return newChar;
}

