# MAX3263x

Peripheral access API for MAX3263X microcontrollers (generated using svd2rust).

You can find an overview of the generated API [here](https://docs.rs/svd2rust/0.23.0/svd2rust/#peripheral-api).

The SVD file itself is for the Maxim Integrated MAX32630 and MAX32631 from [here](https://developer.arm.com/tools-and-software/embedded/cmsis/cmsis-packs). The following changes have been made to the original file:

- Remove `alt_sck_hi_clk` and `alt_sck_lo_clk` fields from `SPIM0`'s `MSTR_CFG`.
- Add `sdio_sample_point` field to `SPIM0`'s `MSTR_CFG`.
- Add `enable_sck_fb_mode` field to `SPIM0`'s `GEN_CTRL`.
