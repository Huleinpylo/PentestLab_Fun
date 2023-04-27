#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void vulnerable_function(char *input) {
    char buffer[50];
    strcpy(buffer, input);
}

void secret() {
    char secret_buffer[100];
    FILE *fp = fopen("secret.txt", "r");
    fgets(secret_buffer, 100, fp);
    fclose(fp);
    printf("Secret: %s", secret_buffer);
}

int main(int argc, char *argv[]) {
    vulnerable_function(argv[1]);
    return 0;
}
