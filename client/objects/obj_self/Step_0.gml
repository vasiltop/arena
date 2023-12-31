var _up = keyboard_check(ord("W"));
var _left = keyboard_check(ord("A"));
var _down = keyboard_check(ord("S"));
var _right = keyboard_check(ord("D"));

if _up or _left or _down or _right {
	
	//Animation
	if sprite_index == spr_player_idle {
		sprite_index = spr_player_run;
		image_speed = 0.4;
		send({ type: "sprite", id: uuid, sprite: sprite_index});
		
	}
	
	var _hsp = _right - _left;
	var _vsp = _down - _up;
	
	var _xmove = lengthdir_x(spd, point_direction(x, y, x + _hsp, y + _vsp));
	var _ymove = lengthdir_y(spd, point_direction(x, y, x + _hsp, y + _vsp));
	
	//Collisions
	if !place_meeting(x + _xmove, y, obj_col) {
		x += _xmove;	
	}
	
	
	if !place_meeting(x, y + _ymove, obj_col) {
		y += _ymove;	
	}
	
	send({
		type: "pos",
		x: x,
		y: y,
		id: uuid
	});
	
} else {
	if sprite_index == spr_player_run {
		sprite_index = spr_player_idle;
		image_speed = 0.1;
		send({ type: "sprite", id: uuid, sprite: sprite_index});
		
	}
}

var _dir = point_direction(x, y, mouse_x, mouse_y);

if _dir != aim_direction {
	send({ type: "dir", id: uuid, dir: _dir, x_scale: image_xscale});
	aim_direction = _dir;
}

image_xscale = -sign(x - mouse_x);

if mouse_check_button_pressed(mb_left) and alarm[0] <= 0 {
	alarm[0] = 15;
	audio_play_sound(snd_shoot, 1, false, 1);
	send({ type: "shot", id: obj_self.uuid});
	
	var _b = instance_create_layer(x, y, "Instances", obj_tracer);
	_b.dir = aim_direction;
	_b.image_angle = aim_direction;
	
	var _l = ds_list_create();
	collision_line_list(x, y - 4, mouse_x + lengthdir_x(300, aim_direction), mouse_y +  + lengthdir_y(300, aim_direction), obj_col, false, true, _l, true);
	
	var _wall = ds_list_find_value(_l, 0);
	
	
	var _ps = ds_list_create();
	collision_line_list(x, y - 4, mouse_x + lengthdir_x(300, aim_direction), mouse_y +  + lengthdir_y(300, aim_direction), obj_other, false, true, _ps, true);
	
	var _player = ds_list_find_value(_ps, 0);
	
	if  !is_undefined(_player) and (is_undefined(_wall) or point_distance(x, y, _wall.x, _wall.y) > point_distance(x, y, _player.x, _player.y) )  {
		
		send({ type: "dmg", id: _player.uuid, amount: 75 });
	}
}