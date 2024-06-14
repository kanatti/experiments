#include <stdio.h>
#include <netinet/in.h>


int main() {
    printf("Size of struct sockaddr: %zu\n", sizeof(struct sockaddr));
    printf("Size of struct sockaddr_in: %zu\n", sizeof(struct sockaddr_in));
    printf("Size of struct sockaddr_in6: %zu\n", sizeof(struct sockaddr_in6));

    printf("Offset of sa_len in struct sockaddr: %zu\n", __offsetof(struct sockaddr, sa_len));
    printf("Offset of sa_family in struct sockaddr: %zu\n", __offsetof(struct sockaddr, sa_family));
    printf("Offset of sa_data in struct sockaddr: %zu\n", __offsetof(struct sockaddr, sa_data));

    printf("Offset of sin_len in struct sockaddr_in: %zu\n", __offsetof(struct sockaddr_in, sin_len));
    printf("Offset of sin_family in struct sockaddr_in: %zu\n", __offsetof(struct sockaddr_in, sin_family));
    printf("Offset of sin_port in struct sockaddr_in: %zu\n", __offsetof(struct sockaddr_in, sin_port));
    printf("Offset of sin_addr in struct sockaddr_in: %zu\n", __offsetof(struct sockaddr_in, sin_addr));
    printf("Offset of sin_zero in struct sockaddr_in: %zu\n", __offsetof(struct sockaddr_in, sin_zero));

    printf("Offset of sin6_len in struct sockaddr_in6: %zu\n", __offsetof(struct sockaddr_in6, sin6_len));
    printf("Offset of sin6_family in struct sockaddr_in6: %zu\n", __offsetof(struct sockaddr_in6, sin6_family));
    printf("Offset of sin6_port in struct sockaddr_in6: %zu\n", __offsetof(struct sockaddr_in6, sin6_port));
    printf("Offset of sin6_flowinfo in struct sockaddr_in6: %zu\n", __offsetof(struct sockaddr_in6, sin6_flowinfo));
    printf("Offset of sin6_addr in struct sockaddr_in6: %zu\n", __offsetof(struct sockaddr_in6, sin6_addr));
    printf("Offset of sin6_scope_id in struct sockaddr_in6: %zu\n", __offsetof(struct sockaddr_in6, sin6_scope_id));

    return 0;
}