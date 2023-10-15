

module caliptra_fpga_sync_top(
        input aclk;
        input rstn;

        input arvalid,
        input [31:0] araddr,
        input [2:0] arprot,
        output arready,

        input rready,
        output rvalid,
        output [63:0] rdata,
        output [1:0] rresp

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
        output [1:0] bresp,
    );

    axi4lite_intf.slave s_axil;

    always_comb begin
        awready = s_axil.AWREADY;
        s_axis.AWVALID = awvalid;
        s_axis.AWADDR = awaddr;
        s_axis.AWPROT = awprot;

        wready = s_axil.WREADY;
        s_axis.WVALID = wvalid;
        s_axis.WDATA = wdata;
        s_axis.WSTRB = wstrb;

        s_axis.BREADY = bready;
        bvalid = s_axil.BVALID;
        bresp = s_axil.BRESP;

        arready = s_axil.ARREADY;
        s_axis.ARVALID = arvalid;
        s_axis.ARADDR = araddr;
        s_axis.ARPROT = arprot;

        s_axis.RREADY = rready;
        ravlid = s_axil.RVALID;
        rdata = s_axil.RDATA;
        rresp = s_axil.RRESP;
    end


endmodule