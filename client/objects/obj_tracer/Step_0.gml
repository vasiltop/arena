x += lengthdir_x(spd, dir);
y += lengthdir_y(spd, dir);

if place_meeting(x, y, obj_col) {
	instance_destroy(self);	
}