
//Follow a certain x and y position

x += (xTo-x)/spd;
y += (yTo-y)/spd;

if(follow != noone and instance_exists(follow))
{
	xTo = follow.x;
	yTo = follow.y;
}

vm = matrix_build_lookat(x, y, -10, x, y, 0, 0, 1, 0);

camera_set_view_mat(camera, vm);


//Shake the camera if alarm[0] is set
if(alarm[0] > 0) {
	x+=random_range(-range, range);
	y+=random_range(-range, range);
} 