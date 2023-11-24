draw_text(window_get_width() / 3.5, 50, "You are currently dead, wait for the round to end.");


if alarm[0] > 0 {
	draw_text(window_get_width() / 3.5, 70, string(round(alarm[0] / 60)) + " seconds until respawn.");
}