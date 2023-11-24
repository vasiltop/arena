if keyboard_check_pressed(vk_enter) {
	
	var _p = instance_create_layer(50, 50, "Instances", obj_self);
	obj_camera.follow = obj_self;
	_p.uuid = uuid;	
	send({ type: "pos", x: _p.x, y: _p.y, id: uuid});
	ds_map_add(obj_network.players, uuid, _p);
	instance_destroy(self);	
}