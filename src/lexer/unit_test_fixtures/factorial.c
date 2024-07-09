#include <stdio.h>
#include <stdlib.h>

#define MAX_SIZE 100

// Function to calculate factorial
int factorial(int n) {
    if (n == 0 || n == 1) {
        return 1;
    }
    return n * factorial(n - 1);
}

int main() {
    int numbers[MAX_SIZE];
    int size = 0;
    char input[10];

    printf("Enter up to %d numbers (enter 'q' to quit):\n", MAX_SIZE);

    while (size < MAX_SIZE) {
        printf("Number %d: ", size + 1);
        scanf("%s", input);

        if (input[0] == 'q' || input[0] == 'Q') {
            break;
        }

        numbers[size] = atoi(input);
        size++;
    }

    printf("\nFactorials of the entered numbers:\n");
    for (int i = 0; i < size; i++) {
        printf("%d! = %d\n", numbers[i], factorial(numbers[i]));
    }

    return 0;
}
