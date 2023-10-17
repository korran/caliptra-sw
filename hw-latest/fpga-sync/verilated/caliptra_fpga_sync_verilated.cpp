
#include "caliptra_fpga_sync_verilated.h"

#include "Vcaliptra_fpga_sync_top.h"
#include "verilated_vcd_c.h"

#include <memory>

struct caliptra_fpga_sync_verilated {
  Vcaliptra_fpga_sync_top v;
  std::unique_ptr<VerilatedVcdC> tfp;
  uint64_t sim_time = 0;
};


struct caliptra_fpga_sync_verilated* caliptra_fpga_sync_verilated_new() {
  return new caliptra_fpga_sync_verilated();
}

void caliptra_fpga_sync_verilated_destroy(struct caliptra_fpga_sync_verilated* model) {
  if (model->tfp.get()) {
    model->tfp->close();
  }
  delete model;
}

void caliptra_fpga_sync_verilated_trace(struct caliptra_fpga_sync_verilated* model,
                              const char* vcd_out_path, int depth) {
  Verilated::traceEverOn(vcd_out_path ? true : false);
  if (model->tfp.get()) {
    model->tfp->close();
  }
  model->tfp.reset(NULL);

  if (vcd_out_path) {
    model->tfp.reset(new VerilatedVcdC());

    model->v.trace(model->tfp.get(), depth);
    model->tfp->open(vcd_out_path);
  }
}



void caliptra_fpga_sync_verilated_eval(struct caliptra_fpga_sync_verilated* model,
                             const struct caliptra_fpga_sync_sig_in* in,
                             struct caliptra_fpga_sync_sig_out* out) {

  Vcaliptra_fpga_sync_top* v = &model->v;
  v->eval();

  v->aclk = in->aclk;
  v->rstn = in->rstn;

  v->arvalid = in->arvalid;
  v->araddr = in->araddr;
  v->arprot = in->arprot;

  v->rready = in->rready;

  v->awvalid = in->awvalid;
  v->awaddr = in->awaddr;
  v->awprot = in->awprot;

  v->wvalid = in->wvalid;
  v->wdata = in->wdata;
  v->wstrb = in->wstrb;

  v->bready = in->bready;

  if (model->tfp.get()) {
    model->tfp->dump(model->sim_time++);
  }

  out->arready = v->arready;
  
  out->rvalid = v->rvalid;
  out->rdata = v->rdata;
  out->rresp = v->rresp;

  out->awready = v->awready;

  out->wready = v->wready;

  out->bvalid = v->bvalid;
  out->bresp = v->bresp;
}
