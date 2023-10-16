
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

    reg [63:0] counter;

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

        .hwif_in (hwif_in),
        .hwif_out(hwif_out)
    );

    logic aclk_gated;

    reg [31:0] aclk_gated_cycle_count;
    reg aclk_gated_en;
    assign aclk_gated = aclk_gated_en && aclk;


    always @ (negedge aclk or negedge rstn) begin : reg_update
        if (!rstn) begin
            aclk_gated_en <= '0;
        end
        else begin
            if (hwif_out.clock_control.go.value)
            begin
                aclk_gated_cycle_count <= hwif_out.clock_control.cycle_count.value - 1;
                aclk_gated_en <= 1'b1;
            end
            else if (aclk_gated_cycle_count > 0) begin
                aclk_gated_cycle_count <= aclk_gated_cycle_count - 1;
            end
            else begin
                aclk_gated_en <= '0;
            end
        end
    end

    always @ (posedge aclk_gated) begin : reg_update_gated
        counter <= counter + 1;
    end // reg_update_gated

    always_comb begin
        hwif_in.clock_control.cycle_count.next = aclk_gated_cycle_count;

        hwif_in.counter.counter.next = counter;
    end

endmodule