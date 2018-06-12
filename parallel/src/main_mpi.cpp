#include <stdio.h>
#include <time.h>
#include <iostream>
#include <mpi.h>
#include "bmp.cpp"

using namespace std;


char toGrayScale(char red, char green, char blue) {
   return (char)(0.2126f * red + 0.7512f * green + 0.0722 * blue);
}

int SIZE_PER_NODE = 128 * 256 * 3;

int main (int argc, char *argv[]) {
    MPI_Init(NULL, NULL);
    
    int world_size;
    MPI_Comm_size(MPI_COMM_WORLD, &world_size);

    int world_rank;
    MPI_Comm_rank(MPI_COMM_WORLD, &world_rank);
    if (world_rank == 0) {
        printf("master %i\n", world_rank);
        char const *inputFile = "data/in.bmp";
        char const *outputFile = "data/out.bmp";
        clock_t start, end;
        unsigned char pixel[3];

        unsigned char image[256][256][3];
        readBotmapImage(inputFile, image);
        start = clock();

        MPI_Send(image, SIZE_PER_NODE, MPI_UNSIGNED_CHAR, 1, 0, MPI_COMM_WORLD);
        MPI_Send(image[128], SIZE_PER_NODE, MPI_UNSIGNED_CHAR, 2, 0, MPI_COMM_WORLD);

        MPI_Recv(image, SIZE_PER_NODE, MPI_UNSIGNED_CHAR, 1, 0, MPI_COMM_WORLD, MPI_STATUS_IGNORE);
        MPI_Recv(image[128], SIZE_PER_NODE, MPI_UNSIGNED_CHAR, 2, 0, MPI_COMM_WORLD, MPI_STATUS_IGNORE);
        end = clock();
        printf("master time %d\n", end-start);
        generateBitmapImage(image, 256, 256, outputFile);

    } else {
        printf("slave %i\n", world_rank);

        char grayScale;
        unsigned char subimage[128][256][3];
        MPI_Recv(subimage, SIZE_PER_NODE, MPI_UNSIGNED_CHAR, 0, 0, MPI_COMM_WORLD, MPI_STATUS_IGNORE);
        for (int i = 0; i < 128; i++) {
            for (int j = 0; j < 256; j++) {
                grayScale = toGrayScale(subimage[i][j][0], subimage[i][j][1], subimage[i][j][2]);
                subimage[i][j][0] = grayScale;
                subimage[i][j][1] = grayScale;
                subimage[i][j][2] = grayScale;
            }
        }
        MPI_Send(subimage, SIZE_PER_NODE, MPI_UNSIGNED_CHAR, 0, 0, MPI_COMM_WORLD);        
    }

    MPI_Finalize();

    return 0;
}
