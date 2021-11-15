#include <stdbool.h>

#define uint unsigned int

struct Gate {
    struct {
        uint inputs;
        uint *outputs;
        uint output_len;
    } data;
    union {
        struct {
            bool * (*calculate)(bool *inputs);
        } builtin;
        struct {
            struct Gate *nodes;
            uint nodes_len;
        } normal;
        struct {
            int change; // how far to go back or forwards
        } loop;
    } gate;
    enum { Builtin, Normal, Loop } type;
};
