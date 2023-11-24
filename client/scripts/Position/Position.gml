function new_spawn_pos() {
	
	
	var _x = RAND_X;
	var _y = RAND_Y;
	
	while position_meeting(_x, _y, obj_col) {
		_x = RAND_X;
		_y = RAND_Y;
	}
	
	return { x: _x, y: _y };

}