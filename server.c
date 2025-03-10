#include <stdio.h>
#include <fcntl.h>
#include <string.h>
#include <assert.h>
#include <stdlib.h>

#define LINE_SIZE 4096


const char* FILEPATH = "./db.txt";

int main(int argc, char* argv[]) {
    assert(argc == 3 || argc == 4);
    char* key = argv[2];
    FILE* db_file = fopen(FILEPATH, "r+");
    if (db_file == NULL) {

        perror("Error opening file");
        return 1;
    }
    size_t len = 0;
    char* line = NULL;

    if (strcmp(argv[1], "get") == 0 || strcmp(argv[1], "GET") == 0) {
        while (1) {
            ssize_t bytes_read = getline(&line, &len, db_file);
            if (bytes_read == -1) {
                if (feof(db_file)) {
                    printf("End of file reached. Cannot find key.\n");
                } else {
                    perror("Error reading line");
                }
                return 1;
            } else {
                char* line_copy = strdup(line);
                char* key_in_line = strtok(line_copy, " ");
                if (key_in_line != NULL && strcmp(key_in_line, key) == 0) {
                    char* value = strtok(NULL, "\n");
                    if (value != NULL) {
                        printf("value: %s\n", value);
                        free(line_copy);
                        break;
                    }
                }
            }
        }
        free(line);
    } else if (strcmp(argv[1], "set") == 0 || strcmp(argv[1], "SET") == 0) {
        char* new_value = argv[3];
        while (1) {
            ssize_t bytes_read = getline(&line, &len, db_file);
            if (bytes_read == -1) {
                if (feof(db_file)) {
                    char buffer[LINE_SIZE];
                    snprintf(buffer, LINE_SIZE, "%s %s\n", key, new_value);
                    fprintf(db_file, "%s", buffer);
                    break;
                }
            } else { // we need to overwrite the value
                char* key_in_line = strtok(line, " ");
                if (key_in_line != NULL && strcmp(key_in_line, key) == 0) {
                    char* value = strtok(NULL, "\n");
                    if (value != NULL) {
                        // how do we overwrite the line from after the whitespace?
                        strcpy(value, new_value);
                    }
                }
            }
        }
    }
}