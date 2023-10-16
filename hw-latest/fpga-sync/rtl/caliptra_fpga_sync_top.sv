
module caliptra_fpga_sync_top
        import caliptra_fpga_sync_regs_pkg::*;
    (
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

    axi4lite_intf s_axil ();

    caliptra_fpga_sync_regs__in_t hwif_in;
    caliptra_fpga_sync_regs__out_t hwif_out;

    assign awready = s_axil.AWREADY;
    assign wready = s_axil.WREADY;
    assign bvalid = s_axil.BVALID;
    assign bresp = s_axil.BRESP;
    assign arready = s_axil.ARREADY;
    assign rvalid = s_axil.RVALID;
    assign rdata = s_axil.RDATA;
    assign rresp = s_axil.RRESP;

    always_comb begin
        s_axil.AWVALID = awvalid;
        s_axil.AWADDR = awaddr;
        s_axil.AWPROT = awprot;

        s_axil.WVALID = wvalid;
        s_axil.WDATA = wdata;
        s_axil.WSTRB = wstrb;

        s_axil.BREADY = bready;

        s_axil.ARVALID = arvalid;
        s_axil.ARADDR = araddr;
        s_axil.ARPROT = arprot;

        s_axil.RREADY = rready;
    end

    // Register Block
    caliptra_fpga_sync_regs regs (
        .clk(aclk),
        .rst(1'b0),

        .s_axil(s_axil),

        .hwif_in (hwif_in ),
        .hwif_out(hwif_out)
    );


    


endmodule