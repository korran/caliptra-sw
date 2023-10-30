
// `default_nettype none

`include "config_defines.svh"
`include "caliptra_macros.svh"

module caliptra_fpga_sync_top
        import caliptra_fpga_sync_regs_pkg::*;
    (
        (* gated_clock = "yes" *) input aclk,
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

    import soc_ifc_pkg::*;

    logic [`CALIPTRA_IMEM_ADDR_WIDTH-1:0] imem_addr;
    logic [`CALIPTRA_IMEM_DATA_WIDTH-1:0] imem_rdata;
    logic imem_cs;

    logic mbox_sram_cs;
    logic mbox_sram_we;
    logic [14:0] mbox_sram_addr;
    logic [MBOX_DATA_AND_ECC_W-1:0] mbox_sram_wdata;
    logic [MBOX_DATA_AND_ECC_W-1:0] mbox_sram_wdata_bitflip;
    logic [MBOX_DATA_AND_ECC_W-1:0] mbox_sram_rdata;

    el2_mem_if el2_mem_export ();

    reg [63:0] generic_output_wires_prev;
    wire [63:0] generic_output_wires;
    assign hwif_in.generic_output_wires.value.next = generic_output_wires;
    wire bkpt_generic_output_wires = generic_output_wires_prev != generic_output_wires && !hwif_out.clock_control.bkpt_generic_output_wires.value;
    assign hwif_in.clock_control.bkpt_generic_output_wires.next = hwif_out.clock_control.bkpt_generic_output_wires.value | bkpt_generic_output_wires;

    wire mailbox_data_avail;
    assign hwif_in.status.mailbox_data_avail.next = mailbox_data_avail;
    wire bkpt_mailbox_data_avail = mailbox_data_avail && !hwif_out.clock_control.bkpt_mailbox_data_avail.value;
    assign hwif_in.clock_control.bkpt_mailbox_data_avail.next = hwif_out.clock_control.bkpt_mailbox_data_avail.value | bkpt_mailbox_data_avail;

    wire mailbox_flow_done;
    assign hwif_in.status.mailbox_flow_done.next = mailbox_flow_done;
    wire bkpt_mailbox_flow_done = mailbox_flow_done && !hwif_out.clock_control.bkpt_mailbox_flow_done.value;
    assign hwif_in.clock_control.bkpt_mailbox_flow_done.next = hwif_out.clock_control.bkpt_mailbox_flow_done.value | bkpt_mailbox_flow_done;

    wire etrng_req;
    assign hwif_in.trng_out.etrng_req.next = etrng_req;
    wire bkpt_etrng_req = etrng_req && !hwif_out.clock_control.bkpt_etrng_req.value;
    assign hwif_in.clock_control.bkpt_etrng_req.next = hwif_out.clock_control.bkpt_etrng_req.value | bkpt_etrng_req;

    assign hwif_in.clock_control.go.next = aclk_gated_en;

    always @ (negedge aclk or negedge rstn) begin : reg_update
        if (!rstn) begin
            aclk_gated_en <= '0;
        end
        else begin
            if (hwif_out.clock_control.go.value && !aclk_gated_en)
            begin
                aclk_gated_cycle_count <= hwif_out.clock_control.cycle_count.value - 1;
                aclk_gated_en <= 1'b1;
            end
            else if (aclk_gated_cycle_count > 0) begin
                if (bkpt_generic_output_wires || bkpt_mailbox_data_avail || bkpt_mailbox_flow_done || bkpt_etrng_req) 
                    aclk_gated_en <= 0;
                else 
                    aclk_gated_cycle_count <= aclk_gated_cycle_count - 1;
            end
            else begin
                aclk_gated_en <= '0;
            end
        end
    end



    always @ (posedge aclk_gated) begin : reg_update_gated
        counter <= counter + 1;
        generic_output_wires_prev <= generic_output_wires;
    end // reg_update_gated

    always_comb begin
        hwif_in.clock_control.cycle_count.next = aclk_gated_cycle_count;

        hwif_in.counter.counter.next = counter;
    end

    caliptra_top caliptra_top_dut (
        .cptra_pwrgood              (hwif_out.control.cptra_pwrgood.value),
        .cptra_rst_b                (hwif_out.control.cptra_rst_b.value),
        .clk                        (aclk_gated),

        .cptra_obf_key              ({hwif_out.cptra_obf_key[0].value.value,
                                    hwif_out.cptra_obf_key[1].value.value,
                                    hwif_out.cptra_obf_key[2].value.value,
                                    hwif_out.cptra_obf_key[3].value.value}),

        .jtag_tck(1'b0),
        .jtag_tdi(1'b0),
        .jtag_tms(1'b0),
        .jtag_trst_n(1'b0),
        .jtag_tdo(),

        .PADDR(hwif_out.apb_in0.paddr.value),
        .PPROT(hwif_out.apb_in1.pprot.value),
        .PAUSER(hwif_out.apb_in1.pauser.value),
        .PENABLE(hwif_out.apb_in1.penable.value),
        .PRDATA(hwif_in.apb_out.pdata.next),
        .PREADY(hwif_in.apb_out.pready.next),
        .PSEL(hwif_out.apb_in1.psel.value),
        .PSLVERR(hwif_in.apb_out.pslverr.next),
        .PWDATA(hwif_out.apb_in0.pdata.value),
        .PWRITE(hwif_out.apb_in1.pwrite.value),

        .qspi_clk_o(),
        .qspi_cs_no(),
        .qspi_d_i(4'b0),
        .qspi_d_o(),
        .qspi_d_en_o(),

        .el2_mem_export(el2_mem_export.veer_sram_src),

        .ready_for_fuses(hwif_in.status.ready_for_fuses.next),
        .ready_for_fw_push(hwif_in.status.ready_for_fw_push.next),
        .ready_for_runtime(hwif_in.status.ready_for_runtime.next),

        .mbox_sram_cs(mbox_sram_cs),
        .mbox_sram_we(mbox_sram_we),
        .mbox_sram_addr(mbox_sram_addr),
        .mbox_sram_wdata(mbox_sram_wdata),
        .mbox_sram_rdata(mbox_sram_rdata),

        .imem_cs(imem_cs),
        .imem_addr(imem_addr),
        .imem_rdata(imem_rdata),

        .mailbox_data_avail(mailbox_data_avail),
        .mailbox_flow_done(mailbox_flow_done),
        .BootFSM_BrkPoint('x), //FIXME TIE-OFF

        .generic_input_wires(hwif_out.generic_input_wires.value.value), //FIXME TIE-OFF
        .generic_output_wires(generic_output_wires),

        .scan_mode(),

        //FIXME: export these
        .cptra_error_fatal(),
        .cptra_error_non_fatal(),

        .etrng_req(etrng_req),
        .itrng_data(hwif_out.trng_in.itrng_data.value),
        .itrng_valid(hwif_out.trng_in.itrng_valid.value),

        .security_state({hwif_out.control.ss_debug_locked.value, hwif_out.control.ss_device_lifecycle.value})
    );

    caliptra_veer_sram_export veer_sram_export_inst (
        .el2_mem_export(el2_mem_export.veer_sram_sink)
    );

    caliptra_sram_dual
    #(
        .DATA_WIDTH(MBOX_DATA_AND_ECC_W),
        .DEPTH     (MBOX_DEPTH         )
    )
    mbox_ram1
    (
        .a_clk_i(aclk_gated),

        .a_cs_i(mbox_sram_cs),
        .a_we_i(mbox_sram_we),
        .a_addr_i(mbox_sram_addr),
        .a_wdata_i(mbox_sram_wdata),

        .a_rdata_o(mbox_sram_rdata),

        .b_clk_i('0),

        .b_cs_i('0),
        .b_we_i('0),
        .b_addr_i('0),
        .b_wdata_i('0),

        .b_rdata_o()
    );

    assign hwif_in.rom_mem.rd_ack = hwif_out.rom_mem.req && !hwif_out.rom_mem.req_is_wr;
    assign hwif_in.rom_mem.wr_ack = hwif_out.rom_mem.req && hwif_out.rom_mem.req_is_wr;

    //SRAM for imem
    caliptra_sram_dual #(
        .DEPTH     (`CALIPTRA_IMEM_DEPTH     ), // Depth in WORDS
        .DATA_WIDTH(`CALIPTRA_IMEM_DATA_WIDTH),
        .ADDR_WIDTH(`CALIPTRA_IMEM_ADDR_WIDTH)
    ) imem_inst1 (
        .a_clk_i   (aclk_gated),
        .a_cs_i    (imem_cs),
        .a_we_i    (),
        .a_addr_i  (imem_addr),
        .a_wdata_i (),
        .a_rdata_o (imem_rdata),

        .b_clk_i   (aclk),
        .b_cs_i    (hwif_out.rom_mem.req),
        .b_we_i    (hwif_out.rom_mem.req_is_wr),
        .b_addr_i  (hwif_out.rom_mem.addr[15:3]),
        .b_wdata_i (hwif_out.rom_mem.wr_data),
        .b_rdata_o (hwif_in.rom_mem.rd_data)
    );

endmodule


module caliptra_sram_dual #(
     parameter DEPTH      = 64
    ,parameter DATA_WIDTH = 32
    ,parameter ADDR_WIDTH = $clog2(DEPTH)

    )
    (
    input  logic                       a_clk_i,

    input  logic                       a_cs_i,
    input  logic                       a_we_i,
    input  logic [ADDR_WIDTH-1:0]      a_addr_i,
    input  logic [DATA_WIDTH-1:0]      a_wdata_i,
    output logic [DATA_WIDTH-1:0]      a_rdata_o,

    input  logic                       b_clk_i,

    input  logic                       b_cs_i,
    input  logic                       b_we_i,
    input  logic [ADDR_WIDTH-1:0]      b_addr_i,
    input  logic [DATA_WIDTH-1:0]      b_wdata_i,
    output logic [DATA_WIDTH-1:0]      b_rdata_o
    );

    localparam NUM_BYTES = DATA_WIDTH/8 + ((DATA_WIDTH % 8) ? 1 : 0);

    (* ram_style = "block" *)
    reg [DATA_WIDTH-1:0] ram [DEPTH];

    always @(posedge a_clk_i) begin
        if (a_cs_i & a_we_i) begin
            ram[a_addr_i] <= a_wdata_i;
        end
        if (a_cs_i & ~a_we_i) begin
            a_rdata_o <= ram[a_addr_i];
        end
    end

    always @(posedge b_clk_i) begin
        if (b_cs_i & b_we_i) begin
            ram[b_addr_i] <= b_wdata_i;
        end
        if (b_cs_i & ~b_we_i) begin
            b_rdata_o <= ram[b_addr_i];
        end
    end

endmodule
