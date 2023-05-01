#[allow(unused, non_camel_case_types)]
pub enum HellStyle {
	width_0,
	width_0d5,
	width_1,
	width_1d5,
	width_2,
	width_2d5,
	width_3,
	width_3d5,
	width_4,
	width_5,
	width_6,
	width_7,
	width_8,
	width_9,
	width_10,
	width_px,
	width_auto,
	width_1s2,
	width_1s3,
	width_2s3,
	width_1s4,
	width_2s4,
	width_3s4,
	width_1s5,
	width_2s5,
	width_3s5,
	width_4s5,
	width_1s20,
	width_2s20,
	width_3s20,
	width_4s20,
	width_5s20,
	width_6s20,
	width_7s20,
	width_8s20,
	width_9s20,
	width_10s20,
	width_11s20,
	width_12s20,
	width_13s20,
	width_14s20,
	width_15s20,
	width_16s20,
	width_17s20,
	width_18s20,
	width_19s20,
	width_full,
	width_screen,
	width_min,
	width_max,
	width_fit,
	min_width_0,
	min_width_full,
	min_width_min,
	min_width_max,
	min_width_fit,
	max_width_0,
	max_width_none,
	max_width_xs,
	max_width_sm,
	max_width_md,
	max_width_lg,
	max_width_xl,
	max_width_2xl,
	max_width_3xl,
	max_width_4xl,
	max_width_5xl,
	max_width_6xl,
	max_width_7xl,
	max_width_full,
	max_width_max,
	max_width_min,
	max_width_fit,
	height_0,
	height_0d5,
	height_1,
	height_1d5,
	height_2,
	height_2d5,
	height_3,
	height_3d5,
	height_4,
	height_5,
	height_6,
	height_8,
	height_10,
	height_12,
	height_14,
	height_16,
	height_20,
	height_24,
	height_28,
	height_32,
	height_36,
	height_40,
	height_px,
	height_auto,
	height_1s2,
	height_1s3,
	height_2s3,
	height_1s4,
	height_2s4,
	height_3s4,
	height_1s5,
	height_2s5,
	height_3s5,
	height_4s5,
	height_1s20,
	height_2s20,
	height_3s20,
	height_4s20,
	height_5s20,
	height_6s20,
	height_7s20,
	height_8s20,
	height_9s20,
	height_10s20,
	height_11s20,
	height_12s20,
	height_13s20,
	height_14s20,
	height_15s20,
	height_16s20,
	height_17s20,
	height_18s20,
	height_19s20,
	height_full,
	height_screen,
	height_min,
	height_max,
	height_fit,
	min_height_0,
	min_height_full,
	min_height_screen,
	min_height_min,
	min_height_max,
	min_height_fit,
	max_height_0,
	max_height_0d5,
	max_height_1,
	max_height_1d5,
	max_height_2,
	max_height_2d5,
	max_height_3,
	max_height_3d5,
	max_height_4,
	max_height_5,
	max_height_6,
	max_height_8,
	max_height_10,
	max_height_px,
	max_height_auto,
	max_height_full,
	max_height_screen,
	max_height_min,
	max_height_max,
	max_height_fit,
	pos_static,
	pos_fixed,
	pos_absolute,
	pos_relative,
	pos_sticky,
	inset_0,
	inset_y_0,
	inset_x_0,
	top_0,
	top_50pct,
	right_0,
	right_50pct,
	bottom_0,
	bottom_1,
	bottom_2,
	bottom_3,
	bottom_4,
	bottom_50pct,
	left_0,
	left_50pct,
	pad_0,
	pad_0d5,
	pad_1,
	pad_1d5,
	pad_2,
	pad_2d5,
	pad_3,
	pad_3d5,
	pad_4,
	pad_5,
	pad_6,
	pad_8,
	pad_10,
	pad_x_0,
	pad_x_0d5,
	pad_x_1,
	pad_x_1d5,
	pad_x_2,
	pad_x_2d5,
	pad_x_3,
	pad_x_3d5,
	pad_x_4,
	pad_x_5,
	pad_x_6,
	pad_x_8,
	pad_x_10,
	pad_y_0,
	pad_y_0d5,
	pad_y_1,
	pad_y_1d5,
	pad_y_2,
	pad_y_2d5,
	pad_y_3,
	pad_y_3d5,
	pad_y_4,
	pad_y_5,
	pad_y_6,
	pad_y_8,
	pad_y_10,
	mgn_auto,
	mgn_0,
	mgn_0d5,
	mgn_1,
	mgn_1d5,
	mgn_2,
	mgn_2d5,
	mgn_3,
	mgn_3d5,
	mgn_4,
	mgn_5,
	mgn_6,
	mgn_8,
	mgn_10,
	mgn_top_0,
	mgn_top_0d5,
	mgn_top_1,
	mgn_top_1d5,
	mgn_top_2,
	mgn_top_2d5,
	mgn_top_3,
	mgn_top_3d5,
	mgn_top_4,
	mgn_top_5,
	mgn_top_6,
	mgn_top_8,
	mgn_top_10,
	mgn_top_12,
	mgn_top_14,
	mgn_top_16,
	mgn_top_18,
	mgn_top_20,
	mgn_top_1s20,
	mgn_top_2s20,
	mgn_top_3s20,
	mgn_top_4s20,
	mgn_top_5s20,
	mgn_top_6s20,
	mgn_top_7s20,
	mgn_top_8s20,
	mgn_top_9s20,
	mgn_top_10s20,
	mgn_top_11s20,
	mgn_top_12s20,
	mgn_top_13s20,
	mgn_top_14s20,
	mgn_top_15s20,
	mgn_top_16s20,
	mgn_top_17s20,
	mgn_top_18s20,
	mgn_top_19s20,
	mgn_bottom_0,
	mgn_bottom_0d5,
	mgn_bottom_1,
	mgn_bottom_1d5,
	mgn_bottom_2,
	mgn_bottom_2d5,
	mgn_bottom_3,
	mgn_bottom_3d5,
	mgn_bottom_4,
	mgn_bottom_5,
	mgn_bottom_6,
	mgn_bottom_8,
	mgn_bottom_10,
	mgn_bottom_12,
	mgn_bottom_14,
	mgn_bottom_16,
	mgn_bottom_18,
	mgn_bottom_20,
	mgn_bottom_1s20,
	mgn_bottom_2s20,
	mgn_bottom_3s20,
	mgn_bottom_4s20,
	mgn_bottom_5s20,
	mgn_bottom_6s20,
	mgn_bottom_7s20,
	mgn_bottom_8s20,
	mgn_bottom_9s20,
	mgn_bottom_10s20,
	mgn_bottom_11s20,
	mgn_bottom_12s20,
	mgn_bottom_13s20,
	mgn_bottom_14s20,
	mgn_bottom_15s20,
	mgn_bottom_16s20,
	mgn_bottom_17s20,
	mgn_bottom_18s20,
	mgn_bottom_19s20,
	overflow_auto,
	overflow_x_auto,
	overflow_y_auto,
	overflow_hidden,
	overflow_x_hidden,
	overflow_y_hidden,
	overflow_visible,
	overflow_x_visible,
	overflow_y_visible,
	overflow_scroll,
	overflow_x_scroll,
	overflow_y_scroll,
	bg_primary_400,
	bg_secondary_400,
	bg_accent_400,
	bg_submit_400,
	bg_deny_400,
	bg_inactive_400,
	txt_primary_400,
	txt_secondary_400,
	txt_accent_400,
	txt_submit_400,
	txt_deny_400,
	txt_inactive_400,
	txt_left,
	txt_center,
	txt_right,
	txt_justify,
	txt_xs,
	txt_sm,
	txt_base,
	txt_lg,
	txt_xl,
	txt_2xl,
	txt_3xl,
	txt_4xl,
	txt_5xl,
	txt_6xl,
	txt_7xl,
	txt_8xl,
	txt_9xl,
	flex,
	flex_row,
	flex_row_reverse,
	flex_col,
	flex_col_reverse,
	grid,
	grid_cols_1,
	grid_cols_2,
	grid_cols_3,
	grid_cols_4,
	grid_cols_5,
	grid_cols_6,
	grid_cols_7,
	grid_cols_8,
	grid_cols_9,
	grid_cols_10,
	grid_cols_11,
	grid_cols_12,
	grid_cols_none,
	grid_rows_1,
	grid_rows_2,
	grid_rows_3,
	grid_rows_4,
	grid_rows_5,
	grid_rows_6,
	grid_rows_none,
	col_auto,
	col_span_1,
	col_span_2,
	col_span_3,
	col_span_4,
	col_span_5,
	col_span_6,
	col_span_7,
	col_span_8,
	col_span_9,
	col_span_10,
	col_span_11,
	col_span_12,
	col_start_1,
	col_start_2,
	col_start_3,
	col_start_4,
	col_start_5,
	col_start_6,
	col_start_7,
	col_start_8,
	col_start_9,
	col_start_10,
	col_start_11,
	col_start_12,
	col_end_1,
	col_end_2,
	col_end_3,
	col_end_4,
	col_end_5,
	col_end_6,
	col_end_7,
	col_end_8,
	col_end_9,
	col_end_10,
	col_end_11,
	col_end_12,
	col_end_auto,
	row_auto,
	row_span_1,
	row_span_2,
	row_span_3,
	row_span_4,
	row_span_5,
	row_span_6,
	row_start_1,
	row_start_2,
	row_start_3,
	row_start_4,
	row_start_5,
	row_start_6,
	row_start_auto,
	row_end_1,
	row_end_2,
	row_end_3,
	row_end_4,
	row_end_5,
	row_end_6,
	row_end_auto,
	grid_flow_row,
	grid_flow_col,
	grid_flow_row_dense,
	grid_flow_col_dense,
	auto_cols_auto,
	auto_cols_min,
	auto_cols_max,
	auto_cols_fr,
	flex_1,
	flex_auto,
	flex_initial,
	flex_none,
	gap_0,
	gap_0d5,
	gap_1,
	gap_1d5,
	gap_2,
	gap_2d5,
	gap_3,
	gap_3d5,
	gap_4,
	justify_start,
	justify_center,
	justify_end,
	justify_between,
	justify_around,
	justify_evenly,
	justify_items_stretch,
	justify_items_start,
	justify_items_center,
	justify_items_end,
	justify_self_stretch,
	justify_self_start,
	justify_self_center,
	justify_self_end,
	justify_self_auto,
	content_start,
	content_center,
	content_end,
	content_between,
	content_around,
	content_evenly,
	items_stretch,
	items_start,
	items_center,
	items_end,
	items_baseline,
	border_solid,
	border_dashed,
	border_dotted,
	border_double,
	border_hidden,
	border_none,
	rounded_none,
	rounded_sm,
	rounded,
	rounded_md,
	rounded_lg,
	rounded_xl,
	rounded_2xl,
	rounded_3xl,
	rounded_full,
	border,
	border_0,
	border_2,
	border_4,
	border_8,
	border_t,
	border_t_0,
	border_t_2,
	border_t_4,
	border_t_8,
	border_r,
	border_r_0,
	border_r_2,
	border_r_4,
	border_r_8,
	border_b,
	border_b_0,
	border_b_2,
	border_b_4,
	border_b_8,
	border_l,
	border_l_0,
	border_l_2,
	border_l_4,
	border_l_8,
	border_transparent,
	border_primary_400,
	border_secondary_400,
	border_accent_400,
	border_submit_400,
	border_deny_400,
	border_inactive_400,
	translate_x_0,
	translate_x_50pct,
	translate_x_m50pct,
	translate_y_50pct,
	translate_y_m50pct,
}
impl AsRef<str> for HellStyle {
	fn as_ref(&self) -> &str {
		match &self {
			Self::width_0 => "width_0",
			Self::width_0d5 => "width_0d5",
			Self::width_1 => "width_1",
			Self::width_1d5 => "width_1d5",
			Self::width_2 => "width_2",
			Self::width_2d5 => "width_2d5",
			Self::width_3 => "width_3",
			Self::width_3d5 => "width_3d5",
			Self::width_4 => "width_4",
			Self::width_5 => "width_5",
			Self::width_6 => "width_6",
			Self::width_7 => "width_7",
			Self::width_8 => "width_8",
			Self::width_9 => "width_9",
			Self::width_10 => "width_10",
			Self::width_px => "width_px",
			Self::width_auto => "width_auto",
			Self::width_1s2 => "width_1s2",
			Self::width_1s3 => "width_1s3",
			Self::width_2s3 => "width_2s3",
			Self::width_1s4 => "width_1s4",
			Self::width_2s4 => "width_2s4",
			Self::width_3s4 => "width_3s4",
			Self::width_1s5 => "width_1s5",
			Self::width_2s5 => "width_2s5",
			Self::width_3s5 => "width_3s5",
			Self::width_4s5 => "width_4s5",
			Self::width_1s20 => "width_1s20",
			Self::width_2s20 => "width_2s20",
			Self::width_3s20 => "width_3s20",
			Self::width_4s20 => "width_4s20",
			Self::width_5s20 => "width_5s20",
			Self::width_6s20 => "width_6s20",
			Self::width_7s20 => "width_7s20",
			Self::width_8s20 => "width_8s20",
			Self::width_9s20 => "width_9s20",
			Self::width_10s20 => "width_10s20",
			Self::width_11s20 => "width_11s20",
			Self::width_12s20 => "width_12s20",
			Self::width_13s20 => "width_13s20",
			Self::width_14s20 => "width_14s20",
			Self::width_15s20 => "width_15s20",
			Self::width_16s20 => "width_16s20",
			Self::width_17s20 => "width_17s20",
			Self::width_18s20 => "width_18s20",
			Self::width_19s20 => "width_19s20",
			Self::width_full => "width_full",
			Self::width_screen => "width_screen",
			Self::width_min => "width_min",
			Self::width_max => "width_max",
			Self::width_fit => "width_fit",
			Self::min_width_0 => "min_width_0",
			Self::min_width_full => "min_width_full",
			Self::min_width_min => "min_width_min",
			Self::min_width_max => "min_width_max",
			Self::min_width_fit => "min_width_fit",
			Self::max_width_0 => "max_width_0",
			Self::max_width_none => "max_width_none",
			Self::max_width_xs => "max_width_xs",
			Self::max_width_sm => "max_width_sm",
			Self::max_width_md => "max_width_md",
			Self::max_width_lg => "max_width_lg",
			Self::max_width_xl => "max_width_xl",
			Self::max_width_2xl => "max_width_2xl",
			Self::max_width_3xl => "max_width_3xl",
			Self::max_width_4xl => "max_width_4xl",
			Self::max_width_5xl => "max_width_5xl",
			Self::max_width_6xl => "max_width_6xl",
			Self::max_width_7xl => "max_width_7xl",
			Self::max_width_full => "max_width_full",
			Self::max_width_max => "max_width_max",
			Self::max_width_min => "max_width_min",
			Self::max_width_fit => "max_width_fit",
			Self::height_0 => "height_0",
			Self::height_0d5 => "height_0d5",
			Self::height_1 => "height_1",
			Self::height_1d5 => "height_1d5",
			Self::height_2 => "height_2",
			Self::height_2d5 => "height_2d5",
			Self::height_3 => "height_3",
			Self::height_3d5 => "height_3d5",
			Self::height_4 => "height_4",
			Self::height_5 => "height_5",
			Self::height_6 => "height_6",
			Self::height_8 => "height_8",
			Self::height_10 => "height_10",
			Self::height_12 => "height_12",
			Self::height_14 => "height_14",
			Self::height_16 => "height_16",
			Self::height_20 => "height_20",
			Self::height_24 => "height_24",
			Self::height_28 => "height_28",
			Self::height_32 => "height_32",
			Self::height_36 => "height_36",
			Self::height_40 => "height_40",
			Self::height_px => "height_px",
			Self::height_auto => "height_auto",
			Self::height_1s2 => "height_1s2",
			Self::height_1s3 => "height_1s3",
			Self::height_2s3 => "height_2s3",
			Self::height_1s4 => "height_1s4",
			Self::height_2s4 => "height_2s4",
			Self::height_3s4 => "height_3s4",
			Self::height_1s5 => "height_1s5",
			Self::height_2s5 => "height_2s5",
			Self::height_3s5 => "height_3s5",
			Self::height_4s5 => "height_4s5",
			Self::height_1s20 => "height_1s20",
			Self::height_2s20 => "height_2s20",
			Self::height_3s20 => "height_3s20",
			Self::height_4s20 => "height_4s20",
			Self::height_5s20 => "height_5s20",
			Self::height_6s20 => "height_6s20",
			Self::height_7s20 => "height_7s20",
			Self::height_8s20 => "height_8s20",
			Self::height_9s20 => "height_9s20",
			Self::height_10s20 => "height_10s20",
			Self::height_11s20 => "height_11s20",
			Self::height_12s20 => "height_12s20",
			Self::height_13s20 => "height_13s20",
			Self::height_14s20 => "height_14s20",
			Self::height_15s20 => "height_15s20",
			Self::height_16s20 => "height_16s20",
			Self::height_17s20 => "height_17s20",
			Self::height_18s20 => "height_18s20",
			Self::height_19s20 => "height_19s20",
			Self::height_full => "height_full",
			Self::height_screen => "height_screen",
			Self::height_min => "height_min",
			Self::height_max => "height_max",
			Self::height_fit => "height_fit",
			Self::min_height_0 => "min_height_0",
			Self::min_height_full => "min_height_full",
			Self::min_height_screen => "min_height_screen",
			Self::min_height_min => "min_height_min",
			Self::min_height_max => "min_height_max",
			Self::min_height_fit => "min_height_fit",
			Self::max_height_0 => "max_height_0",
			Self::max_height_0d5 => "max_height_0d5",
			Self::max_height_1 => "max_height_1",
			Self::max_height_1d5 => "max_height_1d5",
			Self::max_height_2 => "max_height_2",
			Self::max_height_2d5 => "max_height_2d5",
			Self::max_height_3 => "max_height_3",
			Self::max_height_3d5 => "max_height_3d5",
			Self::max_height_4 => "max_height_4",
			Self::max_height_5 => "max_height_5",
			Self::max_height_6 => "max_height_6",
			Self::max_height_8 => "max_height_8",
			Self::max_height_10 => "max_height_10",
			Self::max_height_px => "max_height_px",
			Self::max_height_auto => "max_height_auto",
			Self::max_height_full => "max_height_full",
			Self::max_height_screen => "max_height_screen",
			Self::max_height_min => "max_height_min",
			Self::max_height_max => "max_height_max",
			Self::max_height_fit => "max_height_fit",
			Self::pos_static => "pos_static",
			Self::pos_fixed => "pos_fixed",
			Self::pos_absolute => "pos_absolute",
			Self::pos_relative => "pos_relative",
			Self::pos_sticky => "pos_sticky",
			Self::inset_0 => "inset_0",
			Self::inset_y_0 => "inset_y_0",
			Self::inset_x_0 => "inset_x_0",
			Self::top_0 => "top_0",
			Self::top_50pct => "top_50pct",
			Self::right_0 => "right_0",
			Self::right_50pct => "right_50pct",
			Self::bottom_0 => "bottom_0",
			Self::bottom_1 => "bottom_1",
			Self::bottom_2 => "bottom_2",
			Self::bottom_3 => "bottom_3",
			Self::bottom_4 => "bottom_4",
			Self::bottom_50pct => "bottom_50pct",
			Self::left_0 => "left_0",
			Self::left_50pct => "left_50pct",
			Self::pad_0 => "pad_0",
			Self::pad_0d5 => "pad_0d5",
			Self::pad_1 => "pad_1",
			Self::pad_1d5 => "pad_1d5",
			Self::pad_2 => "pad_2",
			Self::pad_2d5 => "pad_2d5",
			Self::pad_3 => "pad_3",
			Self::pad_3d5 => "pad_3d5",
			Self::pad_4 => "pad_4",
			Self::pad_5 => "pad_5",
			Self::pad_6 => "pad_6",
			Self::pad_8 => "pad_8",
			Self::pad_10 => "pad_10",
			Self::pad_x_0 => "pad_x_0",
			Self::pad_x_0d5 => "pad_x_0d5",
			Self::pad_x_1 => "pad_x_1",
			Self::pad_x_1d5 => "pad_x_1d5",
			Self::pad_x_2 => "pad_x_2",
			Self::pad_x_2d5 => "pad_x_2d5",
			Self::pad_x_3 => "pad_x_3",
			Self::pad_x_3d5 => "pad_x_3d5",
			Self::pad_x_4 => "pad_x_4",
			Self::pad_x_5 => "pad_x_5",
			Self::pad_x_6 => "pad_x_6",
			Self::pad_x_8 => "pad_x_8",
			Self::pad_x_10 => "pad_x_10",
			Self::pad_y_0 => "pad_y_0",
			Self::pad_y_0d5 => "pad_y_0d5",
			Self::pad_y_1 => "pad_y_1",
			Self::pad_y_1d5 => "pad_y_1d5",
			Self::pad_y_2 => "pad_y_2",
			Self::pad_y_2d5 => "pad_y_2d5",
			Self::pad_y_3 => "pad_y_3",
			Self::pad_y_3d5 => "pad_y_3d5",
			Self::pad_y_4 => "pad_y_4",
			Self::pad_y_5 => "pad_y_5",
			Self::pad_y_6 => "pad_y_6",
			Self::pad_y_8 => "pad_y_8",
			Self::pad_y_10 => "pad_y_10",
			Self::mgn_auto => "mgn_auto",
			Self::mgn_0 => "mgn_0",
			Self::mgn_0d5 => "mgn_0d5",
			Self::mgn_1 => "mgn_1",
			Self::mgn_1d5 => "mgn_1d5",
			Self::mgn_2 => "mgn_2",
			Self::mgn_2d5 => "mgn_2d5",
			Self::mgn_3 => "mgn_3",
			Self::mgn_3d5 => "mgn_3d5",
			Self::mgn_4 => "mgn_4",
			Self::mgn_5 => "mgn_5",
			Self::mgn_6 => "mgn_6",
			Self::mgn_8 => "mgn_8",
			Self::mgn_10 => "mgn_10",
			Self::mgn_top_0 => "mgn_top_0",
			Self::mgn_top_0d5 => "mgn_top_0d5",
			Self::mgn_top_1 => "mgn_top_1",
			Self::mgn_top_1d5 => "mgn_top_1d5",
			Self::mgn_top_2 => "mgn_top_2",
			Self::mgn_top_2d5 => "mgn_top_2d5",
			Self::mgn_top_3 => "mgn_top_3",
			Self::mgn_top_3d5 => "mgn_top_3d5",
			Self::mgn_top_4 => "mgn_top_4",
			Self::mgn_top_5 => "mgn_top_5",
			Self::mgn_top_6 => "mgn_top_6",
			Self::mgn_top_8 => "mgn_top_8",
			Self::mgn_top_10 => "mgn_top_10",
			Self::mgn_top_12 => "mgn_top_12",
			Self::mgn_top_14 => "mgn_top_14",
			Self::mgn_top_16 => "mgn_top_16",
			Self::mgn_top_18 => "mgn_top_18",
			Self::mgn_top_20 => "mgn_top_20",
			Self::mgn_top_1s20 => "mgn_top_1s20",
			Self::mgn_top_2s20 => "mgn_top_2s20",
			Self::mgn_top_3s20 => "mgn_top_3s20",
			Self::mgn_top_4s20 => "mgn_top_4s20",
			Self::mgn_top_5s20 => "mgn_top_5s20",
			Self::mgn_top_6s20 => "mgn_top_6s20",
			Self::mgn_top_7s20 => "mgn_top_7s20",
			Self::mgn_top_8s20 => "mgn_top_8s20",
			Self::mgn_top_9s20 => "mgn_top_9s20",
			Self::mgn_top_10s20 => "mgn_top_10s20",
			Self::mgn_top_11s20 => "mgn_top_11s20",
			Self::mgn_top_12s20 => "mgn_top_12s20",
			Self::mgn_top_13s20 => "mgn_top_13s20",
			Self::mgn_top_14s20 => "mgn_top_14s20",
			Self::mgn_top_15s20 => "mgn_top_15s20",
			Self::mgn_top_16s20 => "mgn_top_16s20",
			Self::mgn_top_17s20 => "mgn_top_17s20",
			Self::mgn_top_18s20 => "mgn_top_18s20",
			Self::mgn_top_19s20 => "mgn_top_19s20",
			Self::mgn_bottom_0 => "mgn_bottom_0",
			Self::mgn_bottom_0d5 => "mgn_bottom_0d5",
			Self::mgn_bottom_1 => "mgn_bottom_1",
			Self::mgn_bottom_1d5 => "mgn_bottom_1d5",
			Self::mgn_bottom_2 => "mgn_bottom_2",
			Self::mgn_bottom_2d5 => "mgn_bottom_2d5",
			Self::mgn_bottom_3 => "mgn_bottom_3",
			Self::mgn_bottom_3d5 => "mgn_bottom_3d5",
			Self::mgn_bottom_4 => "mgn_bottom_4",
			Self::mgn_bottom_5 => "mgn_bottom_5",
			Self::mgn_bottom_6 => "mgn_bottom_6",
			Self::mgn_bottom_8 => "mgn_bottom_8",
			Self::mgn_bottom_10 => "mgn_bottom_10",
			Self::mgn_bottom_12 => "mgn_bottom_12",
			Self::mgn_bottom_14 => "mgn_bottom_14",
			Self::mgn_bottom_16 => "mgn_bottom_16",
			Self::mgn_bottom_18 => "mgn_bottom_18",
			Self::mgn_bottom_20 => "mgn_bottom_20",
			Self::mgn_bottom_1s20 => "mgn_bottom_1s20",
			Self::mgn_bottom_2s20 => "mgn_bottom_2s20",
			Self::mgn_bottom_3s20 => "mgn_bottom_3s20",
			Self::mgn_bottom_4s20 => "mgn_bottom_4s20",
			Self::mgn_bottom_5s20 => "mgn_bottom_5s20",
			Self::mgn_bottom_6s20 => "mgn_bottom_6s20",
			Self::mgn_bottom_7s20 => "mgn_bottom_7s20",
			Self::mgn_bottom_8s20 => "mgn_bottom_8s20",
			Self::mgn_bottom_9s20 => "mgn_bottom_9s20",
			Self::mgn_bottom_10s20 => "mgn_bottom_10s20",
			Self::mgn_bottom_11s20 => "mgn_bottom_11s20",
			Self::mgn_bottom_12s20 => "mgn_bottom_12s20",
			Self::mgn_bottom_13s20 => "mgn_bottom_13s20",
			Self::mgn_bottom_14s20 => "mgn_bottom_14s20",
			Self::mgn_bottom_15s20 => "mgn_bottom_15s20",
			Self::mgn_bottom_16s20 => "mgn_bottom_16s20",
			Self::mgn_bottom_17s20 => "mgn_bottom_17s20",
			Self::mgn_bottom_18s20 => "mgn_bottom_18s20",
			Self::mgn_bottom_19s20 => "mgn_bottom_19s20",
			Self::overflow_auto => "overflow_auto",
			Self::overflow_x_auto => "overflow_x_auto",
			Self::overflow_y_auto => "overflow_y_auto",
			Self::overflow_hidden => "overflow_hidden",
			Self::overflow_x_hidden => "overflow_x_hidden",
			Self::overflow_y_hidden => "overflow_y_hidden",
			Self::overflow_visible => "overflow_visible",
			Self::overflow_x_visible => "overflow_x_visible",
			Self::overflow_y_visible => "overflow_y_visible",
			Self::overflow_scroll => "overflow_scroll",
			Self::overflow_x_scroll => "overflow_x_scroll",
			Self::overflow_y_scroll => "overflow_y_scroll",
			Self::bg_primary_400 => "bg_primary_400",
			Self::bg_secondary_400 => "bg_secondary_400",
			Self::bg_accent_400 => "bg_accent_400",
			Self::bg_submit_400 => "bg_submit_400",
			Self::bg_deny_400 => "bg_deny_400",
			Self::bg_inactive_400 => "bg_inactive_400",
			Self::txt_primary_400 => "txt_primary_400",
			Self::txt_secondary_400 => "txt_secondary_400",
			Self::txt_accent_400 => "txt_accent_400",
			Self::txt_submit_400 => "txt_submit_400",
			Self::txt_deny_400 => "txt_deny_400",
			Self::txt_inactive_400 => "txt_inactive_400",
			Self::txt_left => "txt_left",
			Self::txt_center => "txt_center",
			Self::txt_right => "txt_right",
			Self::txt_justify => "txt_justify",
			Self::txt_xs => "txt_xs",
			Self::txt_sm => "txt_sm",
			Self::txt_base => "txt_base",
			Self::txt_lg => "txt_lg",
			Self::txt_xl => "txt_xl",
			Self::txt_2xl => "txt_2xl",
			Self::txt_3xl => "txt_3xl",
			Self::txt_4xl => "txt_4xl",
			Self::txt_5xl => "txt_5xl",
			Self::txt_6xl => "txt_6xl",
			Self::txt_7xl => "txt_7xl",
			Self::txt_8xl => "txt_8xl",
			Self::txt_9xl => "txt_9xl",
			Self::flex => "flex",
			Self::flex_row => "flex_row",
			Self::flex_row_reverse => "flex_row_reverse",
			Self::flex_col => "flex_col",
			Self::flex_col_reverse => "flex_col_reverse",
			Self::grid => "grid",
			Self::grid_cols_1 => "grid_cols_1",
			Self::grid_cols_2 => "grid_cols_2",
			Self::grid_cols_3 => "grid_cols_3",
			Self::grid_cols_4 => "grid_cols_4",
			Self::grid_cols_5 => "grid_cols_5",
			Self::grid_cols_6 => "grid_cols_6",
			Self::grid_cols_7 => "grid_cols_7",
			Self::grid_cols_8 => "grid_cols_8",
			Self::grid_cols_9 => "grid_cols_9",
			Self::grid_cols_10 => "grid_cols_10",
			Self::grid_cols_11 => "grid_cols_11",
			Self::grid_cols_12 => "grid_cols_12",
			Self::grid_cols_none => "grid_cols_none",
			Self::grid_rows_1 => "grid_rows_1",
			Self::grid_rows_2 => "grid_rows_2",
			Self::grid_rows_3 => "grid_rows_3",
			Self::grid_rows_4 => "grid_rows_4",
			Self::grid_rows_5 => "grid_rows_5",
			Self::grid_rows_6 => "grid_rows_6",
			Self::grid_rows_none => "grid_rows_none",
			Self::col_auto => "col_auto",
			Self::col_span_1 => "col_span_1",
			Self::col_span_2 => "col_span_2",
			Self::col_span_3 => "col_span_3",
			Self::col_span_4 => "col_span_4",
			Self::col_span_5 => "col_span_5",
			Self::col_span_6 => "col_span_6",
			Self::col_span_7 => "col_span_7",
			Self::col_span_8 => "col_span_8",
			Self::col_span_9 => "col_span_9",
			Self::col_span_10 => "col_span_10",
			Self::col_span_11 => "col_span_11",
			Self::col_span_12 => "col_span_12",
			Self::col_start_1 => "col_start_1",
			Self::col_start_2 => "col_start_2",
			Self::col_start_3 => "col_start_3",
			Self::col_start_4 => "col_start_4",
			Self::col_start_5 => "col_start_5",
			Self::col_start_6 => "col_start_6",
			Self::col_start_7 => "col_start_7",
			Self::col_start_8 => "col_start_8",
			Self::col_start_9 => "col_start_9",
			Self::col_start_10 => "col_start_10",
			Self::col_start_11 => "col_start_11",
			Self::col_start_12 => "col_start_12",
			Self::col_end_1 => "col_end_1",
			Self::col_end_2 => "col_end_2",
			Self::col_end_3 => "col_end_3",
			Self::col_end_4 => "col_end_4",
			Self::col_end_5 => "col_end_5",
			Self::col_end_6 => "col_end_6",
			Self::col_end_7 => "col_end_7",
			Self::col_end_8 => "col_end_8",
			Self::col_end_9 => "col_end_9",
			Self::col_end_10 => "col_end_10",
			Self::col_end_11 => "col_end_11",
			Self::col_end_12 => "col_end_12",
			Self::col_end_auto => "col_end_auto",
			Self::row_auto => "row_auto",
			Self::row_span_1 => "row_span_1",
			Self::row_span_2 => "row_span_2",
			Self::row_span_3 => "row_span_3",
			Self::row_span_4 => "row_span_4",
			Self::row_span_5 => "row_span_5",
			Self::row_span_6 => "row_span_6",
			Self::row_start_1 => "row_start_1",
			Self::row_start_2 => "row_start_2",
			Self::row_start_3 => "row_start_3",
			Self::row_start_4 => "row_start_4",
			Self::row_start_5 => "row_start_5",
			Self::row_start_6 => "row_start_6",
			Self::row_start_auto => "row_start_auto",
			Self::row_end_1 => "row_end_1",
			Self::row_end_2 => "row_end_2",
			Self::row_end_3 => "row_end_3",
			Self::row_end_4 => "row_end_4",
			Self::row_end_5 => "row_end_5",
			Self::row_end_6 => "row_end_6",
			Self::row_end_auto => "row_end_auto",
			Self::grid_flow_row => "grid_flow_row",
			Self::grid_flow_col => "grid_flow_col",
			Self::grid_flow_row_dense => "grid_flow_row_dense",
			Self::grid_flow_col_dense => "grid_flow_col_dense",
			Self::auto_cols_auto => "auto_cols_auto",
			Self::auto_cols_min => "auto_cols_min",
			Self::auto_cols_max => "auto_cols_max",
			Self::auto_cols_fr => "auto_cols_fr",
			Self::flex_1 => "flex_1",
			Self::flex_auto => "flex_auto",
			Self::flex_initial => "flex_initial",
			Self::flex_none => "flex_none",
			Self::gap_0 => "gap_0",
			Self::gap_0d5 => "gap_0d5",
			Self::gap_1 => "gap_1",
			Self::gap_1d5 => "gap_1d5",
			Self::gap_2 => "gap_2",
			Self::gap_2d5 => "gap_2d5",
			Self::gap_3 => "gap_3",
			Self::gap_3d5 => "gap_3d5",
			Self::gap_4 => "gap_4",
			Self::justify_start => "justify_start",
			Self::justify_center => "justify_center",
			Self::justify_end => "justify_end",
			Self::justify_between => "justify_between",
			Self::justify_around => "justify_around",
			Self::justify_evenly => "justify_evenly",
			Self::justify_items_stretch => "justify_items_stretch",
			Self::justify_items_start => "justify_items_start",
			Self::justify_items_center => "justify_items_center",
			Self::justify_items_end => "justify_items_end",
			Self::justify_self_stretch => "justify_self_stretch",
			Self::justify_self_start => "justify_self_start",
			Self::justify_self_center => "justify_self_center",
			Self::justify_self_end => "justify_self_end",
			Self::justify_self_auto => "justify_self_auto",
			Self::content_start => "content_start",
			Self::content_center => "content_center",
			Self::content_end => "content_end",
			Self::content_between => "content_between",
			Self::content_around => "content_around",
			Self::content_evenly => "content_evenly",
			Self::items_stretch => "items_stretch",
			Self::items_start => "items_start",
			Self::items_center => "items_center",
			Self::items_end => "items_end",
			Self::items_baseline => "items_baseline",
			Self::border_solid => "border_solid",
			Self::border_dashed => "border_dashed",
			Self::border_dotted => "border_dotted",
			Self::border_double => "border_double",
			Self::border_hidden => "border_hidden",
			Self::border_none => "border_none",
			Self::rounded_none => "rounded_none",
			Self::rounded_sm => "rounded_sm",
			Self::rounded => "rounded",
			Self::rounded_md => "rounded_md",
			Self::rounded_lg => "rounded_lg",
			Self::rounded_xl => "rounded_xl",
			Self::rounded_2xl => "rounded_2xl",
			Self::rounded_3xl => "rounded_3xl",
			Self::rounded_full => "rounded_full",
			Self::border => "border",
			Self::border_0 => "border_0",
			Self::border_2 => "border_2",
			Self::border_4 => "border_4",
			Self::border_8 => "border_8",
			Self::border_t => "border_t",
			Self::border_t_0 => "border_t_0",
			Self::border_t_2 => "border_t_2",
			Self::border_t_4 => "border_t_4",
			Self::border_t_8 => "border_t_8",
			Self::border_r => "border_r",
			Self::border_r_0 => "border_r_0",
			Self::border_r_2 => "border_r_2",
			Self::border_r_4 => "border_r_4",
			Self::border_r_8 => "border_r_8",
			Self::border_b => "border_b",
			Self::border_b_0 => "border_b_0",
			Self::border_b_2 => "border_b_2",
			Self::border_b_4 => "border_b_4",
			Self::border_b_8 => "border_b_8",
			Self::border_l => "border_l",
			Self::border_l_0 => "border_l_0",
			Self::border_l_2 => "border_l_2",
			Self::border_l_4 => "border_l_4",
			Self::border_l_8 => "border_l_8",
			Self::border_transparent => "border_transparent",
			Self::border_primary_400 => "border_primary_400",
			Self::border_secondary_400 => "border_secondary_400",
			Self::border_accent_400 => "border_accent_400",
			Self::border_submit_400 => "border_submit_400",
			Self::border_deny_400 => "border_deny_400",
			Self::border_inactive_400 => "border_inactive_400",
			Self::translate_x_0 => "translate_x_0",
			Self::translate_x_50pct => "translate_x_50pct",
			Self::translate_x_m50pct => "translate_x_m50pct",
			Self::translate_y_50pct => "translate_y_50pct",
			Self::translate_y_m50pct => "translate_y_m50pct",
		}
	}
}
