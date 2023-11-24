if keyboard_check_pressed(ord("Q")) {
	
	var _p = ds_map_find_previous(obj_network.players, spectating);
	
	if !is_undefined(_p) {
		spectating =  _p;
	}
}

if keyboard_check_pressed(ord("E")) {
	
	var _n = ds_map_find_next(obj_network.players, spectating);
	
	if !is_undefined(_n) {
		spectating = _n;
	}
}

var _s = ds_map_find_value(obj_network.players, spectating);

if instance_exists(_s) {
	obj_camera.follow = _s;	
}


if ds_map_size(obj_network.players) <= 1 and !timer_started {
	timer_started = true;
	alarm[0] = 300;
}


if alarm[0] <= 0 and timer_started {

	var _pos = new_spawn_pos();
	var _p = instance_create_layer(_pos.x, _pos.y, "Instances", obj_self);
	obj_camera.follow = obj_self;
	_p.uuid = uuid;
	send({ type: "pos", x: _p.x, y: _p.y, id: uuid});
	ds_map_add(obj_network.players, uuid, _p);
	instance_destroy(self);
	

}

