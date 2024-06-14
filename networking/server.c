#include<stdio.h>
#include<stdlib.h>
#include<unistd.h>
#include<errno.h>
#include<string.h>
#include<sys/types.h>
#include<sys/socket.h>
#include<netinet//in.h>
#include<netdb.h>
#include<arpa/inet.h>
#include<sys/wait.h>
#include<signal.h>

#define PORT "3490"

#define BACKLOG 10

#define ERROR1 -1

#define ERROR2 -2

#define SUCCESS 0


// Returns 0 if successful
int getServerInfo(char *port, struct addrinfo *server_info) {
    // No heap allocation, only on stack.
    struct addrinfo hints;
    int rv;

    memset(&hints, 0, sizeof hints);
    hints.ai_family = AF_UNSPEC;
    hints.ai_socktype = SOCK_STREAM;
    hints.ai_flags = AI_PASSIVE; // use my IP

    if ((rv = getaddrinfo(NULL, PORT, &hints, &server_info)) != SUCCESS) {
        fprintf(stderr, "getaddrinfo: %s\n", gai_strerror(rv));
        return ERROR1;
    }

    return SUCCESS;
}

// Returns 0 if successful with server_info and listen_fd appropriately set.
int bindServer(struct addrinfo *server_info, int *listen_fd) {
    struct addrinfo *p;
    int yes=1;

    for (p = server_info; p != NULL; p = p->ai_next) {
        // First get a socket fd
        if ((*listen_fd = socket(p->ai_family, p->ai_socktype, p->ai_protocol)) == -1) {
            perror("server: Error getting socket to listen");
            // Try next AddrInfo
            continue;
        }

        // Enable Reuse, otherwise based on TIME_WAIT the TCP socket might not be closed
        if (setsockopt(*listen_fd, SOL_SOCKET, SO_REUSEADDR, &yes, sizeof(int)) == -1) {
            perror("setsockopt failed");
            // Cannot recover
            return ERROR1;
        }

        if (bind(*listen_fd, p->ai_addr, p->ai_addrlen) == -1) {
            close(*listen_fd);
            perror("server: bind");
            // Try next AddrInfo
            continue;
        }

        // Break and return on successful bind
        return SUCCESS;
    }

    // Couldnt bind to any of the addrinfo
    return ERROR2;
}

void sigchld_handler(int s)
{
    // waitpid() might overwrite errno, so we save and restore it:
    int saved_errno = errno;

    while(waitpid(-1, NULL, WNOHANG) > 0);

    errno = saved_errno;
}

int main(void) {
    int listen_fd, conn_fd;
    struct addrinfo *server_info;
    struct sockaddr_storage peer_addr;
    socklen_t sin_size;
    struct sigaction sa;
    char s[INET6_ADDRSTRLEN];

    if (getServerInfo(PORT, server_info) != SUCCESS) {
        return 1;
    }

    if (bindServer(server_info, &listen_fd) != 0) {
        perror("Error Binding Server ");
        return 2;
    }

    freeaddrinfo(server_info);

    // Start Listening
    if (listen(listen_fd, BACKLOG) == -1) {
        perror("Error listening");
        return 3;
    }

    sa.sa_handler = sigchld_handler; // reap all dead processes
    sigemptyset(&sa.sa_mask);
    sa.sa_flags = SA_RESTART;
    if (sigaction(SIGCHLD, &sa, NULL) == -1) {
        perror("sigaction");
        exit(1);
    }

    printf("server: waiting for connections...\n");

    // Accept a connection and handle it in forked process
    while(1) {

    }


    return 0;
} 