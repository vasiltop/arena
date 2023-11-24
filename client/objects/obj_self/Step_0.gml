var _up = keyboard_check(ord("W"));
var _left = keyboard_check(ord("A"));
var _down = keyboard_check(ord("S"));
var _right = keyboard_check(ord("D"));

if _up or _left or _down or _right {
	
	if sprite_index == spr_player_idle {
		sprite_index = spr_player_run;
		image_speed = 0.4;
		send({ type: "sprite", id: uuid, sprite: sprite_index});
		
	}
	
	var _hsp = _right - _left;
	var _vsp = _down - _up;
	
	x += lengthdir_x(spd, point_direction(x, y, x + _hsp, y + _vsp));
	y += lengthdir_y(spd, point_direction(x, y, x + _hsp, y + _vsp));
	
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


if mouse_check_button_pressed(mb_left) {
	send({ type: "shot", id: obj_self.uuid});
	var _b = instance_create_layer(x, y, "Instances", obj_tracer);
	_b.dir = aim_direction;
	_b.image_angle = aim_direction;
	var _col = collision_line(x, y - 4, mouse_x, mouse_y, obj_other, false, true);
	if  _col != noone {
		
		if _col.hp <= 0 {
			send({ type: "death", id: _col.uuid });
		} else {
			send({ type: "dmg", id: _col.uuid, amount: 50 });
		}
	}
}