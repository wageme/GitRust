#include <stdio.h>

extern void rust_function();

int main() {
    printf("Hello");
    rust_function();
}