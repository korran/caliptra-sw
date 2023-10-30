module caliptra_fpga_sync_top_vivado    (
        input aclk,
        input rstn,

        input arvalid,
        input [31:0] araddr,
        input [2:0] arprot,
        output arready,

        input rready,
        output rvalid,
        output [63:0] rdata,
        output [1:0] rresp,

        input awvalid,
        input [31:0] awaddr,
        input [2:0] awprot,
        output awready,

        input wvalid,
        input [63:0] wdata,
        input [7:0] wstrb,
        output wready,

        input bready,
        output bvalid,
        output [1:0] bresp
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
