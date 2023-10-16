
#include <stdbool.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

struct caliptra_fpga_sync_verilated;

struct caliptra_fpga_sync_sig_in {
    bool aclk;
    bool rstn;

    bool arvalid;
    uint32_t araddr;
    uint8_t arprot;

    bool rready;

    bool awvalid;
    uint32_t awaddr;
    uint8_t awprot;

    bool wvalid;
    uint64_t wdata;
    uint8_t wstrb;

    bool bready;
};

struct caliptra_fpga_sync_sig_out {
    bool arready;

    bool rvalid;
    uint64_t rdata;
    uint8_t rresp;

    bool awready;

    bool wready;

    bool bvalid;
    uint8_t bresp;
};

struct caliptra_fpga_sync_verilated* caliptra_fpga_sync_verilated_new();
void caliptra_fpga_sync_verilated_destroy(struct caliptra_fpga_sync_verilated* model);

void caliptra_fpga_sync_verilated_trace(struct caliptra_fpga_sync_verilated* model,
                              const char* vcd_out_path, int depth);

// Evaluates the model into out, then copies all `in` signals into psuedo
// flip-flops that will be visible to always_ff blocks in subsequent
// evaluations.
void caliptra_fpga_sync_verilated_eval(struct caliptra_fpga_sync_verilated* model,
                             const struct caliptra_fpga_sync_sig_in* in,
                             struct caliptra_fpga_sync_sig_out* out);

#ifdef __cplusplus
}
#endif

