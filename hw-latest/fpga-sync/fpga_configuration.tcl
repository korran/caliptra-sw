# Create path variables
set fpgaDir [file dirname [info script]]
set outputDir $fpgaDir/caliptra_build
set packageDir $outputDir/caliptra_package
set adapterDir $outputDir/soc_adapter_package
# Clean and create output directory.
file delete -force $outputDir
file mkdir $outputDir
file mkdir $packageDir
file mkdir $adapterDir

# Path to rtl
set rtlDir $fpgaDir/../caliptra-rtl

# Simplistic processing of command line arguments to enable different features
# Defaults:
set BUILD FALSE
set GUI   FALSE
set JTAG  TRUE
set ITRNG FALSE
foreach arg $argv {
    regexp {(.*)=(.*)} $arg fullmatch option value
    set $option "$value"
}
# If VERSION was not set by tclargs, set it from the commit ID.
# This assumes it is run from within caliptra-sw. If building from outside caliptra-sw call with "VERSION=[hex number]"
if {[info exists VERSION] == 0} {
  set VERSION [exec git rev-parse --short HEAD]
}

# Set Verilog defines to:
#     Make Caliptra use an icg that doesn't clock gate
#     Make the VEER core be optimized for FPGA (no clock gating)
#     Define VEER TEC_RV_ICG to allow beh_lib to synthesise without error
set VERILOG_OPTIONS {TECH_SPECIFIC_ICG USER_ICG=fpga_fake_icg RV_FPGA_OPTIMIZE TEC_RV_ICG=clockhdr}
if {$ITRNG} {
  # Add option to use Caliptra's internal TRNG instead of ETRNG
  lappend VERILOG_OPTIONS CALIPTRA_INTERNAL_TRNG
}

# Start the Vivado GUI for interactive debug
if {$GUI} {
  start_gui
}

# Create a project to package Caliptra.
# Packaging Caliptra allows Vivado to recognize the APB bus as an endpoint for the memory map.
create_project caliptra_fpga_sync $outputDir -part xczu7ev-ffvc1156-2-e

set_property verilog_define $VERILOG_OPTIONS [current_fileset]

# Add VEER Headers
add_files $rtlDir/src/riscv_core/veer_el2/rtl/el2_param.vh
add_files $rtlDir/src/riscv_core/veer_el2/rtl/pic_map_auto.h
add_files $rtlDir/src/riscv_core/veer_el2/rtl/el2_pdef.vh

