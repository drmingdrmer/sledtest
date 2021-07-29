#include <stdio.h>
#include <fcntl.h>
#include <unistd.h>
#include <stdlib.h>
#include <errno.h>
#include <string.h>

#define N 1000

int main()
{
    const char *teststr = "test";
    int fd = open("c.dump", O_WRONLY|O_CREAT|O_TRUNC);
    if (fd < 0) {
        printf("open fail");
        return -1;
    }
    printf("fd=%d",fd);

    for (int i=0; i<N; i++) {
        int n = write(fd, teststr, 4);
        if (n<0 ) {
            printf("write fail, errno: %d %s", errno, strerror(errno));
            return -1;
        }
        int k = fcntl(fd, F_FULLFSYNC);
        if (k!=0) {
          printf("fcntl fail, errno: %d %s", errno, strerror(errno));
          return -1;
        }
        int j = fsync(fd);
        if ( j!= 0) {
            printf("sync fail");
            return -1;
        }

    }
    close(fd);
    return 0;
}
