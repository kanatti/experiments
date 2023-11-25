#include<stdio.h>
#include<string.h>
#include<sys/types.h>
#include<sys/socket.h>
#include<netdb.h>
#include<arpa/inet.h>
#include<netinet/in.h>

/* -- USAGE --
❯ ./showip google.com
IP Addresses for google.com:

  IPV6: 2607:f8b0:400b:807::200e
  IPV4: 142.251.41.46
❯ ./showip google.com  --ipv4
IP Addresses for google.com:

  IPV4: 142.251.41.46
❯ ./showip google.com  --ipv6
IP Addresses for google.com:

  IPV6: 2607:f8b0:400b:807::200e
*/


void printUsage() {
    fprintf(stderr, "usage: \nshowip hostname\nshowip hostname --ipv4\nshowip hostname --ipv6\n");
}


// Prints IPV4 and IPV6 Addresses
int main(int argc, char *argv[]) {
    // Uninitialized, these can garbage values.
    struct addrinfo hints, *res, *p;
    int status;
    char ipstr[INET6_ADDRSTRLEN];

    if (argc < 2 || argc > 3) {
        printUsage();
        return 1;
    }

    char *hostname = argv[1];
    int ai_family = AF_UNSPEC;

    if (argc == 3) {
        if (strcmp(argv[2], "--ipv4") == 0) {
            ai_family = AF_INET;
        } else if (strcmp(argv[2], "--ipv6") == 0) {
            ai_family = AF_INET6;
        } else {
            printUsage();
            return 1;
        }
    }

    // First I thought memset involves some allocation.
    // But thats not the case. hints can have garbage value in stack.
    // Applies to both basic types and pointers (Garbage pointer values).
    // Memset sets value of basic stack types to zero and pointers to null.
    // No allocation, no interaction with heap.
    memset(&hints, 0, sizeof hints);
    hints.ai_family = ai_family;
    hints.ai_socktype = SOCK_STREAM;

    if ((status = getaddrinfo(hostname, NULL, &hints, &res)) != 0) {
        fprintf(stderr, "getaddrinfo: %s\n", gai_strerror(status));
        return 2;
    }

    printf("IP Addresses for %s:\n\n", hostname);

    for (p = res; p != NULL; p = p->ai_next) {
        void *addr;
        char *ipver;

        if (p->ai_family == AF_INET) {
            struct sockaddr_in *ipv4 = (struct sockaddr_in *)p->ai_addr;
            addr = &(ipv4->sin_addr);
            ipver = "IPV4";
        } else {
            struct sockaddr_in6 *ipv6 = (struct sockaddr_in6 *)p->ai_addr;
            addr = &(ipv6->sin6_addr);
            ipver = "IPV6";
        }

        inet_ntop(p->ai_family, addr, ipstr, sizeof ipstr);
        printf("  %s: %s\n", ipver, ipstr);
    }

    freeaddrinfo(res);

    return 0;
}