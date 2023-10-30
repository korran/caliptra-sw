module caliptra_fpga_sync_top_vivado    (
        input wire aclk,
        input wire rstn,

        input wire arvalid,
        input wire [31:0] araddr,
        input wire [2:0] arprot,
        output wire arready,

        input wire rready,
        output wire rvalid,
        output wire [63:0] rdata,
        output wire [1:0] rresp,

        input wire awvalid,
        input wire [31:0] awaddr,
        input wire [2:0] awprot,
        output wire awready,

        input wire wvalid,
        input wire [63:0] wdata,
        input wire [7:0] wstrb,
        output wire wready,

        input wire bready,
        output wire bvalid,
        output wire [1:0] bresp
    );

    caliptra_fpga_sync_top regs (
        .aclk(aclk),
        .rstn(rstn),

        .arvalid(arvalid),
        .araddr(araddr),
        .arprot(arprot),
        .arready(arready),

        .rready(rready),
        .rvalid(rvalid),
        .rdata(rdata),
        .rresp(rresp),

        .awvalid(awvalid),
        .awaddr(awaddr),
        .awprot(awprot),
        .awready(awready),

        .wvalid(wvalid),
        .wdata(wdata),
        .wstrb(wstrb),
        .wready(wready),

        .bready(bready),
        .bvalid(bvalid),
        .bresp(bresp)
    );

endmodule
