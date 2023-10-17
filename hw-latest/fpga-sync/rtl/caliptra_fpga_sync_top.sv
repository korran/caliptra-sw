
`default_nettype none

`include "config_defines.svh"
`include "caliptra_macros.svh"

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


    import caliptra_top_tb_pkg::*;
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

        .mailbox_data_avail(),
        .mailbox_flow_done(),
        .BootFSM_BrkPoint('x), //FIXME TIE-OFF

        .generic_input_wires(hwif_out.generic_input_wires.value.value), //FIXME TIE-OFF
        .generic_output_wires(hwif_in.generic_output_wires.value.next),

        .scan_mode(),

        //FIXME: export these
        .cptra_error_fatal(),
        .cptra_error_non_fatal(),

        .etrng_req(hwif_in.trng_out.etrng_req.next),
        .itrng_data(hwif_out.trng_in.itrng_data.value),
        .itrng_valid(hwif_out.trng_in.itrng_valid.value),

        .security_state({hwif_out.control.ss_debug_locked.value, hwif_out.control.ss_device_lifecycle.value})
    );


    caliptra_veer_sram_export veer_sram_export_inst (
        .sram_error_injection_mode('0),
        .el2_mem_export(el2_mem_export.veer_sram_sink)
    );

    //SRAM for mbox (preload raw data here)
    caliptra_sram
    #(
        .DATA_WIDTH(MBOX_DATA_W),
        .DEPTH     (MBOX_DEPTH )
    )
    dummy_mbox_preloader
    (
        .clk_i(aclk_gated),

        .cs_i   (),
        .we_i   (),
        .addr_i (),
        .wdata_i(),
        .rdata_o()
    );
    // Actual Mailbox RAM -- preloaded with data from
    // dummy_mbox_preloader with ECC bits appended
    caliptra_sram
    #(
        .DATA_WIDTH(MBOX_DATA_AND_ECC_W),
        .DEPTH     (MBOX_DEPTH         )
    )
    mbox_ram1
    (
        .clk_i(aclk_gated),

        .cs_i(mbox_sram_cs),
        .we_i(mbox_sram_we),
        .addr_i(mbox_sram_addr),
        .wdata_i(mbox_sram_wdata ^ mbox_sram_wdata_bitflip),

        .rdata_o(mbox_sram_rdata)
    );

    assign hwif_in.rom_mem.rd_ack = '1;
    assign hwif_in.rom_mem.wr_ack = '1;

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
        .b_addr_i  (hwif_out.rom_mem.addr),
        .b_wdata_i (hwif_out.rom_mem.wr_data),
        .b_rdata_o (hwif_in.rom_mem.rd_data)
    );

    // This is used to load the generated ICCM hexfile prior to
    // running slam_iccm_ram
    caliptra_sram #(
        .DEPTH     (16384        ), // 128KiB
        .DATA_WIDTH(64           ),
        .ADDR_WIDTH($clog2(16384))

    ) dummy_iccm_preloader (
        .clk_i   (aclk_gated),

        .cs_i    (        ),
        .we_i    (        ),
        .addr_i  (        ),
        .wdata_i (        ),
        .rdata_o (        )
    );


    // This is used to load the generated DCCM hexfile prior to
    // running slam_dccm_ram
    caliptra_sram #(
        .DEPTH     (16384        ), // 128KiB
        .DATA_WIDTH(64           ),
        .ADDR_WIDTH($clog2(16384))

    ) dummy_dccm_preloader (
        .clk_i   (),

        .cs_i    (        ),
        .we_i    (        ),
        .addr_i  (        ),
        .wdata_i (        ),
        .rdata_o (        )
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

    //storage element
    logic [7:0] ram [DEPTH][NUM_BYTES-1:0];

    always @(posedge a_clk_i) begin
        if (a_cs_i & a_we_i) begin
            for (int i = 0; i < NUM_BYTES; i++) begin
                ram[a_addr_i][i] <= a_wdata_i[i*8 +: 8];
            end
        end
        if (a_cs_i & ~a_we_i) begin
            for (int i = 0; i < NUM_BYTES; i++) begin
                a_rdata_o[i*8 +: 8] <= ram[a_addr_i][i];
            end
        end
    end

    always @(posedge b_clk_i) begin
        if (b_cs_i & b_we_i) begin
            for (int i = 0; i < NUM_BYTES; i++) begin
                ram[b_addr_i][i] <= b_wdata_i[i*8 +: 8];
            end
        end
        if (b_cs_i & ~b_we_i) begin
            for (int i = 0; i < NUM_BYTES; i++) begin
                b_rdata_o[i*8 +: 8] <= ram[b_addr_i][i];
            end
        end
    end

endmodule