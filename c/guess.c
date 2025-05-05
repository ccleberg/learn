#include <time.h>
#include <stdlib.h>
#include <stdio.h>

// Create separate function to get a random number between 0 - 100
int get_rand() {
	srand(time(NULL));
	int r = rand() % 100;
	return r;
}

int main() {
    // Call function to get a random number between 0 - 100
    int random_number = get_rand();
    printf("Random number: %d", random_number);

    // Set a qualifier to check if the user has guessed correctly
    int user_success = 0;

    // Create a loop to allow the user to guess as many times as they want
    while (user_success == 0) {
        // Read user input
        char line[256];
        if(fgets(line, sizeof line, stdin) != NULL)
        {
            // Convert user input to an integer
            int user_input = atoi(line);

            // Do the main comparison to determine if the guess is correct
            if (user_input == random_number) {
                user_success = 1;
                printf("Congratulations! You solved the puzzle!");
            } else {
                printf("Sorry, that's not correct! Please try again:\n");
            }
        }
    }

    return 0;
}
