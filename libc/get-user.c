#include <stdio.h>
#include <unistd.h>
#include <sys/types.h>
#include <pwd.h>

char *get_username(uid_t user_id);

/*
Error Handling:

getuid: Always succeeds
getpwuid: This can fail.
*/

int main() {
	printf("Welcome, %s!\n", get_username(getuid()));
}

/*
TODO: Check return null and errno for better error handling.
*/
char *get_username(uid_t user_id) {
    struct passwd *user_entry = getpwuid(user_id);
    return user_entry->pw_name;
}