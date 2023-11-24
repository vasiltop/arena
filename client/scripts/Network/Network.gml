function process(_data) {
	
	//show_debug_message("Received: ");
	//show_debug_message(_data);

	switch _data[0] {
		
		case "init":
			
			var _p = instance_create_layer(50, 50, "Instances", obj_self);
			
			_p.uuid = _data[1];	
			send({ type: "pos", x: x, y: y, id: _data[1]});
			ds_map_add(players, _data[1], _p);
			

		break;
		
		
		case "pos":
			
			var _p = ds_map_find_value(players, _data[1]);
			
			if !is_undefined(_p) and instance_exists(_p) {
				_p.x = _data[2];
				_p.y = _data[3];
			} else if !instance_exists(obj_respawn_menu) {
				var _player = instance_create_layer(_data[2], _data[3], "Instances", obj_other);
				
				ds_map_add(players, _data[1], _player);
				_player.uuid = _data[1];
				send({ type: "pos", x: obj_self.x, y: obj_self.y, id: obj_self.uuid});
			}

		break;
		
		case "disconnect": 
			
			var _p = ds_map_find_value(players, _data[1]);
			if !is_undefined(_p) and instance_exists(_p) {
				instance_destroy(_p);
			}
		
		break;
		
		case "dir":
			
			var _p = ds_map_find_value(players, _data[1]);
			if !is_undefined(_p) and instance_exists(_p) {
				_p.aim_direction = _data[2];
				_p.image_xscale = _data[3];
			}
			
		
		break;
		
		case "sprite":
			var _p = ds_map_find_value(players, _data[1]);
			if !is_undefined(_p) and instance_exists(_p) {
				_p.sprite_index = _data[2];
			}
		
		break;
		
		case "shot":
			var _p = ds_map_find_value(players, _data[1]);
			if !is_undefined(_p) and instance_exists(_p) {
				var _b = instance_create_layer(_p.x, _p.y, "Instances", obj_tracer);
				_b.dir = _p.aim_direction;
				_b.image_angle = _p.aim_direction;
				
				var _dist = point_distance(obj_self.x, obj_self.y, _p.x, _p.y);
				
				audio_play_sound(snd_shoot, 1, false, 1 - (_dist / 500));
			}
			
		break;
		
		case "death":
			var _p = ds_map_find_value(players, _data[1]);
			if !is_undefined(_p) and instance_exists(_p) {
				
				ds_map_delete(players, _p.uuid);
				
				if _p.uuid == obj_self.uuid {
					var _m = instance_create_layer(obj_self.x, obj_self.y, "Instances", obj_respawn_menu);
					_m.uuid = _p.uuid;
					obj_camera.follow = _m;
				} 
				
				instance_destroy(_p);
			}
		break;
		
		case "dmg":
		
			
			var _p = ds_map_find_value(players, _data[1]);
			if !is_undefined(_p) and instance_exists(_p) {
				_p.hp -= _data[2];
			}
		break;
		
	}
}



function send(_data) {
	
	var _encoded = buffer_create(1, buffer_grow, 1);
	SnapBufferWriteMessagePack(_encoded, _data);
	
	var _size_encoded = buffer_tell(_encoded);
	var _buff_with_size = buffer_create(_size_encoded + 1, buffer_fixed, 1);
	buffer_write(_buff_with_size, buffer_u64, _size_encoded);
	buffer_copy(_encoded, 0, _size_encoded, _buff_with_size, 1);
	//show_debug_message("Sent: ");
	//show_debug_message(buffer_prettyprint(_buff_with_size));
	network_send_raw(obj_network.socket, _buff_with_size, _size_encoded + 1);
	
}