# Add VEER sources
add_files [ glob $rtlDir/src/riscv_core/veer_el2/rtl/*.sv ]
add_files [ glob $rtlDir/src/riscv_core/veer_el2/rtl/*/*.sv ]
add_files [ glob $rtlDir/src/riscv_core/veer_el2/rtl/*/*.v ]

# Add Caliptra Headers
add_files [ glob $rtlDir/src/*/rtl/*.svh ]
# Add Caliptra Sources
add_files [ glob $rtlDir/src/*/rtl/*.sv ]
add_files [ glob $rtlDir/src/*/rtl/*.v ]

# Remove spi_host files that aren't used yet and are flagged as having syntax errors
# TODO: Re-include these files when spi_host is used.
remove_files [ glob $rtlDir/src/spi_host/rtl/*.sv ]

remove_files [ glob $rtlDir/src/ecc/rtl/ecc_ram_tdp_file.sv ]

# Key Vault is very large. Replacing KV with a version with the minimum number of entries.
remove_files [ glob $rtlDir/src/keyvault/rtl/kv_reg.sv ]

# Add FPGA specific sources
add_files [ glob $fpgaDir/rtl/*.v]
add_files [ glob $fpgaDir/rtl/*.sv]
add_files [ glob $fpgaDir/../fpga/src/fpga_icg.sv ]
add_files [ glob $fpgaDir/../fpga/src/caliptra_veer_sram_export.sv ]
add_files [ glob $fpgaDir/../fpga/src/kv_reg.sv]

# Mark all Verilog sources as SystemVerilog because some of them have SystemVerilog syntax.
set_property file_type SystemVerilog [get_files *.v]

# Exception: caliptra_fpga_sync_top_vivado.v needs to be Verilog to be included in a Block Diagram.
set_property file_type Verilog [get_files $fpgaDir/rtl/caliptra_fpga_sync_top_vivado.v]

# Add include paths
set_property include_dirs $rtlDir/src/integration/rtl [current_fileset]

# Set caliptra_fpga_sync_top_vivado as top in case next steps fail so that the top is something useful.
set_property top caliptra_fpga_sync_top_vivado [current_fileset]

# Create block diagram that includes an instance of caliptra_fpga_sync_top_vivado
create_bd_design "caliptra_fpga_sync_package_bd"
create_bd_cell -type module -reference caliptra_fpga_sync_top_vivado caliptra_fpga_sync_top_vivado_0

# Add Zynq PS
create_bd_cell -type ip -vlnv xilinx.com:ip:zynq_ultra_ps_e:3.4 zynq_ultra_ps_e_0
set_property CONFIG.PSU__CRL_APB__PL0_REF_CTRL__FREQMHZ {20} [get_bd_cells zynq_ultra_ps_e_0]
set_property CONFIG.PSU__USE__IRQ0 {1} [get_bd_cells zynq_ultra_ps_e_0]
set_property CONFIG.PSU__MAXIGP2__DATA_WIDTH {64} [get_bd_cells zynq_ultra_ps_e_0]

# AXI4->AXI4Lite protocol converter
create_bd_cell -type ip -vlnv xilinx.com:ip:axi_protocol_converter:2.1 axi4toaxi4lite_0
set_property CONFIG.DATA_WIDTH {64} [get_bd_cells axi4toaxi4lite_0]

# Create reset block
create_bd_cell -type ip -vlnv xilinx.com:ip:proc_sys_reset:5.0 proc_sys_reset_0

# Connect ports

connect_bd_intf_net -intf_net zynq_ultra_ps_e_0_M_AXI_HPM0_LPD [get_bd_intf_pins axi4toaxi4lite_0/S_AXI] [get_bd_intf_pins zynq_ultra_ps_e_0/M_AXI_HPM0_LPD]

connect_bd_intf_net -boundary_type upper [get_bd_intf_pins axi4toaxi4lite_0/M_AXI] [get_bd_intf_pins caliptra_fpga_sync_top_vivado_0/interface_aximm]

connect_bd_net -net proc_sys_reset_0_peripheral_aresetn [get_bd_pins caliptra_fpga_sync_top_vivado_0/rstn] [get_bd_pins axi4toaxi4lite_0/aresetn] [get_bd_pins proc_sys_reset_0/peripheral_aresetn]

connect_bd_net -net zynq_ultra_ps_e_0_pl_clk0 [get_bd_pins caliptra_fpga_sync_top_vivado_0/aclk] [get_bd_pins axi4toaxi4lite_0/aclk] [get_bd_pins proc_sys_reset_0/slowest_sync_clk] [get_bd_pins zynq_ultra_ps_e_0/maxihpm0_lpd_aclk] [get_bd_pins zynq_ultra_ps_e_0/pl_clk0]

connect_bd_net -net zynq_ultra_ps_e_0_pl_resetn0 [get_bd_pins proc_sys_reset_0/ext_reset_in] [get_bd_pins zynq_ultra_ps_e_0/pl_resetn0]

# Create address segments
assign_bd_address -offset 0x80000000 -range 0x00002000 -target_address_space [get_bd_addr_spaces zynq_ultra_ps_e_0/Data] [get_bd_addr_segs caliptra_fpga_sync_top_vivado_0/interface_aximm/reg0] -force

# Make the diagram pretty
set_property location {1 340 230} [get_bd_cells zynq_ultra_ps_e_0]
set_property location {1 346 430} [get_bd_cells proc_sys_reset_0]
set_property location {2 895 231} [get_bd_cells axi4toaxi4lite_0]
set_property location {3 1273 318} [get_bd_cells caliptra_fpga_sync_top_vivado_0]

save_bd_design

make_wrapper -files [get_files $outputDir/caliptra_fpga_sync.srcs/sources_1/bd/caliptra_fpga_sync_package_bd/caliptra_fpga_sync_package_bd.bd] -top

add_files -norecurse $outputDir/caliptra_fpga_sync.gen/sources_1/bd/caliptra_fpga_sync_package_bd/hdl/caliptra_fpga_sync_package_bd_wrapper.v

update_compile_order -fileset sources_1

set_property STEPS.WRITE_BITSTREAM.ARGS.BIN_FILE true [get_runs impl_1]
set_property STEPS.SYNTH_DESIGN.ARGS.GATED_CLOCK_CONVERSION on [get_runs synth_1]

close_bd_design [get_bd_designs caliptra_fpga_sync_package_bd]

# Start build
if {$BUILD} {
  launch_runs synth_1 -jobs 10
  wait_on_runs synth_1
  launch_runs impl_1 -jobs 10
  wait_on_runs impl_1
  open_run impl_1
  report_utilization -file $outputDir/utilization.txt
  write_bitstream -bin_file $outputDir/caliptra_fpga
}

